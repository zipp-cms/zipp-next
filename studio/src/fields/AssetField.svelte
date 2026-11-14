<script lang="ts">
	import { onlyVideos, type Asset } from '../assets/api';
	import type { FieldOf, Values } from '../core/schema';
	import { valueOf } from '../core/schema';
	import AddButton from 'zipp-ui/AddButton.svelte';
	import AssetChip from '../assets/AssetChip.svelte';
	import AssetPicker from '../assets/AssetPicker.svelte';
	import ItemActions from 'zipp-ui/ItemActions.svelte';
	import DragHandle from 'zipp-ui/DragHandle.svelte';
	import SortLine from 'zipp-ui/SortLine.svelte';
	import { listSortable } from 'zipp-ui/sortable';
	import AssetCard from '../assets/AssetCard.svelte';
	import { linkItems } from '../assets/actions';
	import { chosenAssets } from '../assets/chosen.svelte';
	import { openAsset } from '../core/flyout.svelte';
	import { inserted, isFull, removed, replaced } from '../core/list';
	import Icon from 'zipp-ui/Icon.svelte';
	import UploadButton from 'zipp-ui/UploadButton.svelte';
	import { FileDrag, uploads } from '../assets/uploads.svelte';
	import {
		acceptOf,
		prepare,
		pathDirs,
		fromInput,
		type Upload,
	} from '../assets/files';
	import { t } from '../i18n/index.svelte';
	import { getEditing } from '../core/editing';
	import { fieldFolder } from '../assets/folders';

	/**
	 * Images and videos as cards, other files as chips, in an order; a
	 * click opens one in the flyout for its title, alt text and focal
	 * point. Files dropped on the field or uploaded with its drop zone
	 * are added.
	 */
	let { field, values }: { field: FieldOf<'assets'>; values: Values } =
		$props();

	const images = $derived(field.images);
	const videos = $derived(onlyVideos(field.extensions));
	const cards = $derived(images || videos);
	const max = $derived(field.max);
	const ids = $derived(valueOf(values, field) ?? []);
	const full = $derived(isFull(ids.length, max));
	const set = (next: number[]) => (values[field.handle] = next);
	// of the entry around, also in its blocks
	const editing = getEditing();
	const folder = $derived(fieldFolder(field.folder, editing));

	const chosen = chosenAssets(() => ids);
	let picking = $state(false);
	// the file another one from the library replaces
	let replacing = $state<number | null>(null);

	// cards side by side, files one under the other
	let list = $state<HTMLElement>();
	const sort = listSortable(
		() => list,
		() => ids,
		set,
		{ grid: () => cards },
	);

	function toggle(a: Asset) {
		const i = ids.indexOf(a.id);
		if (i >= 0) set(removed(ids, i));
		else if (!full) set(inserted(ids, a.id));
		// with a single one there is nothing more to pick
		if (max === 1) picking = false;
	}

	/** The file `replacing` swapped for `a`, in its place. */
	function replace(a: Asset) {
		// an upload that ends after the picker closed
		const i = replacing === null ? -1 : ids.indexOf(replacing);
		replacing = null;
		if (i < 0) return;

		const next = replaced(ids, i, a.id);
		set(next.filter((id, k) => k === i || id !== a.id));
	}

	/**
	 * New files into the field's folder (made when missing) or the
	 * library, the ones it takes, as many as fit.
	 */
	async function upload(items: Upload[]) {
		const room = max ? max - ids.length : items.length;
		items = prepare(items, field, room, pathDirs(folder));
		const added = await uploads.run(items, null);
		let next = ids;
		for (const a of added)
			if (!isFull(next.length, max)) next = inserted(next, a.id);
		set(next);
	}

	const drag = new FileDrag(upload, () => !full);
	const one = $derived(max === 1);
	const types = $derived(
		field.extensions?.map(e => e.toUpperCase()).join(', '),
	);
	const dragHint = $derived(
		types
			? t(one ? 'fields.dragType' : 'fields.dragTypes', { types })
			: videos
				? t(one ? 'fields.dragVideo' : 'fields.dragVideos')
				: images
					? t(one ? 'fields.dragImage' : 'fields.dragImages')
					: t(one ? 'fields.dragFile' : 'fields.dragFiles'),
	);
</script>

{#snippet itemActions(i: number)}
	<ItemActions onremove={() => set(removed(ids, i))} />
{/snippet}

{#snippet handle(i: number)}
	<DragHandle sortable={sort} index={i} label={t('fields.moveFile')} />
{/snippet}

<div
	class="assets"
	class:images={cards}
	bind:this={list}
	role="group"
	aria-label={field.label}
	ondragenter={drag.dragenter}
	ondragleave={drag.dragleave}
	ondragover={drag.dragover}
	ondrop={drag.drop}
>
	<!-- by the ids of the field: its order, also while loading -->
	{#each ids as id, i (id)}
		{@const a = chosen.get(id)}
		<div data-sort class="item" class:sort-dragging={sort.moving(i)}>
			{#if !cards}{@render handle(i)}{/if}
			{#if a === null}
				<div class="chip box gone">
					<span class="note">{t('fields.deletedFile')}</span>
					{@render itemActions(i)}
				</div>
			{:else if !a}
				<div class="loading">
					<span class="note">{t('common.loading')}</span>
				</div>
			{:else if cards}
				<div class="card-size">
					<AssetCard
						asset={a}
						openTitle={t('assets.details')}
						onopen={() => openAsset(a.id)}
						menu={[
							{
								label: t('fields.details'),
								icon: 'edit',
								onclick: () => openAsset(a.id),
							},
							{
								label: t('fields.replace'),
								icon: 'replace',
								onclick: () => (replacing = a.id),
							},
							...linkItems(a),
							{
								label: t('ui.remove'),
								icon: 'close',
								danger: true,
								onclick: () => set(removed(ids, i)),
							},
						]}
						handle={ids.length > 1 ? cardHandle : undefined}
					/>
					{#snippet cardHandle()}{@render handle(i)}{/snippet}
				</div>
			{:else}
				<AssetChip asset={a}>
					{#snippet actions()}{@render itemActions(i)}{/snippet}
				</AssetChip>
			{/if}
		</div>
	{/each}
	<SortLine sortable={sort} />
	{#if !full && cards && ids.length}
		<!-- next to the cards in their shape: a click chooses from the
		library, its bar uploads -->
		<div class="drop-zone tile" class:over={drag.over}>
			<button
				type="button"
				class="choose plain"
				onclick={() => (picking = true)}
			>
				<Icon name={videos ? 'add-video' : 'add-image'} size={22} />
				<span class="label">
					{drag.over
						? t('fields.dropToAdd')
						: t(videos ? 'fields.addVideos' : 'fields.addImages')}
				</span>
				<span class="hint">
					{t('fields.orDropThem')}
				</span>
				{#if types}<span class="types">{types}</span>{/if}
			</button>
			<span class="bar">
				<UploadButton
					onfiles={f => upload(fromInput(f))}
					multiple={!one}
					accept={acceptOf(field)}
					icon="upload"
				>
					{t('common.upload')}
				</UploadButton>
			</span>
		</div>
	{:else if !full}
		<!-- a row, also of an empty image field: a click on the zone
		chooses from the library -->
		<div
			class="drop-zone"
			class:over={drag.over}
			role="presentation"
			onclick={e => {
				if (e.target === e.currentTarget) picking = true;
			}}
		>
			<Icon name="upload" size={18} />
			<span class="hint">
				{drag.over ? t('fields.dropToAdd') : dragHint}
			</span>
			<span class="buttons">
				<AddButton onclick={() => (picking = true)}>
					{t('common.choose')}
				</AddButton>
				<UploadButton
					onfiles={f => upload(fromInput(f))}
					multiple={!one}
					accept={acceptOf(field)}
					icon="upload"
				>
					{t('common.upload')}
				</UploadButton>
			</span>
		</div>
	{/if}
</div>

{#if replacing !== null}
	<AssetPicker
		{images}
		extensions={field.extensions}
		folderPath={folder}
		multiple={false}
		selected={[replacing]}
		onpick={replace}
		onclose={() => (replacing = null)}
	/>
{:else if picking}
	<AssetPicker
		{images}
		extensions={field.extensions}
		folderPath={folder}
		multiple={!one}
		selected={ids}
		onpick={toggle}
		onclose={() => (picking = false)}
	/>
{/if}

<style lang="scss">
	.assets {
		position: relative;
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	// images and videos as cards side by side
	.images {
		flex-direction: row;
		flex-wrap: wrap;
		gap: 0.75rem;
		align-items: flex-start;
	}

	// a file: its handle, then its chip
	.item {
		display: flex;
		align-items: center;
		gap: 0.25rem;

		> :global(:last-child) {
			flex: 1;
			min-width: 0;
		}
	}

	.images .item {
		display: block;
	}

	// an image card, and the box while it loads
	.card-size,
	.loading {
		width: 10rem;
	}

	.loading {
		display: grid;
		place-items: center;
		aspect-ratio: 1;
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
	}

	.gone {
		justify-content: space-between;
	}

	// further from the files than they are from each other, like the
	// Choose of a relation; cards keep the gap of their grid
	.assets:not(.images) .item ~ .drop-zone {
		margin-top: 0.375rem;
	}

	// where files are dropped, uploaded or chosen
	.drop-zone {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		padding: 0.625rem 0.75rem;
		border: 1px dashed var(--border-strong);
		border-radius: var(--radius-lg);
		color: var(--muted);
		cursor: pointer;

		> :global(.icon),
		.hint {
			pointer-events: none;
		}

		&.over {
			border-color: var(--accent);
			background: var(--accent-soft);
			color: var(--accent);
		}
	}

	.buttons {
		display: flex;
		flex-wrap: wrap;
		gap: 0.375rem;
		margin-left: auto;
		cursor: auto;
	}

	// as large as a card: the choice where its picture is, the upload
	// where its title is
	.tile {
		flex-direction: column;
		align-items: stretch;
		gap: 0;
		width: 10rem;
		aspect-ratio: 1;
		padding: 0;
		background: var(--surface-2);
		cursor: auto;

		@include hover {
			&:hover {
				border-color: var(--accent);
			}
		}
	}

	.choose {
		display: flex;
		flex: 1;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.25rem;
		padding: 0.5rem;
		color: var(--muted);
		font-size: var(--fs-sm);
		font-weight: 400;
		text-align: center;

		> :global(.icon) {
			margin-bottom: 0.25rem;
			color: var(--subtle);
		}

		@include hover {
			&:hover .label {
				text-decoration: underline;
			}
		}
	}

	.label {
		color: var(--accent);
		font-size: var(--fs-md);
		font-weight: 500;
	}

	.types {
		color: var(--subtle);
		font-size: var(--fs-xs);
	}

	.tile .bar {
		display: flex;
		min-height: 2.25rem;
		border-top: 1px dashed var(--border-strong);

		:global(.upload) {
			flex: 1;
			justify-content: center;
			padding: 0 0.625rem;
			border: none;
			border-radius: 0 0 calc(var(--radius-lg) - 1px)
				calc(var(--radius-lg) - 1px);
			background: none;
			color: var(--muted);
			font-size: var(--fs-sm);
		}

		@include hover {
			:global(.upload:hover) {
				background: var(--hover);
				color: var(--text);
			}
		}
	}

	.tile.over {
		.choose,
		.choose > :global(.icon),
		.label {
			color: var(--accent);
		}
	}
</style>
