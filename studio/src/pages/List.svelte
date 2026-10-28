<script lang="ts">
	import { listEntries, type Entry, type Schema } from '../api';

	let { schema, type }: { schema: Schema; type: string } = $props();

	const comp = $derived(schema.components.find(c => c.handle === type)!);
	let search = $state('');
	let entries = $state<Entry[]>([]);
	let total = $state(0);
	let error = $state('');

	$effect(() => {
		const params: Record<string, string> = { type, limit: '50' };
		if (search) params.search = search;
		listEntries(params)
			.then(r => {
				entries = r.entries;
				total = r.total;
			})
			.catch(e => (error = e.message));
	});

	function date(ms: number) {
		return new Date(ms).toLocaleDateString();
	}
</script>

<header>
	<h2>{comp.name}</h2>
	<a href="#/new/{type}"><button class="primary">New {comp.name}</button></a>
</header>

<input placeholder="Search titles" bind:value={search} />

{#if error}<p class="error">{error}</p>{/if}

<table>
	<tbody>
		{#each entries as e (e.id)}
			<tr>
				<td><a href="#/entry/{e.id}">{e.title}</a></td>
				<td class="muted">/{e.slug}</td>
				<td class="muted">{e.published ? 'Published' : 'Draft'}</td>
				<td class="muted">{date(e.updatedAt)}</td>
			</tr>
		{:else}
			<tr><td class="muted">No entries yet</td></tr>
		{/each}
	</tbody>
</table>
{#if total > entries.length}
	<p class="muted">{entries.length} of {total}</p>
{/if}

<style>
	header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	table {
		width: 100%;
		margin-top: 1rem;
		border-collapse: collapse;
	}

	td {
		padding: 0.6rem 0.5rem;
		border-bottom: 1px solid var(--line);
	}

	a {
		color: inherit;
	}

	.muted {
		color: var(--muted);
	}
</style>
