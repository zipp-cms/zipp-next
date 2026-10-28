<script>
	import Img from '@/components/Img.svelte';

	let { image, caption, width } = $props();
</script>

<figure class={width ?? 'wide'}>
	{#each image as img (img.url)}
		<Img {img} />
	{/each}
	{#if caption}<figcaption>{caption}</figcaption>{/if}
</figure>

<style>
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

	figcaption {
		margin-top: 0.4rem;
		color: #666;
		font-size: 0.9rem;
	}
</style>
