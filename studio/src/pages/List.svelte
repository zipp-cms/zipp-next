<script lang="ts">
	import SearchInput from '../lib/SearchInput.svelte';
	import { plural } from '../lib/list';
	import SiteBadges from '../lib/SiteBadges.svelte';
	import PageTop from '../lib/PageTop.svelte';
	import { can, sectionCrumbs } from '../lib/access';
	import {
		deleteEntry,
		entryUrl,
		listEntries,
		moveEntry,
		type Entry,
		type Schema,
	} from '../api';
	import Menu, { type MenuItem } from '../lib/Menu.svelte';
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
	let loaded = $state(false);

	function load() {
		const params: Record<string, string> = {
			section,
			site: site.current,
			limit: '500',
		};
		if (tree) params.order = 'structure';
		const q = search;
		if (q) params.search = q;
		listEntries(params)
			.then(r => {
				// an answer to an older search
				if (q !== search) return;
				// a single has no list, it opens its entry
				if (sec.kind === 'single' && r.entries[0]) {
					go(entryPath(section, r.entries[0].id), true);
					return;
				}
				entries = r.entries;
				total = r.total;
				published = r.published;
				drafts = r.drafts;
				loaded = true;
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
	function isBelow(e: Entry, ancestor: Entry): boolean {
		if (e.parentId === null) return false;
		if (e.parentId === ancestor.id) return true;
		const parent = entries.find(p => p.id === e.parentId);
		return parent ? isBelow(parent, ancestor) : false;
	}

	function ondragover(ev: DragEvent, e: Entry) {
		if (!dragged || e === dragged || isBelow(e, dragged)) return;
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

	/** Edit, view and delete without opening the entry. */
	function actions(e: Entry): MenuItem[] {
		const items: MenuItem[] = [
			{
				label: 'Edit',
				icon: 'edit',
				href: '/admin' + entryPath(section, e.id),
			},
		];
		const url = entryUrl(schema, site.current, e.uri);
		if (url && published[e.id]?.[site.current])
			items.push({
				label: 'View on the website',
				icon: 'external',
				href: url,
			});
		if (sec.kind !== 'single' && can(schema, section, 'delete'))
			items.push({
				label: 'Delete',
				icon: 'delete',
				danger: true,
				onclick: () => remove(e),
			});
		return items;
	}

	async function remove(e: Entry) {
		// the entries below it go with it
		const below = entries.filter(c => c !== e && isBelow(c, e)).length;
		const also = below
			? ` The ${below === 1 ? 'entry' : `${below} entries`} below it too.`
			: '';
		if (!confirm(`Delete ${e.title} in all sites?${also}`)) return;
		try {
			await deleteEntry(e.id);
			load();
		} catch (err: any) {
			error = err.message;
		}
	}

	function date(ms: number) {
		return new Date(ms).toLocaleDateString();
	}
</script>

<PageTop crumbs={sectionCrumbs(schema, section)}>
	{#snippet actions()}
		{#if sec.kind !== 'single' && can(schema, section, 'edit')}
			<button
				class="primary"
				onclick={() => go(`/entries/${section}/new`)}
			>
				<Icon name="add" />
				New entry
			</button>
		{/if}
	{/snippet}
</PageTop>

{#if error}<p class="error">{error}</p>{/if}

<div class="card list">
	<div class="search">
		<span class="field">
			<SearchInput bind:value={search} placeholder="Search entries" />
		</span>
		<span class="note total">{plural(total, 'entry', 'entries')}</span>
	</div>
	{#if entries.length}
		<div class="row columns" class:types>
			<span>Title</span>
			<span>Status</span>
			{#if types}<span class="hide-small">Type</span>{/if}
			<span class="hide-small">Updated</span>
			<span></span>
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
				<SiteBadges {schema} published={published[e.id]} />
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
			<Menu items={actions(e)} label="Actions for {e.title}" />
		</div>
	{:else}
		<p class="empty">
			{loaded
				? search
					? 'Nothing found'
					: 'No entries yet'
				: 'Loading…'}
		</p>
	{/each}
	{#if total > entries.length}
		<p class="empty">{entries.length} of {total}</p>
	{/if}
</div>

<style lang="scss">
	// no overflow hidden, the menus of the last rows reach out
	.list {
		padding: 0 0 6px;
	}

	.search {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 10px 14px;

		.field {
			width: 320px;
		}

		.total {
			margin-left: auto;
		}
	}

	.row {
		display: grid;
		grid-template-columns: minmax(0, 1fr) 13rem 7rem 2rem;
		align-items: center;
		gap: 12px;
		min-height: 44px;
		padding: 4px 16px;
		border-top: 1px solid var(--line);

		&.types {
			grid-template-columns: minmax(0, 1fr) 13rem 10rem 7rem 2rem;
		}

		&:not(.columns):hover {
			background: var(--surface-2);
		}
	}

	.columns {
		min-height: 32px;
		color: var(--muted);
		font-size: var(--fs-xs);
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
	}

	.type,
	.date {
		color: var(--muted);
		font-size: var(--fs-md);
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
		font-size: var(--fs-base);
	}

	@media (max-width: 800px) {
		.row,
		.row.types {
			grid-template-columns: minmax(0, 1fr) auto 2rem;
		}

		.hide-small {
			display: none;
		}

		.search .field {
			width: 100%;
		}
	}
</style>
