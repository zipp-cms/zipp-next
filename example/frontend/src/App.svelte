<script lang="ts">
	import { gql, sites, uri, url, type Entry } from './api';
	import Blocks from './Blocks.svelte';

	// everything the page shows in one request, linked entries included
	const PAGE = `
		fragment Body on Block {
			id
			typeHandle
			... on heading_Block { text }
			... on text_Block { content }
			... on button_Block { label link { title uri } }
			... on events_Block { city }
		}

		query ($uri: String, $site: String) {
			entry(uri: $uri, site: $site) {
				id
				typeHandle
				title
				... on home_Entry { intro body { ...Body } }
				... on page_Entry { intro body { ...Body } }
				... on event_Entry {
					date city price description
					page { title uri }
				}
			}
		}
	`;

	// the same entry in every site, for the language switch
	const SITES = `
		query ($id: Int) {
			${sites.map(s => `${s.handle}: entry(id: $id, site: "${s.handle}") { uri }`).join('\n')}
		}
	`;

	let entry = $state<Entry | null>(null);
	let missing = $state(false);
	let others = $state<Record<string, { uri: string | null } | null>>({});

	gql<{ entry: Entry | null }>(PAGE, { uri }).then(async r => {
		entry = r.entry;
		missing = !entry;
		if (entry) others = await gql(SITES, { id: entry.id });
	});
</script>

<nav>
	{#each sites as s (s.handle)}
		{@const other = others[s.handle]}
		<a href={other ? url(other, s.handle) : s.prefix + '/'}>{s.label}</a>
	{/each}
</nav>

<main>
	{#if missing}
		<h1>Not found</h1>
	{:else if entry?.typeHandle === 'event'}
		<h1>{entry.title}</h1>
		<p>
			{entry.date} in {entry.city}{entry.price
				? ` · CHF ${entry.price}`
				: ''}
		</p>
		{#if entry.description}<p>{entry.description}</p>{/if}
		{#each entry.page ?? [] as p (p.uri)}
			<a href={url(p)}>{p.title}</a>
		{/each}
	{:else if entry}
		<h1>{entry.title}</h1>
		{#if entry.intro}<p class="intro">{entry.intro}</p>{/if}
		<Blocks blocks={entry.body ?? []} />
	{/if}
</main>

<style>
	nav {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		max-width: 40rem;
		margin: 1.5rem auto 0;
		font-family: system-ui, sans-serif;
	}

	main {
		max-width: 40rem;
		margin: 2rem auto;
		font-family: Georgia, serif;
	}

	.intro {
		font-size: 1.2rem;
	}
</style>
