<script lang="ts">
	import { listEntries, moveEntry, type Entry, type Schema } from '../api';
	import { go, site } from '../route.svelte';

	let { schema, section }: { schema: Schema; section: string } = $props();

	const sec = $derived(schema.sections.find(s => s.handle === section)!);
	const tree = $derived(sec.kind === 'structure');
	let search = $state('');
	let entries = $state<Entry[]>([]);
	let total = $state(0);
	let error = $state('');

	function load() {
		const params: Record<string, string> = {
			section,
			site: site.current,
			limit: '500',
		};
		if (tree) params.order = 'structure';
		if (search) params.search = search;
		listEntries(params)
			.then(r => {
				// a single has no list, it opens its entry
				if (sec.kind === 'single' && r.entries[0]) {
					location.replace('#/entry/' + r.entries[0].id);
					return;
				}
				entries = r.entries;
				total = r.total;
			})
			.catch(e => (error = e.message));
	}

	$effect(load);

	/** How deep an entry is in the tree, from the parents in the list. */
	function depth(e: Entry): number {
		const parent = entries.find(p => p.id === e.parentId);
		return parent ? depth(parent) + 1 : 0;
	}

	function siblings(e: Entry) {
		return entries.filter(s => s.parentId === e.parentId);
	}

	async function move(e: Entry, how: 'up' | 'down' | 'in' | 'out') {
		const sibs = siblings(e);
		const i = sibs.indexOf(e);
		try {
			if (how === 'up') await moveEntry(e.id, e.parentId, i - 1);
			if (how === 'down') await moveEntry(e.id, e.parentId, i + 1);
			// into the sibling above, as its last child
			if (how === 'in') await moveEntry(e.id, sibs[i - 1].id, 1e6);
			// after its parent
			if (how === 'out') {
				const parent = entries.find(p => p.id === e.parentId)!;
				const at = siblings(parent).indexOf(parent);
				await moveEntry(e.id, parent.parentId, at + 1);
			}
			load();
		} catch (err: any) {
			error = err.message;
		}
	}

	function date(ms: number) {
		return new Date(ms).toLocaleDateString();
	}
</script>

<header>
	<h2>{sec.name}</h2>
	{#if sec.kind !== 'single'}
		<button class="primary" onclick={() => go('#/new/' + section)}>
			New entry
		</button>
	{/if}
</header>

{#if !tree}
	<input placeholder="Search titles" bind:value={search} />
{/if}

{#if error}<p class="error">{error}</p>{/if}

<table>
	<tbody>
		{#each entries as e (e.id)}
			{@const sibs = siblings(e)}
			{@const i = sibs.indexOf(e)}
			<tr>
				<td style:padding-left="{0.5 + depth(e) * 1.5}rem">
					<a href="#/entry/{e.id}">{e.title}</a>
				</td>
				<td class="muted hide-small">
					{e.uri === null ? '' : '/' + e.uri}
				</td>
				<td class="muted">{e.published ? 'Published' : 'Draft'}</td>
				{#if tree}
					<td class="moves">
						<button
							class="icon"
							title="Up"
							disabled={i === 0}
							onclick={() => move(e, 'up')}
						>
							↑
						</button>
						<button
							class="icon"
							title="Down"
							disabled={i === sibs.length - 1}
							onclick={() => move(e, 'down')}
						>
							↓
						</button>
						<button
							class="icon"
							title="Into the entry above"
							disabled={i === 0}
							onclick={() => move(e, 'in')}
						>
							→
						</button>
						<button
							class="icon"
							title="Out of its parent"
							disabled={e.parentId === null}
							onclick={() => move(e, 'out')}
						>
							←
						</button>
					</td>
				{:else}
					<td class="muted hide-small">{date(e.updatedAt)}</td>
				{/if}
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

	.moves {
		white-space: nowrap;
		text-align: right;
	}

	a {
		color: inherit;
	}

	.muted {
		color: var(--muted);
	}

	@media (max-width: 700px) {
		.hide-small {
			display: none;
		}
	}
</style>
