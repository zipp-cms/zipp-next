<script module lang="ts">
	import type { Schema as SchemaT } from '../core/schema';
	import { listEntries as list, type EntryList } from './api';
	import { entryPath, go, site, type LoadProps } from '../core/router.svelte';

	/** The entries of the section; a single opens its entry. */
	export async function loadProps(
		props: { schema: SchemaT; section: string },
		lp: LoadProps,
	) {
		const sec = props.schema.sections.find(s => s.handle === props.section);
		if (!sec) throw new Error(`There is no section ${props.section}`);
		const first = await list(
			listParams(sec.kind, props.section, site.current, ''),
		);
		if (sec.kind === 'single' && first.entries[0])
			lp.setRedirect(entryPath(props.section, first.entries[0].id));
		return { first };
	}

	function listParams(
		kind: string,
		section: string,
		site: string,
		search: string,
	) {
		const params: Record<string, string> = { section, site, limit: '500' };
		if (kind === 'structure') params.order = 'structure';
		if (search) params.search = search;
		return params;
	}
</script>

<script lang="ts">
	import { untrack } from 'svelte';
	import SearchInput, { SEARCH_DELAY_MS } from 'zipp-ui/SearchInput.svelte';
	import SiteBadges from './SiteBadges.svelte';
	import PageTop from 'zipp-ui/PageTop.svelte';
	import { can, sectionCrumbs } from '../core/access';
	import { entryUrl, type Schema } from '../core/schema';
	import { deleteEntry, listEntries, moveEntry, type Entry } from './api';
	import Menu, { type MenuItem } from 'zipp-ui/Menu.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import { date, plural } from '../core/format';
	import { depth, isBelow } from './tree';
	import DragHandle from 'zipp-ui/DragHandle.svelte';
	import SortLine from 'zipp-ui/SortLine.svelte';
	import { Sortable, placeIn } from 'zipp-ui/sortable';
	import { componentName, sectionOf } from '../core/schema';

	let {
		schema,
		section,
		first,
	}: { schema: Schema; section: string; first: EntryList } = $props();

	const sec = $derived(sectionOf(schema, section)!);
	const tree = $derived(sec.kind === 'structure');
	// moving changes the website at once, like publishing
	let search = $state('');
	// not while searching, the results are no tree
	const canMove = $derived(
		tree && !search && can(schema, section, 'publish'),
	);
	let entries = $state<Entry[]>(untrack(() => first.entries));
	let published = $state(untrack(() => first.published));
	// the entries with unpublished changes
	let drafts = $state(untrack(() => first.drafts));
	// the type is worth a column only with several
	const types = $derived(sec.types.length > 1);

	let total = $state(untrack(() => first.total));
	let error = $state('');

	/** Loads the list again: after a search, a move, a delete. */
	function load() {
		const q = search;
		listEntries(listParams(sec.kind, section, site.current, q))
			.then(r => {
				// an answer to an older search
				if (q !== search) return;
				entries = r.entries;
				total = r.total;
				published = r.published;
				drafts = r.drafts;
			})
			.catch(e => (error = e.message));
	}

	// the first list came with the page
	let searched = '';
	// a moment after the last key, not with every one
	$effect(() => {
		if (search === searched) return;
		const wait = setTimeout(() => {
			searched = search;
			load();
		}, SEARCH_DELAY_MS);
		return () => clearTimeout(wait);
	});

	// moving in a structure: dragging up and down, left and right for the
	// level; a move changes the website at once
	let rows = $state<HTMLElement>();
	const depths = $derived(entries.map(e => depth(entries, e)));
	const sort = new Sortable({
		container: () => rows,
		depths: () => depths,
		maxDepth: () => 20,
		indent: 21,
		inset: 16,
		onmove: async (from, drop) => {
			const to = placeIn(depths, from, drop);
			const parentId = to.parent === null ? null : entries[to.parent].id;
			try {
				await moveEntry(entries[from].id, parentId, to.index);
				load();
			} catch (err: any) {
				error = err.message;
			}
		},
	});

	/** Edit, view and delete without opening the entry. */
	function actions(e: Entry): MenuItem[] {
		const items: MenuItem[] = [
			{
				label: 'Edit',
				icon: 'edit',
				href: entryPath(section, e.id),
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
		const below = entries.filter(c => isBelow(entries, c, e)).length;
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
</script>

<PageTop crumbs={sectionCrumbs(schema, section)}>
	{#snippet actions()}
		{#if sec.kind !== 'single' && can(schema, section, 'edit')}
			<button
				class="primary"
				onclick={() => go(`/admin/entries/${section}/new`)}
			>
				<Icon name="add" />
				New entry
			</button>
		{/if}
	{/snippet}
</PageTop>

{#if error}<p class="error">{error}</p>{/if}

<div class="card list" bind:this={rows}>
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
		{@const i = entries.indexOf(e)}
		<div
			class="row"
			class:types
			data-sort
			class:sort-dragging={sort.moving(i)}
			role="listitem"
		>
			<span
				class="name"
				style:padding-left="{depth(entries, e) * 1.3125}rem"
			>
				{#if canMove}
					<DragHandle
						sortable={sort}
						index={i}
						label="Move {e.title}"
					/>
				{/if}
				<a class="title" href={entryPath(section, e.id)}>
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
					{componentName(schema, e.type)}
				</span>
			{/if}
			<span class="date hide-small">{date(e.updatedAt)}</span>
			<Menu items={actions(e)} label="Actions for {e.title}" />
		</div>
	{:else}
		<p class="empty">
			{search ? 'Nothing found' : 'No entries yet'}
		</p>
	{/each}
	{#if total > entries.length}
		<p class="empty">{entries.length} of {total}</p>
	{/if}
	<SortLine sortable={sort} />
</div>

<style lang="scss">
	// no overflow hidden, the menus of the last rows reach out
	.list {
		position: relative;
		padding: 0 0 0.375rem;
	}

	.search {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.625rem 0.875rem;

		.field {
			width: 20rem;
		}

		.total {
			margin-left: auto;
		}
	}

	.row {
		display: grid;
		grid-template-columns: minmax(0, 1fr) 11.375rem 6.125rem 1.75rem;
		align-items: center;
		gap: 0.75rem;
		min-height: 2.75rem;
		padding: 0.25rem 1rem;
		border-top: 1px solid var(--line);

		&.types {
			grid-template-columns:
				minmax(0, 1fr)
				11.375rem 8.75rem 6.125rem 1.75rem;
		}

		@include hover {
			&:not(.columns):hover {
				background: var(--surface-2);
			}
		}
	}

	.columns {
		min-height: 2rem;
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

		@include hover {
			&:hover {
				color: var(--accent);
			}
		}
	}

	.status {
		display: flex;
		align-items: center;
		gap: 0.25rem;
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
		gap: 0.5rem;
		min-width: 0;
	}

	.type {
		font-size: var(--fs-base);
	}

	@include max-tablet {
		.row,
		.row.types {
			grid-template-columns: minmax(0, 1fr) auto 1.75rem;
		}

		.hide-small {
			display: none;
		}

		.search .field {
			width: 100%;
		}
	}
</style>
