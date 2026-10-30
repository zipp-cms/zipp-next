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
		const [folders, first] = await Promise.all([
			listFolders(),
			list({ search: '', folder: folder === null ? 'top' : `${folder}` }),
		]);
		// a deleted folder
		const known = folder === null || folders.some(f => f.id === folder);
		return { folders, first, folder: known ? folder : null };
	}
</script>

<script lang="ts">
	import { untrack } from 'svelte';
	import PageTop from 'zipp-ui/PageTop.svelte';
	import {
		folderRows,
		listAssets,
		moveAssets,
		uploadAssets,
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
	import UploadButton from 'zipp-ui/UploadButton.svelte';

	let {
		schema,
		first,
		folders: firstFolders,
		folder: firstFolder,
	}: {
		schema: Schema;
		first: Asset[];
		folders: Folder[];
		folder: number | null;
	} = $props();
	const canEdit = $derived(canAssets(schema, 'edit'));
	const canDelete = $derived(canAssets(schema, 'delete'));

	let assets = $state<Asset[]>(untrack(() => first));
	let folders = $state<Folder[]>(untrack(() => firstFolders));
	/** null: the top */
	let folder = $state<number | null>(untrack(() => firstFolder));
	const path = $derived(
		folderRows(folders).find(r => r.folder.id === folder)?.path ?? '',
	);
	let search = $state('');
	let error = $state('');
	const uploading = new Busy();
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
			.then(f => (folders = f))
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
		await uploading.run(() => moveAssets(ids, to));
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

	async function upload(files: FileList | null, into = folder) {
		if (!files?.length) return;
		await uploading.run(() => uploadAssets(files, into));
		load();
		loadFolders();
	}

	function open(a: Asset) {
		current = a;
	}

	function ondrop(e: DragEvent) {
		e.preventDefault();
		if (canEdit) upload(e.dataTransfer?.files ?? null);
	}
</script>

<div
	class="page"
	role="region"
	aria-label="Assets"
	ondragover={e => e.preventDefault()}
	{ondrop}
>
	<PageTop title="Assets">
		{#snippet actions()}
			{#if canEdit}
				<UploadButton onfiles={upload} multiple primary>
					{uploading.busy ? 'Uploading…' : 'Upload'}
				</UploadButton>
			{/if}
		{/snippet}
	</PageTop>

	{#if error || uploading.error}
		<p class="error">{error || uploading.error}</p>
	{/if}

	<div class="library">
		<aside class="folders">
			<FolderTree
				{folders}
				current={search ? -1 : folder}
				onselect={select}
				editable={canEdit}
				deletable={canDelete}
				onchange={loadFolders}
				onmove={move}
				onupload={upload}
			/>
		</aside>

		<div class="card">
			<div class="top">
				<h2>{search ? 'In all folders' : path || 'Files'}</h2>
				<SearchInput bind:value={search} placeholder="Search files" />
			</div>
			<AssetGrid
				{assets}
				onpick={open}
				draggable={canEdit}
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
