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

<p class="meta">
	{[entry.postDate, ...entry.categories.map(c => c.title)].join(' · ')}
</p>
<Intro {entry} />
<Content {blocks} />

<style>
	.meta {
		margin-top: 1.5rem;
		color: #666;
	}
</style>
