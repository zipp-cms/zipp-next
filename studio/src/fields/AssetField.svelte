<script lang="ts">
	import { listAssets, type Asset, type Field, type Values } from '../api';
	import AssetPicker from '../lib/AssetPicker.svelte';
	import Thumb from '../lib/Thumb.svelte';
	import Icon from '../lib/Icon.svelte';

	let { field, values }: { field: Field; values: Values } = $props();

	const images = $derived(field.kind === 'assets' && field.images);
	const max = $derived(field.kind === 'assets' ? field.max : null);
	const ids = $derived<number[]>(values[field.handle] ?? []);
	const full = $derived(max !== null && ids.length >= max);

	let chosen = $state<Asset[]>([]);
	let picking = $state(false);
	// the assets of the field, in its order
	$effect(() => {
		if (!ids.length) return void (chosen = []);
		listAssets({ ids: ids.join(',') }).then(a => (chosen = a));
	});

	function toggle(a: Asset) {
		if (ids.includes(a.id)) {
			values[field.handle] = ids.filter(i => i !== a.id);
		} else if (!full) {
			values[field.handle] = [...ids, a.id];
		}
		// with a single one there is nothing more to pick
		if (max === 1) picking = false;
	}
</script>

<div class="assets">
	{#each chosen as a (a.id)}
		<div class="chosen">
			<Thumb asset={a} />
			<span class="caption" title={a.filename}>
				{a.title || a.filename}
			</span>
			<button
				type="button"
				class="remove"
				title="Remove"
				onclick={() => toggle(a)}
			>
				<Icon name="close" size={14} />
			</button>
		</div>
	{/each}
	{#if !full}
		<button type="button" class="choose" onclick={() => (picking = true)}>
			<Icon name="add" size={14} />
			Choose {images ? 'image' : 'file'}
		</button>
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
		flex-wrap: wrap;
		gap: 12px;
		align-items: flex-start;
	}

	.chosen {
		position: relative;
		display: flex;
		flex-direction: column;
		gap: 4px;
		width: 120px;
	}

	.caption {
		overflow: hidden;
		color: var(--muted);
		font-size: 11px;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	// on the tile, when the pointer is on it
	.remove {
		position: absolute;
		top: 5px;
		right: 5px;
		padding: 3px;
		border: none;
		border-radius: 50%;
		background: rgba(255, 255, 255, 0.92);
		color: var(--text);
		box-shadow: var(--shadow);
		opacity: 0;
		transition: opacity 0.15s;

		&:hover:not(:disabled) {
			background: white;
			color: var(--danger);
		}
	}

	.chosen:hover .remove,
	.remove:focus-visible {
		opacity: 1;
	}

	@media (hover: none) {
		.remove {
			opacity: 1;
		}
	}

	.choose {
		border-style: dashed;
		border-color: var(--border-strong);
		color: var(--accent);
	}
</style>
