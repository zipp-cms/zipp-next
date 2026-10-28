<script lang="ts">
	import { assetUrl, type Asset } from '../api';

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
			{#if a.kind === 'image'}
				<img src={assetUrl(a, true)} alt={a.alt} loading="lazy" />
			{:else}
				<span class="file">{a.filename.split('.').pop()}</span>
			{/if}
			<span class="name">{a.filename}</span>
		</button>
	{:else}
		<p class="muted">No assets yet</p>
	{/each}
</div>

<style>
	.grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(8.5rem, 1fr));
		gap: 0.75rem;
	}

	.tile {
		display: flex;
		flex-direction: column;
		gap: 0.3rem;
		padding: 0.35rem;
		border: 2px solid transparent;
		border-radius: 0.4rem;
		background: none;
		text-align: left;
	}

	.selected {
		border-color: var(--accent);
	}

	img,
	.file {
		width: 100%;
		aspect-ratio: 1;
		object-fit: cover;
		border-radius: 0.3rem;
		background: var(--side);
	}

	.file {
		display: flex;
		align-items: center;
		justify-content: center;
		text-transform: uppercase;
		color: var(--muted);
	}

	.name {
		font-size: 0.8rem;
		color: var(--muted);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.muted {
		color: var(--muted);
	}
</style>
