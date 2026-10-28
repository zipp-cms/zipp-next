<script lang="ts">
	import { listEntries, moveEntry, type Entry, type Schema } from '../api';
	import { go, shown, site } from '../route.svelte';
	import Icon from '../lib/Icon.svelte';

	let { schema, section }: { schema: Schema; section: string } = $props();

	const sec = $derived(schema.sections.find(s => s.handle === section)!);
	const tree = $derived(sec.kind === 'structure');
	let search = $state('');
	let entries = $state<Entry[]>([]);
	let published = $state<Record<number, Record<string, boolean>>>({});

	$effect(() => {
		shown.section = section;
	});
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
				published = r.published;
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

<div class="page-top">
	<h1>{sec.name}</h1>
	{#if sec.kind !== 'single'}
		<button class="dark" onclick={() => go('#/new/' + section)}>
			<Icon name="add" />
			New entry
		</button>
	{/if}
</div>

{#if error}<p class="error">{error}</p>{/if}

<div class="card list">
	{#if !tree}
		<div class="search">
			<input placeholder="Search entries" bind:value={search} />
			<Icon name="search" />
		</div>
	{/if}

	{#each entries as e (e.id)}
		{@const sibs = siblings(e)}
		{@const i = sibs.indexOf(e)}
		<div class="row">
			<a
				class="title"
				href="#/entry/{e.id}"
				style:padding-left="{depth(e) * 1.5}rem"
			>
				{e.title}
			</a>
			<span class="badges">
				{#each schema.sites as st (st.handle)}
					{@const on = published[e.id]?.[st.handle]}
					<span
						class="badge"
						class:draft={!on}
						class:current={st.handle === site.current}
						title="{st.name}: {on ? 'published' : 'draft'}"
					>
						{st.handle.toUpperCase()}
					</span>
				{/each}
			</span>
			<span class="type hide-small">
				Type: {schema.components.find(c => c.handle === e.type)?.name}
			</span>
			{#if tree}
				<span class="moves">
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
				</span>
			{:else}
				<span class="type hide-small">{date(e.updatedAt)}</span>
			{/if}
		</div>
	{:else}
		<p class="empty">No entries yet</p>
	{/each}
	{#if total > entries.length}
		<p class="empty">{entries.length} of {total}</p>
	{/if}
</div>

<style lang="scss">
	.list {
		padding: 0 0 8px;
	}

	.search {
		position: relative;
		display: flex;
		justify-content: flex-end;
		padding: 10px 16px;
		border-bottom: 2px solid var(--line);

		input {
			width: 340px;
			padding-right: 36px;
		}

		:global(.icon) {
			position: absolute;
			right: 28px;
			top: 50%;
			transform: translateY(-50%);
		}
	}

	.row {
		display: grid;
		grid-template-columns: minmax(0, 1fr) 7rem 12rem 9rem;
		align-items: center;
		padding: 10px 26px;
	}

	.title {
		font-size: 17px;
		color: inherit;
		text-decoration: none;

		&:hover {
			text-decoration: underline;
		}
	}

	.badges {
		display: flex;
		gap: 4px;
	}

	.badge {
		padding: 2px 6px;
		font-size: 13px;
		color: white;
		background: var(--dark);
		border-radius: 2px;

		&.draft {
			background: #c4c4c8;
		}

		&.current {
			outline: 2px solid var(--dark);
			outline-offset: 1px;
		}
	}

	.type {
		font-size: 14px;
	}

	.moves {
		justify-self: end;
		white-space: nowrap;
	}

	.empty {
		padding: 10px 26px;
		color: var(--muted);
	}

	@media (max-width: 800px) {
		.row {
			grid-template-columns: minmax(0, 1fr) auto auto;
			padding: 10px 16px;
		}

		.hide-small {
			display: none;
		}

		.search input {
			width: 100%;
		}
	}
</style>
