<script lang="ts">
	import type { Asset } from '../api';
	import Thumb from './Thumb.svelte';
	import Icon from './Icon.svelte';

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
			<span class="caption">{a.title || a.filename}</span>
			{#if selected.includes(a.id)}
				<span class="check"><Icon name="save" size={14} /></span>
			{/if}
		</button>
	{:else}
		<p class="muted">No assets yet</p>
	{/each}
</div>

<style lang="scss">
	.grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
		gap: 16px;
	}

	.tile {
		position: relative;
		display: flex;
		flex-direction: column;
		align-items: stretch;
		gap: 6px;
		padding: 0;
		border: none;
		border-radius: var(--radius-lg);
		background: none;
		font-weight: 400;
		text-align: left;

		&:hover:not(:disabled) {
			background: none;
		}

		&:hover :global(.thumb) {
			border-color: var(--border-strong);
		}
	}

	.caption {
		overflow: hidden;
		color: var(--text-2);
		font-size: var(--fs-sm);
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.selected :global(.thumb) {
		outline: 2px solid var(--accent);
		outline-offset: 1px;
	}

	.check {
		position: absolute;
		top: 6px;
		right: 6px;
		display: grid;
		place-items: center;
		width: 22px;
		height: 22px;
		border-radius: 50%;
		background: var(--accent);
		color: white;
	}
</style>
