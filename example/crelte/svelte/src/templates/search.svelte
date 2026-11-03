<script module>
	import searchQuery from '@/queries/search.graphql';

	const PER_PAGE = 10;

	// `?q=konzert&page=2`; without words nothing is searched
	/** @type {import('crelte').LoadData} */
	export const loadData = async cr => {
		const q = (cr.req.getSearchParam('q') ?? '').trim();
		const page = Math.max(1, Number(cr.req.getSearchParam('page')) || 1);
		if (!q) return { q, page, hits: [], total: 0 };
		const data = await cr.query(searchQuery, {
			query: q,
			offset: (page - 1) * PER_PAGE,
		});
		return { q, page, .../** @type {object} */ (data) };
	};

	// the texts of the page; zipp's translations are part of the content
	// model, the search page isn't
	const TEXTS = {
		de: {
			label: 'Suchbegriff',
			submit: 'Suchen',
			empty: 'Gib ein Wort ein, zum Beispiel «Konzert».',
			none: 'Nichts gefunden für «{q}».',
			found: '{n} Treffer für «{q}»',
			prev: 'Zurück',
			next: 'Weiter',
		},
		en: {
			label: 'Search term',
			submit: 'Search',
			empty: 'Type a word, for example “concert”.',
			none: 'Nothing found for “{q}”.',
			found: '{n} results for “{q}”',
			prev: 'Previous',
			next: 'Next',
		},
	};
</script>

<script>
	import { getSite } from 'crelte';
	import { searchTitle, searchUrl } from '@/search.js';

	const site = getSite();

	let { q, page, hits, total } = $props();

	let t = $derived(TEXTS[$site.language.startsWith('de') ? 'de' : 'en']);
	let pages = $derived(Math.ceil(total / PER_PAGE));

	/** @param {string} text */
	const fill = text => text.replace('{q}', q).replace('{n}', String(total));

	/** @param {number} n */
	const pageUrl = n =>
		`${searchUrl($site)}?q=${encodeURIComponent(q)}&page=${n}`;
</script>

<h1>{searchTitle($site)}</h1>

<form action={searchUrl($site)} role="search">
	<input
		type="search"
		name="q"
		value={q}
		aria-label={t.label}
		placeholder={t.label}
	/>
	<button>{t.submit}</button>
</form>

{#if !q}
	<p class="meta">{t.empty}</p>
{:else if !total}
	<p class="meta">{fill(t.none)}</p>
{:else}
	<p class="meta">{fill(t.found)}</p>
	<ol class="hits">
		{#each hits as hit (hit.entry.url)}
			<li>
				<h2><a href={hit.entry.url}>{hit.entry.title}</a></h2>
				<!-- zipp escapes the texts, only the marks are html -->
				{#if hit.excerptHtml}<p>{@html hit.excerptHtml}</p>{/if}
			</li>
		{/each}
	</ol>
	{#if pages > 1}
		<nav class="pages">
			{#if page > 1}<a href={pageUrl(page - 1)}>{t.prev}</a>{/if}
			<span>{page} / {pages}</span>
			{#if page < pages}<a href={pageUrl(page + 1)}>{t.next}</a>{/if}
		</nav>
	{/if}
{/if}

<style lang="scss">
	form {
		display: flex;
		gap: 0.5rem;
		margin: 1.5rem 0;
	}

	input {
		flex: 1;
		padding: 0.5rem 0.75rem;
		border: 1px solid #ccc;
		border-radius: 0.3rem;
		font: inherit;
	}

	button {
		padding: 0.5rem 1rem;
		border: 0;
		border-radius: 0.3rem;
		background: #2f5bea;
		color: white;
		font: inherit;
	}

	.hits {
		padding: 0;
		list-style: none;
	}

	li {
		margin-bottom: 1.5rem;
	}

	h2 {
		margin: 0 0 0.25rem;
	}

	p :global(mark) {
		background: #fff3a8;
	}

	.pages {
		display: flex;
		gap: 1rem;
	}

	.meta {
		color: #666;
	}
</style>
