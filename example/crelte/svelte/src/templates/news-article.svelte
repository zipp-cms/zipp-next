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
	import { getSite } from 'crelte';
	import Intro from '@/components/Intro.svelte';

	const site = getSite();

	let { entry, blocks } = $props();
</script>

<p class="meta">
	{[
		new Date(entry.postDate).toLocaleDateString($site.language),
		...entry.categories.map(c => c.title),
	].join(' · ')}
</p>
<Intro {entry} />
<Content {blocks} />

<style>
	.meta {
		margin-top: 1.5rem;
		color: #666;
	}
</style>
