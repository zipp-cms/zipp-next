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

	/**
	 * Images as tiles, other files as chips, in an order; a click opens
	 * one in the flyout for its title, alt text and focal point.
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
</script>

{#snippet itemActions(i: number)}
	<ItemActions onremove={() => set(removed(ids, i))} />
{/snippet}

{#snippet handle(i: number)}
	<DragHandle sortable={sort} index={i} label="Move the file" />
{/snippet}

<div class="assets" class:images bind:this={list}>
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
				<div class="tile"><span class="note">Loading…</span></div>
			{:else if images}
				<div class="tile">
					<button
						type="button"
						class="open plain"
						title="Details: title, alt text, focal point"
						onclick={() => openAsset(a.id)}
					>
						<Thumb asset={a} />
					</button>
					<span class="caption truncate" title={a.filename}>
						{a.title || a.filename}
					</span>
					<span class="reveal on-tile">
						{@render handle(i)}
						{@render itemActions(i)}
					</span>
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
		<div>
			<AddButton onclick={() => (picking = true)}>
				Choose {images ? 'image' : 'file'}
			</AddButton>
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

	.tile {
		position: relative;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		width: 7.5rem;
	}

	.open {
		display: block;

		@include hover {
			&:hover :global(.thumb) {
				border-color: var(--border-strong);
			}
		}
	}

	.gone {
		justify-content: space-between;
	}

	.caption {
		color: var(--muted);
		font-size: var(--fs-xs);
	}

	// over the top of the image
	.on-tile {
		position: absolute;
		top: 0.25rem;
		right: 0.25rem;
		display: flex;
		border-radius: var(--radius);
		background: var(--surface);
		box-shadow: var(--shadow);
	}
</style>
