<script lang="ts">
	import { assetUrl, type Asset } from '../api';

	/** A square tile with the file type, like the media of the old Zipp. */
	let { asset }: { asset: Asset } = $props();

	const ext = $derived(asset.filename.split('.').pop()?.toUpperCase() ?? '');
</script>

<div class="thumb">
	{#if asset.kind === 'image'}
		<img src={assetUrl(asset, true)} alt={asset.alt} loading="lazy" />
	{:else}
		<span class="name">{asset.filename}</span>
	{/if}
	<span class="ext">{ext}</span>
</div>

<style>
	.thumb {
		position: relative;
		width: 100%;
		aspect-ratio: 1;
		background: var(--bg);
		overflow: hidden;
	}

	img {
		display: block;
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.ext {
		position: absolute;
		top: 8px;
		left: 6px;
		padding: 1px 5px;
		font-size: 12px;
		color: white;
		background: black;
	}

	.name {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		padding: 4px 6px;
		font-size: 12px;
		color: white;
		background: black;
		word-break: break-all;
	}
</style>
