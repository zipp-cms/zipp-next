<script lang="ts">
	import { can, sectionCrumbs } from '../lib/access';
	import Crumbs from '../lib/Crumbs.svelte';
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
	// the entries with unpublished changes
	let drafts = $state<number[]>([]);
	// the type is worth a column only with several
	const types = $derived(sec.types.length > 1);

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
				drafts = r.drafts;
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

	function date(ms: number) {
		return new Date(ms).toLocaleDateString();
	}
</script>

<div class="page-top">
	<h1><Crumbs items={sectionCrumbs(schema, section)} /></h1>
	{#if sec.kind !== 'single' && can(schema, section, 'edit')}
		<button class="primary" onclick={() => go(`/entries/${section}/new`)}>
			<Icon name="add" />
			New entry
		</button>
	{/if}
</div>

{#if error}<p class="error">{error}</p>{/if}

<div class="card list">
	<div class="search">
		<span class="field">
			<Icon name="search" size={15} />
			<input placeholder="Search entries" bind:value={search} />
		</span>
		<span class="total">{total} {total === 1 ? 'entry' : 'entries'}</span>
	</div>
	{#if entries.length}
		<div class="row columns" class:types>
			<span>Title</span>
			<span>Status</span>
			{#if types}<span class="hide-small">Type</span>{/if}
			<span class="hide-small">Updated</span>
		</div>
	{/if}
	{#each entries as e (e.id)}
		<div
			class="row"
			class:types
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
						<Icon name="move" size={15} />
					</span>
				{/if}
				<a class="title" href="/admin{entryPath(section, e.id)}">
					{e.title}
				</a>
			</span>
			<span class="status">
				<!-- live or not in every site, the current one first -->
				{#each schema.sites as s (s.handle)}
					{@const on = published[e.id]?.[s.handle]}
					<span
						class="badge"
						class:draft={!on}
						class:current={s.handle === site.current}
						title="{s.name}: {on ? 'published' : 'not published'}"
					>
						{s.handle.toUpperCase()}
					</span>
				{/each}
				{#if drafts.includes(e.id)}
					<span class="badge changes" title="Unpublished changes">
						Draft
					</span>
				{/if}
			</span>
			{#if types}
				<span class="type hide-small">
					{schema.components.find(c => c.handle === e.type)?.name}
				</span>
			{/if}
			<span class="date hide-small">{date(e.updatedAt)}</span>
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
		padding: 0 0 6px;
		overflow: hidden;
	}

	.search {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 10px 14px;

		.field {
			position: relative;
			display: flex;
			align-items: center;
			width: 320px;
			color: var(--subtle);

			:global(.icon) {
				position: absolute;
				left: 10px;
			}
		}

		input {
			padding-left: 32px;
		}

		.total {
			margin-left: auto;
			color: var(--muted);
			font-size: 12px;
		}
	}

	.row {
		display: grid;
		grid-template-columns: minmax(0, 1fr) 13rem 7rem;
		align-items: center;
		gap: 12px;
		min-height: 44px;
		padding: 4px 16px;
		border-top: 1px solid var(--line);

		&.types {
			grid-template-columns: minmax(0, 1fr) 13rem 10rem 7rem;
		}

		&:not(.columns):hover {
			background: var(--surface-2);
		}
	}

	.columns {
		min-height: 32px;
		color: var(--muted);
		font-size: 11px;
		font-weight: 600;
		letter-spacing: 0.04em;
		text-transform: uppercase;
		background: var(--surface-2);
	}

	.title {
		color: var(--text);
		font-weight: 500;
		text-decoration: none;

		&:hover {
			color: var(--accent);
		}
	}

	.status {
		display: flex;
		align-items: center;
		gap: 4px;
		flex-wrap: wrap;

		.current {
			outline: 1px solid currentColor;
			outline-offset: -1px;
		}
	}

	.type,
	.date {
		color: var(--muted);
		font-size: 13px;
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
		color: var(--subtle);
		opacity: 0;
	}

	.row:hover .handle {
		opacity: 1;
	}

	.dragging {
		opacity: 0.4;
	}

	.drop-before {
		box-shadow: inset 0 2px 0 var(--accent);
	}

	.drop-after {
		box-shadow: inset 0 -2px 0 var(--accent);
	}

	.drop-into {
		background: var(--accent-soft);
	}

	.type {
		font-size: 14px;
	}

	.empty {
		padding: 10px 26px;
		color: var(--muted);
	}

	@media (max-width: 800px) {
		.row,
		.row.types {
			grid-template-columns: minmax(0, 1fr) auto;
		}

		.hide-small {
			display: none;
		}

		.search .field {
			width: 100%;
		}
	}
</style>
