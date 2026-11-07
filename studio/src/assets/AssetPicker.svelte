<script lang="ts">
	import { untrack } from 'svelte';
	import { t } from '../i18n/index.svelte';
	import type { Asset } from './api';
	import { AssetPickerState } from './assetPicker.svelte';
	import FolderTree from './FolderTree.svelte';
	import AssetGrid from './AssetGrid.svelte';
	import Flyout from 'zipp-ui/Flyout.svelte';
	import SearchInput from 'zipp-ui/SearchInput.svelte';
	import UploadButton from 'zipp-ui/UploadButton.svelte';
	import DropOverlay from './DropOverlay.svelte';
	import { FileDrag, uploads } from './uploads.svelte';
	import { acceptOf, fromInput, type Upload } from './files';

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

	const picker = untrack(
		() =>
			new AssetPickerState({
				images,
				extensions,
				folderPath,
				multiple,
				onpick: a => onpick(a),
			}),
	);
	const { lib } = picker;
	const upload = (items: Upload[]) => picker.upload(items);

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
		class:flat={!picker.folders.length}
		role="region"
		aria-label={t('assets.files')}
		ondragenter={drag.dragenter}
		ondragleave={drag.dragleave}
		ondragover={drag.dragover}
		ondrop={drag.drop}
	>
		{#if picker.folders.length}
			<FolderTree
				folders={picker.folders}
				top={lib.top}
				current={picker.current}
				searching={!!lib.search}
				onselect={id => picker.select(id)}
			/>
		{/if}
		<div class="card files">
			{#if drag.over}
				<DropOverlay label={t('assets.dropToUpload')} />
			{/if}
			<AssetGrid
				assets={picker.showNew ? [] : lib.assets}
				{selected}
				loading={!picker.showNew && !lib.loaded}
				empty={picker.showNew ? t('assets.newFolderEmpty') : lib.empty}
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
