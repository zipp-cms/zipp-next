<script lang="ts" module>
	/** The folder the picker showed last, it opens there again. */
	let lastFolder: number | null = null;
</script>

<script lang="ts">
	import { untrack } from 'svelte';
	import { folderByPath, type Asset } from './api';
	import FolderTree from './FolderTree.svelte';
	import AssetGrid from './AssetGrid.svelte';
	import Flyout from 'zipp-ui/Flyout.svelte';
	import SearchInput from 'zipp-ui/SearchInput.svelte';
	import UploadButton from 'zipp-ui/UploadButton.svelte';
	import DropOverlay from './DropOverlay.svelte';
	import { Library } from './library.svelte';
	import {
		FileDrag,
		acceptOf,
		pathDirs,
		prepare,
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

	const lib = untrack(() => new Library({ images, extensions }));

	// the field's folder first, else where the picker was
	lib.loadFolders().then(() => {
		const own = folderByPath(lib.folders, folderPath);
		const last = lib.folders.some(f => f.id === lastFolder);
		lib.select(own ? own.id : last ? lastFolder : null);
	});

	function select(id: number | null) {
		lastFolder = id;
		lib.select(id);
	}

	/**
	 * Into the open folder, and picked; on the top with a folder of the
	 * field, into that (made when missing).
	 */
	async function upload(items: Upload[]) {
		const dir = lib.folder === null ? pathDirs(folderPath) : [];
		const room = multiple ? items.length : 1;
		items = prepare(items, { images, extensions }, room, dir);
		const added = await uploads.run(items, lib.folder, () =>
			lib.loadFolders(),
		);
		lib.assets = [...added, ...lib.assets];
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
		<SearchInput
			bind:value={() => lib.search, v => lib.type(v)}
			placeholder="Search files"
		/>
	</div>
	{#if lib.error}<p class="error">{lib.error}</p>{/if}
	<div
		class="library"
		class:flat={!lib.folders.length}
		role="region"
		aria-label="Files"
		ondragenter={drag.dragenter}
		ondragleave={drag.dragleave}
		ondragover={drag.dragover}
		ondrop={drag.drop}
	>
		{#if lib.folders.length}
			<FolderTree
				folders={lib.folders}
				top={lib.top}
				current={lib.folder}
				searching={!!lib.search}
				onselect={select}
			/>
		{/if}
		<div class="card files">
			{#if drag.over}
				<DropOverlay label="Drop to upload" />
			{/if}
			<AssetGrid
				assets={lib.assets}
				{selected}
				loading={!lib.loaded}
				empty={lib.empty}
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
