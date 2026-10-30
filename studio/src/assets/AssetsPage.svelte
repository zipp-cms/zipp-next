<script module lang="ts">
	import type { LoadProps } from '../core/router.svelte';
	import { listAssets as list, listFolders } from './api';

	/** `?folder=12` for a folder, none for the top. */
	function folderOf(param: string | null): number | null {
		const id = Number(param);
		return param && Number.isInteger(id) ? id : null;
	}

	/** The folders and the newest assets of the one in the url. */
	export async function loadProps(_: unknown, lp: LoadProps) {
		const folder = folderOf(lp.req.search.get('folder'));
		const [{ folders, top }, first] = await Promise.all([
			listFolders(),
			list({ search: '', folder: folder === null ? 'top' : `${folder}` }),
		]);
		// a deleted folder
		const known = folder === null || folders.some(f => f.id === folder);
		return { folders, top, first, folder: known ? folder : null };
	}
</script>

<script lang="ts">
	import { untrack } from 'svelte';
	import PageTop from 'zipp-ui/PageTop.svelte';
	import {
		assetUrl,
		assetUsage,
		deleteAsset,
		ROOT,
		folderRows,
		listAssets,
		moveAssets,
		updateAsset,
		type Asset,
		type Folder,
	} from './api';
	import FolderTree from './FolderTree.svelte';
	import { type Schema } from '../core/schema';
	import { canAssets } from '../core/access';
	import AssetGrid from './AssetGrid.svelte';
	import AssetDetail from './AssetDetail.svelte';
	import SearchInput, { SEARCH_DELAY_MS } from 'zipp-ui/SearchInput.svelte';
	import { Busy } from '../core/busy.svelte';
	import DropOverlay from './DropOverlay.svelte';
	import UploadButton from 'zipp-ui/UploadButton.svelte';
	import type { MenuItem } from 'zipp-ui/Menu.svelte';
	import { plural } from '../core/format';
	import {
		FileDrag,
		fromInput,
		uploads,
		type Upload,
	} from './uploads.svelte';

	let {
		schema,
		first,
		folders: firstFolders,
		folder: firstFolder,
		top: firstTop,
	}: {
		schema: Schema;
		first: Asset[];
		folders: Folder[];
		folder: number | null;
		top: number;
	} = $props();
	const canEdit = $derived(canAssets(schema, 'edit'));
	const canDelete = $derived(canAssets(schema, 'delete'));

	let assets = $state<Asset[]>(untrack(() => first));
	let folders = $state<Folder[]>(untrack(() => firstFolders));
	let top = $state(untrack(() => firstTop));
	/** null: the top */
	let folder = $state<number | null>(untrack(() => firstFolder));
	/** `Library / Photos / Events` */
	const path = $derived(
		[ROOT, folderRows(folders).find(r => r.folder.id === folder)?.path]
			.filter(Boolean)
			.join(' / '),
	);
	let search = $state('');
	let error = $state('');
	const action = new Busy();
	let current = $state<Asset | null>(null);

	/** The assets of the folder, a search looks in all of them. */
	function load() {
		const q = search;
		const f = folder;
		const params: Record<string, string> = { search: q };
		if (!q) params.folder = f === null ? 'top' : `${f}`;
		listAssets(params)
			.then(a => {
				if (q === search && f === folder) assets = a;
			})
			.catch(e => (error = e.message));
	}

	/** The folders with their counts. */
	function loadFolders() {
		listFolders()
			.then(f => {
				folders = f.folders;
				top = f.top;
			})
			.catch(e => (error = e.message));
	}

	/** Another folder, in the url too, without loading the page again. */
	function select(id: number | null) {
		if (id === folder && !search) return;
		folder = id;
		search = '';
		searched = '';
		const url = id === null ? location.pathname : `?folder=${id}`;
		history.replaceState(history.state, '', url);
		load();
	}

	async function move(ids: number[], to: number | null) {
		await action.run(() => moveAssets(ids, to));
		load();
		loadFolders();
	}

	// the first assets came with the page
	let searched = '';
	// a moment after the last key, not with every one
	$effect(() => {
		if (search === searched) return;
		const wait = setTimeout(() => {
			searched = search;
			load();
		}, SEARCH_DELAY_MS);
		return () => clearTimeout(wait);
	});

	/** Files, or folders with theirs, into a folder of the tree. */
	async function upload(items: Upload[], into = folder) {
		if (!items.length) return;
		const made = items.some(i => i.dir.length);
		// the new folders show while the files go up
		const run = uploads.run(items, into);
		if (made) setTimeout(loadFolders, 500);
		await run;
		load();
		loadFolders();
	}

	// files dropped anywhere on the page but on a folder go into the open one
	const drag = new FileDrag(
		items => upload(items),
		() => canEdit,
	);

	function open(a: Asset) {
		current = a;
	}

	async function rename(a: Asset, filename: string): Promise<boolean> {
		const saved = await action.run(() =>
			updateAsset(a.id, {
				title: a.title,
				alt: a.alt,
				focalX: a.focalX,
				focalY: a.focalY,
				filename,
				edits: a.edits,
			}),
		);
		if (saved) assets = assets.map(x => (x.id === a.id ? saved : x));
		return !!saved;
	}

	async function remove(a: Asset) {
		const n = (await assetUsage(a.id).catch(() => [])).length;
		const used = n
			? ` It is used in ${plural(n, 'entry', 'entries')}.`
			: '';
		if (!confirm(`Delete ${a.filename}?${used}`)) return;
		await action.run(() => deleteAsset(a.id));
		load();
		loadFolders();
	}

	function menu(a: Asset, startRename: () => void): MenuItem[] {
		const items: MenuItem[] = [
			{ label: 'Open', icon: 'view', onclick: () => open(a) },
		];
		if (canEdit)
			items.push({ label: 'Rename', icon: 'edit', onclick: startRename });
		items.push(
			{
				label: 'Download',
				icon: 'download',
				href: assetUrl(a, false, true),
				download: a.filename,
			},
			{
				label: 'Copy URL',
				icon: 'copy-url',
				onclick: () =>
					navigator.clipboard.writeText(
						location.origin + assetUrl(a),
					),
			},
		);
		if (canEdit && a.folderId !== null)
			items.push({
				label: `Move to ${ROOT}`,
				icon: 'file',
				onclick: () => move([a.id], null),
			});
		if (canDelete)
			items.push({
				label: 'Delete',
				icon: 'delete',
				danger: true,
				onclick: () => remove(a),
			});
		return items;
	}
</script>

<div
	class="page"
	role="region"
	aria-label="Assets"
	ondragenter={drag.dragenter}
	ondragleave={drag.dragleave}
	ondragover={drag.dragover}
	ondrop={drag.drop}
>
	<PageTop title="Assets">
		{#snippet actions()}
			{#if canEdit}
				<UploadButton
					onfiles={f => upload(fromInput(f))}
					directory
					icon="folder-plus"
				>
					Upload folder
				</UploadButton>
				<UploadButton
					onfiles={f => upload(fromInput(f))}
					multiple
					primary
				>
					{uploads.active ? 'Uploading…' : 'Upload'}
				</UploadButton>
			{/if}
		{/snippet}
	</PageTop>

	{#if error || action.error}
		<p class="error">{error || action.error}</p>
	{/if}

	<div class="library">
		<aside class="folders">
			<FolderTree
				{folders}
				{top}
				current={search ? -1 : folder}
				onselect={select}
				editable={canEdit}
				deletable={canDelete}
				onchange={loadFolders}
				onmove={move}
				onupload={upload}
			/>
		</aside>

		<div class="card files">
			{#if drag.over}
				<DropOverlay label="Drop to upload into {path}" />
			{/if}
			<div class="top">
				<h2>{search ? 'In all folders' : path}</h2>
				<SearchInput bind:value={search} placeholder="Search files" />
			</div>
			<AssetGrid
				{assets}
				onpick={open}
				draggable={canEdit}
				{menu}
				onrename={rename}
				empty={search
					? 'Nothing found'
					: folder === null
						? 'No assets yet'
						: 'No files in this folder'}
			/>
		</div>
	</div>
</div>

{#if current}
	{#key current.id}
		<AssetDetail
			{schema}
			{folders}
			asset={current}
			onsaved={a => {
				const moved = a.folderId !== current?.folderId;
				assets = assets.map(x => (x.id === a.id ? a : x));
				current = null;
				if (moved) {
					load();
					loadFolders();
				}
			}}
			ondeleted={() => {
				current = null;
				load();
				loadFolders();
			}}
			onclose={() => {
				// a replaced file has a new thumbnail
				current = null;
				load();
			}}
		/>
	{/key}
{/if}

<style lang="scss">
	.page {
		min-height: 80vh;
	}

	.library {
		display: grid;
		grid-template-columns: 15rem minmax(0, 1fr);
		gap: 1.5rem;
		align-items: start;

		@include max-tablet {
			grid-template-columns: 1fr;
		}
	}

	.files {
		position: relative;
		min-height: 20rem;
	}

	.folders {
		position: sticky;
		top: 1rem;
	}

	.top {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1rem;

		h2 {
			overflow: hidden;
			margin: 0;
			font-size: var(--fs-md);
			text-overflow: ellipsis;
			white-space: nowrap;
		}

		:global(.search) {
			flex-shrink: 0;
			width: 21.25rem;
			max-width: 50%;
		}
	}
</style>
