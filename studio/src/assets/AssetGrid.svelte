<script lang="ts">
	import type { Asset } from './api';
	import Thumb from './Thumb.svelte';
	import Icon from 'zipp-ui/Icon.svelte';

	/** Thumbnails, files show their name. */
	let {
		assets,
		selected = [],
		loading = false,
		onpick,
	}: {
		assets: Asset[];
		loading?: boolean;
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
		<p class="empty">{loading ? 'Loading…' : 'No assets yet'}</p>
	{/each}
</div>

<style lang="scss">
	.grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(8.75rem, 1fr));
		gap: 1rem;
	}

	.tile {
		position: relative;
		display: flex;
		flex-direction: column;
		align-items: stretch;
		gap: 0.375rem;
		padding: 0;
		border: none;
		border-radius: var(--radius-lg);
		background: none;
		font-weight: 400;
		text-align: left;

		@include hover {
			&:hover:not(:disabled) {
				background: none;
			}
		}

		@include hover {
			&:hover :global(.thumb) {
				border-color: var(--border-strong);
			}
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
		@include focus-ring(1px);
	}

	.check {
		position: absolute;
		top: 0.375rem;
		right: 0.375rem;
		display: grid;
		place-items: center;
		width: 1.375rem;
		height: 1.375rem;
		border-radius: 50%;
		background: var(--accent);
		color: var(--on-accent);
	}
</style>
