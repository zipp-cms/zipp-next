<script lang="ts">
	import type { Asset } from '../assets/api';
	import type { Field, Values } from '../core/schema';
	import AddButton from '../ui/AddButton.svelte';
	import AssetChip from '../assets/AssetChip.svelte';
	import AssetPicker from '../assets/AssetPicker.svelte';
	import ItemActions from '../ui/ItemActions.svelte';
	import Thumb from '../assets/Thumb.svelte';
	import { chosenAssets } from '../assets/chosen.svelte';
	import { openAsset } from '../core/flyout.svelte';
	import { inserted, isFull, moved, removed } from '../core/list';

	/**
	 * Images as tiles, other files as chips, in an order; a click opens
	 * one in the flyout for its title, alt text and focal point.
	 */
	let { field, values }: { field: Field; values: Values } = $props();

	const images = $derived(field.kind === 'assets' && field.images);
	const max = $derived(field.kind === 'assets' ? field.max : null);
	const ids = $derived<number[]>(values[field.handle] ?? []);
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
	{#each chosen.list as a, i (a.id)}
		{#if images}
			<div class="tile">
				<button
					type="button"
					class="open"
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
		gap: 6px;
	}

	.images {
		flex-direction: row;
		flex-wrap: wrap;
		gap: 12px;
		align-items: flex-start;
	}

	.tile {
		position: relative;
		display: flex;
		flex-direction: column;
		gap: 4px;
		width: 120px;
	}

	.open {
		display: block;
		padding: 0;
		border: none;
		background: none;

		&:hover:not(:disabled) {
			background: none;
		}

		&:hover :global(.thumb) {
			border-color: var(--border-strong);
		}
	}

	.caption {
		color: var(--muted);
		font-size: var(--fs-xs);
	}

	// over the top of the image
	.on-tile {
		position: absolute;
		top: 4px;
		right: 4px;
		border-radius: var(--radius);
		background: var(--surface);
		box-shadow: var(--shadow);
	}
</style>
