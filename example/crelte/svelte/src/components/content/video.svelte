<script>
	let { video, poster, loop } = $props();

	const v = $derived(video[0]);
</script>

<!-- a moving image plays by itself, muted; else with the controls -->
{#if v}
	<figure>
		{#if loop}
			<video
				src={v.url}
				poster={poster[0]?.url}
				autoplay
				muted
				loop
				playsinline
			></video>
		{:else}
			<!-- svelte-ignore a11y_media_has_caption -->
			<video
				src={v.url}
				poster={poster[0]?.url}
				controls
				preload="metadata"
			></video>
		{/if}
	</figure>
{/if}

<style lang="scss">
	figure {
		margin: 1.5rem 0;
	}

	video {
		display: block;
		width: 100%;
		border-radius: 0.3rem;
	}
</style>
