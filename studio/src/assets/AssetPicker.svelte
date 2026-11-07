<script lang="ts" module>
	/** The folder the picker showed last, it opens there again. */
	let lastFolder: number | null = null;
</script>

<script lang="ts">
	import { untrack } from 'svelte';
	import { t } from '../i18n/index.svelte';
	import { type Asset, type Folder } from './api';
	import { folderByPath, missingFolders } from './folders';
	import FolderTree from './FolderTree.svelte';
	import AssetGrid from './AssetGrid.svelte';
	import Flyout from 'zipp-ui/Flyout.svelte';
	import SearchInput from 'zipp-ui/SearchInput.svelte';
	import UploadButton from 'zipp-ui/UploadButton.svelte';
	import DropOverlay from './DropOverlay.svelte';
	import { Library } from './library.svelte';
	import { FileDrag, uploads } from './uploads.svelte';
	import {
		acceptOf,
		pathDirs,
		prepare,
		fromInput,
		type Upload,
	} from './files';

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

	// the field's folder before it exists (a new topic's `{slug}`): shown
	// in the tree, empty, and made with the first upload into it
	const NEW = -1;
	let ready = $state(false);
	let inNew = $state(false);
	const missing = $derived(
		ready ? missingFolders(lib.folders, folderPath) : null,
	);
	const folders = $derived<Folder[]>(
		missing
			? [
					...lib.folders,
					{
						id: NEW,
						parentId: missing.parent,
						name: missing.names.join(' / '),
						assets: 0,
					},
				]
			: lib.folders,
	);
	const showNew = $derived(inNew && !!missing && !lib.search);

	// the field's folder first, else where the picker was
	lib.loadFolders().then(() => {
		ready = true;
		if (missing) {
			inNew = true;
			return;
		}
		const own = folderByPath(lib.folders, folderPath);
		const last = lib.folders.some(f => f.id === lastFolder);
		lib.select(own ? own.id : last ? lastFolder : null);
	});

	function select(id: number | null) {
		inNew = id === NEW;
		if (inNew) {
			lib.search = '';
			return;
		}
		lastFolder = id;
		lib.select(id);
	}

	/**
	 * Into the open folder, and picked; into the field's new folder, made
	 * now; on the top with a folder of the field, into that (made when
	 * missing).
	 */
	async function upload(items: Upload[]) {
		const intoNew = showNew ? missing : null;
		const dir = intoNew
			? intoNew.names
			: lib.folder === null
				? pathDirs(folderPath)
				: [];
		const room = multiple ? items.length : 1;
		items = prepare(items, { images, extensions }, room, dir);
		const added = await uploads.run(
			items,
			intoNew ? intoNew.parent : lib.folder,
			() => lib.loadFolders(),
		);
		for (const a of added) onpick(a);

		// the new folder is there now, it opens with its files
		if (intoNew) {
			await lib.loadFolders();
			const own = folderByPath(lib.folders, folderPath);
			if (inNew && own) select(own.id);
			return;
		}
		lib.assets = [...added, ...lib.assets];
	}

	const drag = new FileDrag(upload, () => true);

	const title = $derived(
		images
			? multiple
				? t('assets.chooseImages')
				: t('assets.chooseImage')
			: multiple
				? t('assets.chooseFiles')
				: t('assets.chooseFile'),
	);
</script>

<Flyout {title} {onclose}>
	{#snippet actions()}
		<UploadButton
			onfiles={f => upload(fromInput(f))}
			{multiple}
			accept={acceptOf({ images, extensions })}
		>
			{uploads.active ? t('assets.uploading') : t('common.upload')}
		</UploadButton>
		<button type="button" class="primary" onclick={onclose}>
			{t('ui.done')}
		</button>
	{/snippet}
	<div class="search">
		<SearchInput
			bind:value={() => lib.search, v => lib.type(v)}
			placeholder={t('assets.searchFiles')}
		/>
	</div>
	{#if lib.error}<p class="error">{lib.error}</p>{/if}
	<div
		class="library"
		class:flat={!folders.length}
		role="region"
		aria-label={t('assets.files')}
		ondragenter={drag.dragenter}
		ondragleave={drag.dragleave}
		ondragover={drag.dragover}
		ondrop={drag.drop}
	>
		{#if folders.length}
			<FolderTree
				{folders}
				top={lib.top}
				current={showNew ? NEW : lib.folder}
				searching={!!lib.search}
				onselect={select}
			/>
		{/if}
		<div class="card files">
			{#if drag.over}
				<DropOverlay label={t('assets.dropToUpload')} />
			{/if}
			<AssetGrid
				assets={showNew ? [] : lib.assets}
				{selected}
				loading={!showNew && !lib.loaded}
				empty={showNew ? t('assets.newFolderEmpty') : lib.empty}
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
			grid-template-columns: minmax(0, 1fr);
		}

		@include max-tablet {
			grid-template-columns: minmax(0, 1fr);
		}
	}
</style>
