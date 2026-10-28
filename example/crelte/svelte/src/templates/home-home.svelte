<script module>
	import Content, {
		loadData as loadContentData,
	} from '@/components/Content.svelte';

	/** @type {import('crelte').LoadData} */
	export const loadData = {
		blocks: (cr, entry) => loadContentData(cr, entry.body),
	};
</script>

<script>
	import Img from '@/components/Img.svelte';

	let { entry, blocks } = $props();
</script>

{#each entry.hero as img (img.url)}
	<Img {img} />
{/each}
<h1>{entry.title}</h1>
{#if entry.intro}<p class="intro">{entry.intro}</p>{/if}
<Content {blocks} />
