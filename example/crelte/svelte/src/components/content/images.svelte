<script>
	import Img from '@/components/Img.svelte';

	let { images, caption, width } = $props();
</script>

<!-- one image, or a gallery of several -->
<figure class={width ?? 'wide'} class:gallery={images.length > 1}>
	{#each images as img (img.url)}
		<Img {img} />
	{/each}
	{#if caption}<figcaption>{caption}</figcaption>{/if}
</figure>

<style lang="scss">
	figure {
		margin: 1.5rem 0;
	}

	/* the width the editor picked with the dropdown */
	.small {
		max-width: 50%;
	}

	.full {
		margin-inline: calc(50% - 50vw);
	}

	.full figcaption {
		padding-inline: calc(50vw - 50%);
	}

	.gallery {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(12rem, 1fr));
		gap: 0.5rem;

		figcaption {
			grid-column: 1 / -1;
		}
	}

	figcaption {
		margin-top: 0.4rem;
		color: #666;
		font-size: 0.9rem;
	}
</style>
