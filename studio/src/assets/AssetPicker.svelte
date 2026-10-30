<script lang="ts" module>
	/** The folder the picker showed last, it opens there again. */
	let lastFolder: number | null = null;
</script>

<script lang="ts">
	import {
		listAssets,
		listFolders,
		uploadAssets,
		type Asset,
		type Folder,
	} from './api';
	import FolderTree from './FolderTree.svelte';
	import AssetGrid from './AssetGrid.svelte';
	import { Busy } from '../core/busy.svelte';
	import Flyout from 'zipp-ui/Flyout.svelte';
	import SearchInput from 'zipp-ui/SearchInput.svelte';
	import UploadButton from 'zipp-ui/UploadButton.svelte';

	/** The asset library in a flyout, to pick assets or upload new ones. */
	let {
		images,
		multiple,
		selected,
		onpick,
		onclose,
	}: {
		/** only images */
		images: boolean;
		multiple: boolean;
		selected: number[];
		onpick: (a: Asset) => void;
		onclose: () => void;
	} = $props();

	let library = $state<Asset[]>([]);
	let search = $state('');
	let loaded = $state(false);
	let folders = $state<Folder[]>([]);
	/** null: the top; a search looks in all folders */
	let folder = $state<number | null>(null);
	let foldersLoaded = $state(false);

	listFolders()
		.then(f => {
			folders = f;
			if (f.some(f => f.id === lastFolder)) folder = lastFolder;
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

	async function upload(files: FileList) {
		const added = await action.run(() => uploadAssets(files, folder));
		if (!added) return;
		library = [...added, ...library];
		for (const a of added) onpick(a);
	}

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

<Flyout title="Choose {what}" {onclose} narrow>
	{#snippet actions()}
		<UploadButton
			onfiles={upload}
			{multiple}
			accept={images ? 'image/*' : undefined}
		>
			{action.busy ? 'Uploading…' : 'Upload'}
		</UploadButton>
		<button type="button" class="primary" onclick={onclose}>Done</button>
	{/snippet}
	<div class="search">
		<SearchInput bind:value={search} placeholder="Search files" />
	</div>
	{#if action.error}<p class="error">{action.error}</p>{/if}
	<div class="library" class:flat={!folders.length}>
		{#if folders.length}
			<FolderTree
				{folders}
				current={search ? -1 : folder}
				onselect={select}
			/>
		{/if}
		<div class="card">
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
