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
		const asked = folderOf(lp.req.search.get('folder'));
		const [{ folders, top }, first] = await Promise.all([
			listFolders(),
			list({ folder: asked }),
		]);
		// a deleted folder
		const known = asked === null || folders.some(f => f.id === asked);
		return { folders, top, first, folder: known ? asked : null };
	}
</script>

<script lang="ts">
	import { untrack } from 'svelte';
	import PageTop from 'zipp-ui/PageTop.svelte';
	import {
		ROOT,
		assetUsage,
		deleteAsset,
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
	import SearchInput from 'zipp-ui/SearchInput.svelte';
	import { Busy } from '../core/busy.svelte';
	import DropOverlay from './DropOverlay.svelte';
	import UploadButton from 'zipp-ui/UploadButton.svelte';
	import type { MenuItem } from 'zipp-ui/Menu.svelte';
	import { Library } from './library.svelte';
	import { confirmDelete, linkItems } from './actions';
	import {
		FileDrag,
		fromInput,
		uploads,
		type Upload,
	} from './uploads.svelte';

	let {
		schema,
		first,
		folders,
		folder,
		top,
	}: {
		schema: Schema;
		first: Asset[];
		folders: Folder[];
		folder: number | null;
		top: number;
	} = $props();
	const canEdit = $derived(canAssets(schema, 'edit'));
	const canDelete = $derived(canAssets(schema, 'delete'));

	// what came with the page
	const lib = new Library();
	untrack(() => {
		Object.assign(lib, { folders, top, folder, assets: first });
		lib.loaded = true;
	});
	const action = new Busy();
	let current = $state<Asset | null>(null);

	/** All again: what the list shows and the counts of the folders. */
	function reload() {
		lib.load();
		lib.loadFolders();
	}

	/** Another folder, in the url too, without loading the page again. */
	function select(id: number | null) {
		if (id === lib.folder && !lib.search) return;
		lib.select(id);
		const url = id === null ? location.pathname : `?folder=${id}`;
		history.replaceState(history.state, '', url);
	}

	async function move(ids: number[], to: number | null) {
		await action.run(() => moveAssets(ids, to));
		reload();
	}

	/** Files, or folders with theirs, into a folder of the tree. */
	async function upload(items: Upload[], into = lib.folder) {
		// the new folders show while the files go up
		await uploads.run(items, into, () => lib.loadFolders());
		reload();
	}

	// files dropped anywhere on the page but on a folder go into the open one
	const drag = new FileDrag(
		items => upload(items),
		() => canEdit,
	);

	function open(a: Asset) {
		current = a;
	}

	/** Only the name, the rest as it is. */
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
		if (saved)
			lib.assets = lib.assets.map(x => (x.id === a.id ? saved : x));
		return !!saved;
	}

	async function remove(a: Asset) {
		const n = (await assetUsage(a.id).catch(() => [])).length;
		if (!confirmDelete(a, n)) return;
		await action.run(() => deleteAsset(a.id));
		reload();
	}

	function menu(a: Asset, startRename: () => void): MenuItem[] {
		const items: MenuItem[] = [
			{ label: 'Open', icon: 'view', onclick: () => open(a) },
		];
		if (canEdit)
			items.push({ label: 'Rename', icon: 'edit', onclick: startRename });
		items.push(...linkItems(a));
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

	{#if lib.error || action.error}
		<p class="error">{lib.error || action.error}</p>
	{/if}

	<div class="library">
		<aside class="folders">
			<FolderTree
				folders={lib.folders}
				top={lib.top}
				current={lib.folder}
				searching={!!lib.search}
				onselect={select}
				editable={canEdit}
				deletable={canDelete}
				onchange={() => lib.loadFolders()}
				onmove={move}
				onupload={upload}
			/>
		</aside>

		<div class="card files">
			{#if drag.over}
				<DropOverlay label="Drop to upload into {lib.path}" />
			{/if}
			<div class="top">
				<h2>{lib.search ? 'In all folders' : lib.path}</h2>
				<SearchInput
					bind:value={() => lib.search, v => lib.type(v)}
					placeholder="Search files"
				/>
			</div>
			<AssetGrid
				assets={lib.assets}
				onpick={open}
				draggable={canEdit}
				{menu}
				onrename={rename}
				empty={lib.empty}
			/>
		</div>
	</div>
</div>

{#if current}
	{#key current.id}
		<AssetDetail
			{schema}
			folders={lib.folders}
			asset={current}
			onsaved={a => {
				const moved = a.folderId !== current?.folderId;
				lib.assets = lib.assets.map(x => (x.id === a.id ? a : x));
				current = null;
				if (moved) reload();
			}}
			ondeleted={() => {
				current = null;
				reload();
			}}
			onclose={() => {
				// a replaced file has a new thumbnail
				current = null;
				lib.load();
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
