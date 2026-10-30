<script module>
	import articlesQuery from '@/queries/articles.graphql';

	// like the crelte tutorial: the category comes from `?category=<id>`
	/** @type {import('crelte').LoadData} */
	export const loadData = cr =>
		cr.query(articlesQuery, {
			categories: cr.req.getSearchParam('category'),
		});
</script>

<script>
	import { getRoute, getSite } from 'crelte';
	import { getTranslations } from 'crelte/translations';
	import Intro from '@/components/Intro.svelte';

	const route = getRoute();
	const site = getSite();
	const t = getTranslations();

	let { entry, articles, categories } = $props();

	let active = $derived($route.getSearchParam('category'));

	/** @param {string} date */
	const day = date => new Date(date).toLocaleDateString($site.language);
</script>

<Intro {entry} />

<!-- a filter keeps the scroll position -->
<nav class="categories">
	<a href={entry.url} class:active={!active} data-disable-scroll>
		{$t('news.all')}
	</a>
	{#each categories as c (c.id)}
		<a
			href="{entry.url}?category={c.id}"
			class:active={active === c.id}
			data-disable-scroll
		>
			{c.title}
		</a>
	{/each}
</nav>

<div class="articles">
	{#each articles as a (a.url)}
		<article>
			{#each a.hero as img (img.url)}
				<img src={img.url} alt={img.alt} />
			{/each}
			<div>
				<h2><a href={a.url}>{a.title}</a></h2>
				<p class="meta">
					{[day(a.postDate), ...a.categories.map(c => c.title)].join(
						' · ',
					)}
				</p>
				{#if a.intro?.html}{@html a.intro.html}{/if}
			</div>
		</article>
	{:else}
		<p class="meta">{$t('news.none')}</p>
	{/each}
</div>

<style lang="scss">
	.categories {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin: 1.5rem 0;

		a {
			padding: 0.25rem 0.75rem;
			border: 1px solid #ccc;
			border-radius: 1rem;
			text-decoration: none;
		}

		.active {
			border-color: #2f5bea;
			background: #2f5bea;
			color: white;
		}
	}

	article {
		display: flex;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	img {
		width: 160px;
		height: 120px;
		flex-shrink: 0;
		border-radius: 0.3rem;
	}

	h2 {
		margin: 0 0 0.25rem;
	}

	.meta {
		color: #666;
	}
</style>
