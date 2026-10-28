<script lang="ts">
	import { entries, sites, uri, url, type Refs } from './api';
	import type { AnyEntry } from './content';
	import Blocks from './Blocks.svelte';

	let entry = $state<AnyEntry | null>(null);
	let refs = $state<Refs>({});
	let missing = $state(false);
	// the same entry in the other sites, for the language switch
	let others = $state<Record<string, string>>({});

	entries<AnyEntry>({ uri }).then(async r => {
		entry = r.entries[0] ?? null;
		refs = r.relations;
		missing = !entry;
		if (!entry) return;

		for (const s of sites) {
			const other = await entries({
				ids: String(entry.id),
				site: s.handle,
			});
			if (other.entries[0])
				others[s.handle] = url(other.entries[0], s.handle);
		}
	});
</script>

<nav>
	{#each sites as s (s.handle)}
		<a href={others[s.handle] ?? s.prefix + '/'}>{s.label}</a>
	{/each}
</nav>

<main>
	{#if missing}
		<h1>Not found</h1>
	{:else if entry?.type === 'home' || entry?.type === 'page'}
		<h1>{entry.title}</h1>
		{#if entry.fields.intro}<p class="intro">{entry.fields.intro}</p>{/if}
		<Blocks blocks={entry.fields.body ?? []} {refs} />
	{:else if entry?.type === 'event'}
		{@const f = entry.fields}
		<h1>{entry.title}</h1>
		<p>{f.date} in {f.city}{f.price ? ` · CHF ${f.price}` : ''}</p>
		{#if f.description}<p>{f.description}</p>{/if}
		{#each f.page ?? [] as id (id)}
			{#if refs[id]}<a href={url(refs[id])}>{refs[id].title}</a>{/if}
		{/each}
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
