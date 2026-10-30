<script lang="ts">
	import { assetUrl, type Asset } from './api';
	import Icon from '../ui/Icon.svelte';

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

<style lang="scss">
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
		gap: 0.5rem;
		height: 100%;
		padding: 0.625rem;
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
		top: 0.375rem;
		left: 0.375rem;
		padding: 0 0.3125rem;
		font-size: var(--fs-2xs);
		font-weight: 600;
		line-height: 1rem;
		color: var(--text-2);
		background: rgba(255, 255, 255, 0.88);
		border-radius: var(--radius-sm);
	}
</style>
