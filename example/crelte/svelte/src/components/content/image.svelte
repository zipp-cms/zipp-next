<script>
	import Img from '@/components/Img.svelte';

	let { image, caption } = $props();
</script>

<figure>
	{#each image as img (img.url)}
		<Img {img} />
	{/each}
	{#if caption}<figcaption>{caption}</figcaption>{/if}
</figure>

<style>
	figure {
		margin: 1.5rem 0;
	}

	figcaption {
		margin-top: 0.4rem;
		color: #666;
		font-size: 0.9rem;
	}
</style>
