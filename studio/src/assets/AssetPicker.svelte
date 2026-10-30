<script lang="ts" module>
	/** The folder the picker showed last, it opens there again. */
	let lastFolder: number | null = null;
</script>

<script lang="ts">
	import {
		folderByPath,
		listAssets,
		listFolders,
		type Asset,
		type Folder,
	} from './api';
	import FolderTree from './FolderTree.svelte';
	import AssetGrid from './AssetGrid.svelte';
	import { Busy } from '../core/busy.svelte';
	import Flyout from 'zipp-ui/Flyout.svelte';
	import SearchInput from 'zipp-ui/SearchInput.svelte';
	import UploadButton from 'zipp-ui/UploadButton.svelte';
	import DropOverlay from './DropOverlay.svelte';
	import {
		FileDrag,
		acceptOf,
		allows,
		pathDirs,
		fromInput,
		uploads,
		type Upload,
	} from './uploads.svelte';

	/** The asset library in a flyout, to pick assets or upload new ones. */
	let {
		images,
		extensions,
		folderPath,
		multiple,
		selected,
		onpick,
		onclose,
	}: {
		/** only images */
		images: boolean;
		/** only files that end so */
		extensions?: string[];
		/** the field's folder: the picker opens there, uploads go there */
		folderPath?: string;
		multiple: boolean;
		selected: number[];
		onpick: (a: Asset) => void;
		onclose: () => void;
	} = $props();

	let library = $state<Asset[]>([]);
	let search = $state('');
	let loaded = $state(false);
	let folders = $state<Folder[]>([]);
	let top = $state(0);
	/** null: the top; a search looks in all folders */
	let folder = $state<number | null>(null);
	let foldersLoaded = $state(false);

	listFolders()
		.then(({ folders: f, top: t }) => {
			folders = f;
			top = t;
			// the field's folder first, else where the picker was
			const own = folderByPath(f, folderPath);
			if (own) folder = own.id;
			else if (f.some(f => f.id === lastFolder)) folder = lastFolder;
			foldersLoaded = true;
		})
		.catch(e => (action.error = e.message));

	$effect(() => {
		if (!foldersLoaded) return;
		const q = search;
		const f = folder;
		const params: Record<string, string> = { search: q };
		if (!q) params.folder = f === null ? 'top' : `${f}`;
		if (images) params.kind = 'image';
		if (extensions?.length) params.extensions = extensions.join(',');
		listAssets(params)
			.then(a => {
				if (q !== search || f !== folder) return;
				library = a;
				loaded = true;
			})
			.catch(e => (action.error = e.message));
	});

	function select(id: number | null) {
		folder = lastFolder = id;
		search = '';
	}

	const action = new Busy();

	/** Into the open folder, and picked. */
	async function upload(items: Upload[]) {
		items = items.filter(i => allows({ images, extensions }, i.file));
		if (!multiple) items = items.slice(0, 1);
		// a dropped folder's files go into the open one; on the top with
		// a folder of the field, into that (made when missing)
		const dir = folder === null ? pathDirs(folderPath) : [];
		items = items.map(i => ({ ...i, dir }));
		const added = await uploads.run(items, folder);
		if (dir.length) listFolders().then(r => (folders = r.folders));
		library = [...added, ...library];
		for (const a of added) onpick(a);
	}

	const drag = new FileDrag(upload, () => true);

	const what = $derived(
		images
			? multiple
				? 'images'
				: 'an image'
			: multiple
				? 'files'
				: 'a file',
	);
</script>

<Flyout title="Choose {what}" {onclose}>
	{#snippet actions()}
		<UploadButton
			onfiles={f => upload(fromInput(f))}
			{multiple}
			accept={acceptOf({ images, extensions })}
		>
			{uploads.active ? 'Uploading…' : 'Upload'}
		</UploadButton>
		<button type="button" class="primary" onclick={onclose}>Done</button>
	{/snippet}
	<div class="search">
		<SearchInput bind:value={search} placeholder="Search files" />
	</div>
	{#if action.error}<p class="error">{action.error}</p>{/if}
	<div
		class="library"
		class:flat={!folders.length}
		role="region"
		aria-label="Files"
		ondragenter={drag.dragenter}
		ondragleave={drag.dragleave}
		ondragover={drag.dragover}
		ondrop={drag.drop}
	>
		{#if folders.length}
			<FolderTree
				{folders}
				{top}
				current={search ? -1 : folder}
				onselect={select}
			/>
		{/if}
		<div class="card files">
			{#if drag.over}
				<DropOverlay label="Drop to upload" />
			{/if}
			<AssetGrid
				assets={library}
				{selected}
				loading={!loaded}
				empty={search
					? 'Nothing found'
					: folder === null
						? 'No assets yet'
						: 'No files in this folder'}
				{onpick}
			/>
		</div>
	</div>
</Flyout>

<style lang="scss">
	.search {
		margin: 1rem 0 0.75rem;
	}

	.files {
		position: relative;
		min-height: 15rem;
	}

	.library {
		display: grid;
		grid-template-columns: 12.5rem minmax(0, 1fr);
		gap: 1rem;
		align-items: start;

		&.flat {
			grid-template-columns: 1fr;
		}

		@include max-tablet {
			grid-template-columns: 1fr;
		}
	}
</style>
