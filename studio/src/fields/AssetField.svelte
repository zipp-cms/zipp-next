<script lang="ts">
	import type { Asset } from '../assets/api';
	import type { FieldOf, Values } from '../core/schema';
	import { valueOf } from '../core/schema';
	import AddButton from 'zipp-ui/AddButton.svelte';
	import AssetChip from '../assets/AssetChip.svelte';
	import AssetPicker from '../assets/AssetPicker.svelte';
	import ItemActions from 'zipp-ui/ItemActions.svelte';
	import DragHandle from 'zipp-ui/DragHandle.svelte';
	import SortLine from 'zipp-ui/SortLine.svelte';
	import { Sortable, moved } from 'zipp-ui/sortable';
	import Thumb from '../assets/Thumb.svelte';
	import { chosenAssets } from '../assets/chosen.svelte';
	import { openAsset } from '../core/flyout.svelte';
	import { inserted, isFull, removed } from '../core/list';
	import Icon from 'zipp-ui/Icon.svelte';
	import UploadButton from 'zipp-ui/UploadButton.svelte';
	import {
		FileDrag,
		fromInput,
		uploads,
		type Upload,
	} from '../assets/uploads.svelte';

	/**
	 * Images as cards, other files as chips, in an order; a click opens
	 * one in the flyout for its title, alt text and focal point. Files
	 * dropped on the field or uploaded with its drop zone are added.
	 */
	let { field, values }: { field: FieldOf<'assets'>; values: Values } =
		$props();

	const images = $derived(field.images);
	const max = $derived(field.max);
	const ids = $derived(valueOf(values, field) ?? []);
	const full = $derived(isFull(ids.length, max));
	const set = (next: number[]) => (values[field.handle] = next);

	const chosen = chosenAssets(() => ids);
	let picking = $state(false);

	// images side by side, files one under the other
	let list = $state<HTMLElement>();
	const sort = new Sortable({
		container: () => list,
		depths: () => ids.map(() => 0),
		grid: () => images,
		onmove: (from, drop) => set(moved(ids, from, drop)),
	});

	function toggle(a: Asset) {
		const i = ids.indexOf(a.id);
		if (i >= 0) set(removed(ids, i));
		else if (!full) set(inserted(ids, a.id));
		// with a single one there is nothing more to pick
		if (max === 1) picking = false;
	}

	/** New files into Files, added as far as the field takes them. */
	async function upload(items: Upload[]) {
		if (images) items = items.filter(i => i.file.type.startsWith('image/'));
		const room = max ? max - ids.length : items.length;
		items = items.slice(0, Math.max(room, 0)).map(i => ({ ...i, dir: [] }));
		const added = await uploads.run(items, null);
		let next = ids;
		for (const a of added)
			if (!isFull(next.length, max)) next = inserted(next, a.id);
		set(next);
	}

	const drag = new FileDrag(upload, () => !full);
	const kind = $derived(images ? 'images' : 'files');
</script>

{#snippet itemActions(i: number)}
	<ItemActions onremove={() => set(removed(ids, i))} />
{/snippet}

{#snippet handle(i: number)}
	<DragHandle sortable={sort} index={i} label="Move the file" />
{/snippet}

<div
	class="assets"
	class:images
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
		<div data-sort class="item" class:sort-dragging={sort.dragging === i}>
			{#if !images}{@render handle(i)}{/if}
			{#if a === null}
				<div class="chip box gone">
					<span class="note">A file that was deleted</span>
					{@render itemActions(i)}
				</div>
			{:else if !a}
				<div class="tile loading">
					<span class="note">Loading…</span>
				</div>
			{:else if images}
				<div class="tile">
					<button
						type="button"
						class="open plain"
						title="Details: title, alt text, focal point"
						onclick={() => openAsset(a.id)}
					>
						<Thumb asset={a} />
						<span class="caption" title={a.filename}>
							<span class="truncate">
								{a.title || a.filename}
							</span>
						</span>
					</button>
					{#if ids.length > 1}
						<span class="on-tile handle">{@render handle(i)}</span>
					{/if}
					<button
						type="button"
						class="on-tile remove"
						title="Remove from the field, the file stays"
						aria-label="Remove"
						onclick={() => set(removed(ids, i))}
					>
						<Icon name="close" size={14} />
					</button>
				</div>
			{:else}
				<AssetChip asset={a}>
					{#snippet actions()}{@render itemActions(i)}{/snippet}
				</AssetChip>
			{/if}
		</div>
	{/each}
	<SortLine sortable={sort} />
	{#if !full}
		<!-- a click on the zone chooses from the library -->
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
				{drag.over ? 'Drop to add' : `Drag ${kind} here`}
			</span>
			<span class="buttons">
				<AddButton onclick={() => (picking = true)}>Choose</AddButton>
				<UploadButton
					onfiles={f => upload(fromInput(f))}
					multiple={max !== 1}
					accept={images ? 'image/*' : undefined}
					icon="upload"
				>
					Upload
				</UploadButton>
			</span>
		</div>
	{/if}
</div>

{#if picking}
	<AssetPicker
		{images}
		multiple={max !== 1}
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

	// an image: square with the picture and its title in a bar below
	.tile {
		position: relative;
		display: flex;
		flex-direction: column;
		width: 10rem;
		aspect-ratio: 1;
		overflow: hidden;
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
		background: var(--surface);

		@include hover {
			&:hover {
				border-color: var(--border-strong);
			}
		}

		// the card has the border and the square, the type is in the title
		:global(.thumb) {
			flex: 1;
			min-height: 0;
			aspect-ratio: auto;
			border: none;
			border-radius: 0;
		}

		:global(.ext) {
			display: none;
		}
	}

	.loading {
		display: grid;
		place-items: center;
		aspect-ratio: 1;
	}

	.open {
		display: flex;
		flex: 1;
		flex-direction: column;
		align-items: stretch;
		gap: 0;
		min-height: 0;
		width: 100%;
		text-align: left;
	}

	.gone {
		justify-content: space-between;
	}

	.caption {
		display: flex;
		align-items: baseline;
		gap: 0.375rem;
		padding: 0.4375rem 0.625rem;
		border-top: 1px solid var(--border);
		font-size: var(--fs-sm);
		font-weight: 500;
	}

	// over the corners of the image
	.on-tile {
		position: absolute;
		top: 0.375rem;
		display: grid;
		place-items: center;
		width: 1.75rem;
		height: 1.75rem;
		border-radius: var(--radius);
		box-shadow: var(--shadow);
	}

	.handle {
		left: 0.375rem;
		background: var(--surface);
	}

	.remove {
		right: 0.375rem;
		padding: 0;
		border: none;
		background: color-mix(in srgb, var(--text) 80%, transparent);
		color: var(--surface);

		@include hover {
			&:hover:not(:disabled) {
				background: var(--danger);
				color: var(--surface);
			}
		}
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

	// as large as an image, the buttons under each other
	.images .drop-zone {
		flex-direction: column;
		justify-content: center;
		width: 10rem;
		aspect-ratio: 1;
		padding: 0.625rem;
		font-size: var(--fs-sm);
		text-align: center;

		.buttons {
			flex-direction: column;
			align-items: stretch;
			margin: 0.25rem 0 0;
		}
	}
</style>
