<script lang="ts">
	import type { Asset } from '../api';
	import Thumb from './Thumb.svelte';

	/** Thumbnails, files show their name. */
	let {
		assets,
		selected = [],
		onpick,
	}: {
		assets: Asset[];
		selected?: number[];
		onpick: (a: Asset) => void;
	} = $props();
</script>

<div class="grid">
	{#each assets as a (a.id)}
		<button
			class="tile"
			class:selected={selected.includes(a.id)}
			onclick={() => onpick(a)}
			title={a.filename}
		>
			<Thumb asset={a} />
		</button>
	{:else}
		<p class="muted">No assets yet</p>
	{/each}
</div>

<style>
	.grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
		gap: 20px;
	}

	.tile {
		padding: 0;
		border: 3px solid transparent;
		border-radius: 0;
		background: none;
	}

	.selected {
		border-color: var(--dark);
	}

	.muted {
		color: var(--muted);
	}
</style>
