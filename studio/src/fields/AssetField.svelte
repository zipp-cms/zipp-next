<script lang="ts">
	import type { Asset } from '../assets/api';
	import type { FieldOf, Values } from '../core/schema';
	import { valueOf } from '../core/schema';
	import AddButton from 'zipp-ui/AddButton.svelte';
	import AssetChip from '../assets/AssetChip.svelte';
	import AssetPicker from '../assets/AssetPicker.svelte';
	import ItemActions from 'zipp-ui/ItemActions.svelte';
	import Thumb from '../assets/Thumb.svelte';
	import { chosenAssets } from '../assets/chosen.svelte';
	import { openAsset } from '../core/flyout.svelte';
	import { inserted, isFull, moved, removed } from '../core/list';

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

	function toggle(a: Asset) {
		const i = ids.indexOf(a.id);
		if (i >= 0) set(removed(ids, i));
		else if (!full) set(inserted(ids, a.id));
		// with a single one there is nothing more to pick
		if (max === 1) picking = false;
	}
</script>

{#snippet itemActions(i: number)}
	<ItemActions
		index={i}
		count={ids.length}
		horizontal={images}
		onmove={by => set(moved(ids, i, by))}
		onremove={() => set(removed(ids, i))}
	/>
{/snippet}

<div class="assets" class:images>
	<!-- by the ids of the field: its order, also while loading -->
	{#each ids as id, i (id)}
		{@const a = chosen.get(id)}
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
				<span class="reveal on-tile">{@render itemActions(i)}</span>
			</div>
		{:else}
			<AssetChip asset={a}>
				{#snippet actions()}{@render itemActions(i)}{/snippet}
			</AssetChip>
		{/if}
	{/each}
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
		border-radius: var(--radius);
		background: var(--surface);
		box-shadow: var(--shadow);
	}
</style>
