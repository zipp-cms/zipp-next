<script lang="ts">
	import { can } from '../lib/access';
	import { listEntries, moveEntry, type Entry, type Schema } from '../api';
	import { entryPath, go, shown, site } from '../route.svelte';
	import Icon from '../lib/Icon.svelte';

	let { schema, section }: { schema: Schema; section: string } = $props();

	const sec = $derived(schema.sections.find(s => s.handle === section)!);
	const tree = $derived(sec.kind === 'structure');
	// moving changes the website at once, like publishing
	let search = $state('');
	// not while searching, the results are no tree
	const canMove = $derived(
		tree && !search && can(schema, section, 'publish'),
	);
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
					go(entryPath(section, r.entries[0].id), true);
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

	// drag and drop in a structure: before or after a row, or into it
	let dragged = $state<Entry | null>(null);
	let drop = $state<{
		id: number;
		where: 'before' | 'after' | 'into';
	} | null>(null);

	/** An entry can't go below itself. */
	function below(e: Entry, ancestor: Entry): boolean {
		if (e.parentId === null) return false;
		if (e.parentId === ancestor.id) return true;
		const parent = entries.find(p => p.id === e.parentId);
		return parent ? below(parent, ancestor) : false;
	}

	function ondragover(ev: DragEvent, e: Entry) {
		if (!dragged || e === dragged || below(e, dragged)) return;
		ev.preventDefault();
		const r = (ev.currentTarget as HTMLElement).getBoundingClientRect();
		const y = (ev.clientY - r.top) / r.height;
		const where = y < 0.3 ? 'before' : y > 0.7 ? 'after' : 'into';
		drop = { id: e.id, where };
	}

	async function ondrop(ev: DragEvent) {
		ev.preventDefault();
		const d = dragged;
		const target = entries.find(e => e.id === drop?.id);
		const where = drop?.where;
		dragged = null;
		drop = null;
		if (!d || !target) return;

		try {
			if (where === 'into') {
				await moveEntry(d.id, target.id, 1e6);
			} else {
				// the position among the new siblings, without the dragged one
				const sibs = siblings(target).filter(s => s !== d);
				const at = sibs.indexOf(target) + (where === 'after' ? 1 : 0);
				await moveEntry(d.id, target.parentId, at);
			}
			load();
		} catch (err: any) {
			error = err.message;
		}
	}

	/** Published in how many sites, for the status next to the current one. */
	function publishedCount(e: Entry) {
		return Object.values(published[e.id] ?? {}).filter(Boolean).length;
	}

	function date(ms: number) {
		return new Date(ms).toLocaleDateString();
	}
</script>

<div class="page-top">
	<h1>{sec.name}</h1>
	{#if sec.kind !== 'single' && can(schema, section, 'edit')}
		<button class="dark" onclick={() => go(`/entries/${section}/new`)}>
			<Icon name="add" />
			New entry
		</button>
	{/if}
</div>

{#if error}<p class="error">{error}</p>{/if}

<div class="card list">
	<div class="search">
		<input placeholder="Search entries" bind:value={search} />
		<Icon name="search" />
	</div>
	{#each entries as e (e.id)}
		{@const on = published[e.id]?.[site.current]}
		<div
			class="row"
			class:dragging={dragged === e}
			class:drop-before={drop?.id === e.id && drop.where === 'before'}
			class:drop-after={drop?.id === e.id && drop.where === 'after'}
			class:drop-into={drop?.id === e.id && drop.where === 'into'}
			ondragover={ev => canMove && ondragover(ev, e)}
			ondragleave={() => drop?.id === e.id && (drop = null)}
			{ondrop}
			role="listitem"
		>
			<span class="name" style:padding-left="{depth(e) * 1.5}rem">
				{#if canMove}
					<span
						class="handle"
						draggable="true"
						title="Drag to move"
						role="button"
						tabindex="-1"
						ondragstart={ev => {
							dragged = e;
							ev.dataTransfer?.setData(
								'text/plain',
								String(e.id),
							);
						}}
						ondragend={() => {
							dragged = null;
							drop = null;
						}}
					>
						<Icon name="move" />
					</span>
				{/if}
				<a class="title" href="/admin{entryPath(section, e.id)}">
					{e.title}
				</a>
			</span>
			<span class="status">
				<span
					class="badge"
					class:draft={!on}
					title="{site.current.toUpperCase()}: {on
						? 'published'
						: 'not published'}"
				>
					{site.current.toUpperCase()}
				</span>
				{#if schema.sites.length > 1}
					<span
						class="count"
						title="Published in {publishedCount(e)} of {schema.sites
							.length} sites"
					>
						{publishedCount(e)}/{schema.sites.length}
					</span>
				{/if}
			</span>
			<span class="type hide-small">
				Type: {schema.components.find(c => c.handle === e.type)?.name}
			</span>
			<span class="type hide-small">{date(e.updatedAt)}</span>
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
		grid-template-columns: minmax(0, 1fr) 7rem 12rem 7rem;
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

	.status {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.count {
		font-size: 13px;
		color: var(--muted);
	}

	.name {
		display: flex;
		align-items: center;
		gap: 8px;
		min-width: 0;
	}

	.handle {
		display: flex;
		cursor: grab;
		color: var(--muted);
	}

	.dragging {
		opacity: 0.4;
	}

	.drop-before {
		box-shadow: inset 0 2px 0 var(--dark);
	}

	.drop-after {
		box-shadow: inset 0 -2px 0 var(--dark);
	}

	.drop-into {
		background: var(--hover);
	}

	.type {
		font-size: 14px;
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
