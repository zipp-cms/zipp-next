<script lang="ts">
	import { entries, url, type Refs } from './api';
	import type { Event as EventEntry, Page } from './content';
	import Blocks from './Blocks.svelte';

	// `/` is the page `home`, `/events/<slug>` an event, `/<slug>` a page
	const [first, second] = location.pathname.split('/').filter(Boolean);
	const params =
		first === 'events'
			? { type: 'event', slug: second ?? '' }
			: { type: 'page', slug: first ?? 'home' };

	let entry = $state<Page | EventEntry | null>(null);
	let refs = $state<Refs>({});
	let missing = $state(false);

	entries<Page | EventEntry>(params).then(r => {
		entry = r.entries[0] ?? null;
		refs = r.relations;
		missing = !entry;
	});
</script>

<main>
	{#if missing}
		<h1>Not found</h1>
	{:else if entry?.type === 'page'}
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
	main {
		max-width: 40rem;
		margin: 3rem auto;
		font-family: Georgia, serif;
	}

	.intro {
		font-size: 1.2rem;
	}
</style>
