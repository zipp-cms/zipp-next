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
			<div class="tile"><Thumb asset={a} /></div>
			<button class="icon" title="Remove" onclick={() => toggle(a)}>
				<Icon name="delete" />
			</button>
		</div>
	{/each}
	{#if !full}
		<button onclick={() => (picking = true)}>
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
		gap: 16px;
		align-items: center;
	}

	.chosen {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.tile {
		width: 117px;
	}
</style>
