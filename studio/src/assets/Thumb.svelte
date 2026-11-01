<script lang="ts">
	import { altOf, assetUrl, isVideo, type Asset } from './api';
	import { duration } from '../core/format';
	import { site } from '../core/router.svelte';
	import Icon from 'zipp-ui/Icon.svelte';

	/**
	 * A square tile of a file: the image, a video's first frame with how
	 * long it plays, or an icon with its name (without `named` only the
	 * icon, the name is next to it). `small` is for a row's few pixels:
	 * an icon and a name don't fit there, a file shows only its type and
	 * an image only itself.
	 */
	let {
		asset,
		named = true,
		small = false,
	}: { asset: Asset; named?: boolean; small?: boolean } = $props();

	const ext = $derived(asset.filename.split('.').pop()?.toUpperCase() ?? '');
	// the browser knows it once it read the start of the file
	let seconds = $state(0);
</script>

<div class="thumb">
	{#if asset.kind === 'image'}
		<img
			src={assetUrl(asset, true)}
			alt={altOf(asset, site.current)}
			loading="lazy"
		/>
	{:else if isVideo(asset) && !small}
		<!-- a frame just after the start, the server makes no posters;
		only the start of the file loads -->
		<video
			src="{assetUrl(asset)}#t=0.1"
			preload="metadata"
			muted
			playsinline
			tabindex="-1"
			bind:duration={seconds}
		></video>
		<span class="play"><Icon name="play" size={18} /></span>
		{#if seconds}<span class="duration">{duration(seconds)}</span>{/if}
	{:else if small}
		<span class="type">{ext}</span>
	{:else}
		<span class="file">
			<Icon name="single" size={28} />
			{#if named}<span class="name">{asset.filename}</span>{/if}
		</span>
	{/if}
	{#if !small}<span class="ext">{ext}</span>{/if}
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

	img,
	video {
		display: block;
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	video {
		pointer-events: none;
	}

	.play {
		position: absolute;
		top: 50%;
		left: 50%;
		display: grid;
		place-items: center;
		width: 2.5rem;
		height: 2.5rem;
		color: var(--text);
		background: rgba(255, 255, 255, 0.88);
		border-radius: 50%;
		transform: translate(-50%, -50%);
	}

	.duration {
		position: absolute;
		right: 0.375rem;
		bottom: 0.375rem;
		padding: 0 0.3125rem;
		font-size: var(--fs-2xs);
		font-weight: 600;
		line-height: 1rem;
		color: var(--text-2);
		background: rgba(255, 255, 255, 0.88);
		border-radius: var(--radius-sm);
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

	.type {
		display: grid;
		place-items: center;
		height: 100%;
		overflow: hidden;
		font-size: var(--fs-2xs);
		font-weight: 600;
		color: var(--text-2);
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
