<script lang="ts">
	import type { Image } from './api';

	let { img, sizes }: { img: Image; sizes: string } = $props();
</script>

<img
	src={img.url}
	srcset={img.srcset}
	{sizes}
	width={img.width}
	height={img.height}
	alt={img.alt}
/>

<style>
	img {
		display: block;
		width: 100%;
		height: auto;
		border-radius: 0.3rem;
	}
</style>
