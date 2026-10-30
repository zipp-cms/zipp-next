<script lang="ts">
	import { assetUrl, type Asset } from '../api';
	import Icon from './Icon.svelte';

	/** A square tile of a file: the image, or an icon with its name. */
	let { asset }: { asset: Asset } = $props();

	const ext = $derived(asset.filename.split('.').pop()?.toUpperCase() ?? '');
</script>

<div class="thumb">
	{#if asset.kind === 'image'}
		<img src={assetUrl(asset, true)} alt={asset.alt} loading="lazy" />
	{:else}
		<span class="file">
			<Icon name="single" size={28} />
			<span class="name">{asset.filename}</span>
		</span>
	{/if}
	<span class="ext">{ext}</span>
</div>

<style>
	.thumb {
		position: relative;
		width: 100%;
		aspect-ratio: 1;
		background: var(--surface-2);
		border: 1px solid var(--line);
		border-radius: var(--radius);
		overflow: hidden;
	}

	img {
		display: block;
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.file {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 8px;
		height: 100%;
		padding: 10px;
		color: var(--subtle);
	}

	.name {
		color: var(--text-2);
		font-size: var(--fs-xs);
		text-align: center;
		word-break: break-all;
	}

	.ext {
		position: absolute;
		top: 6px;
		left: 6px;
		padding: 0 5px;
		font-size: var(--fs-2xs);
		font-weight: 600;
		line-height: 16px;
		color: var(--text-2);
		background: rgba(255, 255, 255, 0.88);
		border-radius: var(--radius-sm);
	}
</style>
