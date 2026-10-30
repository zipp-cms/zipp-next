<script module>
	import Content, {
		loadData as loadContentData,
	} from '@/components/Content.svelte';

	/** @type {import('crelte').LoadData} */
	export const loadData = {
		blocks: (cr, entry) => loadContentData(cr, entry.content),
	};
</script>

<script>
	import Intro from '@/components/Intro.svelte';

	let { entry, blocks } = $props();
</script>

<Intro {entry} />
<Content {blocks} />

{#if entry.children.length}
	<ul class="children">
		{#each entry.children as child (child.url)}
			<li><a href={child.url}>{child.title}</a></li>
		{/each}
	</ul>
{/if}
