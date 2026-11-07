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
{#if entry.author?.name}
	<p class="author">
		<strong>{entry.author.name}</strong>
		{#if entry.author.bio}<br />{entry.author.bio}{/if}
	</p>
{/if}

<style>
	.meta {
		margin-top: 1.5rem;
		color: #666;
	}

	.author {
		margin-top: 2rem;
		padding-top: 1rem;
		border-top: 1px solid #ddd;
		color: #666;
	}
</style>
