<script>
	import Img from '@/components/Img.svelte';

	// the start of a page or an article: its image, title and intro, the
	// Intro tab in the studio
	let { entry } = $props();
</script>

{#each entry.hero as img (img.url)}
	<Img {img} />
{/each}
<h1>{entry.title}</h1>
{#if entry.intro?.html}
	<div class="intro">{@html entry.intro.html}</div>
{/if}
