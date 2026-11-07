<script module lang="ts">
	import { sectionOf, type Schema as SchemaT } from '../core/schema';
	import { listEntries, type EntryList } from './api';
	import { go, site, type LoadProps } from '../core/router.svelte';
	import { entryPath } from '../core/paths';
	import { t } from '../i18n/index.svelte';

	/** The entries of the section; a single opens its entry. */
	export async function loadProps(
		props: { schema: SchemaT; section: string },
		lp: LoadProps,
	) {
		const sec = sectionOf(props.schema, props.section);
		if (!sec)
			throw new Error(t('entries.noSection', { section: props.section }));
		const first = await listEntries(
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
	import SearchInput from 'zipp-ui/SearchInput.svelte';
	import { Latest } from '../core/latest.svelte';
	import { Busy } from '../core/busy.svelte';
	import EntryBadges from './EntryBadges.svelte';
	import PageTop from 'zipp-ui/PageTop.svelte';
	import { can, canEntry, canNew } from '../core/access';
	import { sectionCrumbs } from '../core/paths';
	import { entryUrl, type Schema } from '../core/schema';
	import { deleteEntry, moveEntry, type Entry } from './api';
	import Menu, { type MenuItem } from 'zipp-ui/Menu.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import { date } from '../core/format';
	import { ancestors, depths as depthsOf } from '../core/tree';
	import DragHandle from 'zipp-ui/DragHandle.svelte';
	import SortLine from 'zipp-ui/SortLine.svelte';
	import { Sortable, placeIn } from 'zipp-ui/sortable';
	import { componentName, componentOf } from '../core/schema';
	import SchemaIcon from '../core/SchemaIcon.svelte';

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
	// moving and deleting
	const action = new Busy();

	/**
	 * Loads the list again: after a search, a move, a delete. Only the
	 * last one counts, an older answer would show an old order.
	 */
	const loads = new Latest();
	const ask = () =>
		listEntries(listParams(sec.kind, section, site.current, search));
	function show(r: EntryList) {
		entries = r.entries;
		total = r.total;
		published = r.published;
		drafts = r.drafts;
		action.error = '';
	}
	const load = () => loads.run(ask, show);

	// the first list came with the page
	let searched = '';
	// a moment after the last key, not with every one
	$effect(() => {
		if (search === searched) return;
		searched = search;
		loads.later(ask, show);
		return () => loads.stop();
	});

	// moving in a structure: dragging up and down, left and right for the
	// level; a move changes the website at once
	let rows = $state<HTMLElement>();
	const depths = $derived(depthsOf(entries));
	const sort = new Sortable({
		container: () => rows,
		depths: () => depths,
		// depths count from 0, levels from 1
		maxDepth: () => (sec.max_level ?? 21) - 1,
		indent: 21,
		inset: 16,
		onmove: async (from, drop) => {
			const to = placeIn(depths, from, drop);
			const parentId = to.parent === null ? null : entries[to.parent].id;
			await action.run(async () => {
				await moveEntry(entries[from].id, parentId, to.index);
				// the handle keeps the focus in the new order
				await load();
			});
		},
	});

	/** Edit, view and delete without opening the entry. */
	function actions(e: Entry): MenuItem[] {
		const items: MenuItem[] = [
			{
				label: t('common.edit'),
				icon: 'edit',
				href: entryPath(section, e.id),
			},
		];
		const url = entryUrl(schema, site.current, e.uri);
		if (url && published[e.id]?.[site.current])
			items.push({
				label: t('entries.viewOnWebsite'),
				icon: 'external',
				href: url,
			});
		if (
			sec.kind !== 'single' &&
			canEntry(schema, section, e.authorId, 'delete')
		)
			items.push({
				label: t('common.delete'),
				icon: 'delete',
				danger: true,
				onclick: () => remove(e),
			});
		return items;
	}

	async function remove(e: Entry) {
		// the entries below it go with it; while searching the list has
		// only the matches, the count needs the whole tree
		let all = entries;
		if (tree && search) {
			const r = await action.run(() =>
				listEntries(listParams(sec.kind, section, site.current, '')),
			);
			if (!r) return;
			all = r.entries;
		}
		const byId = new Map(all.map(c => [c.id, c]));
		const below = all.filter(c =>
			ancestors(byId, c).some(a => a.id === e.id),
		).length;
		const question = below
			? t('entries.confirmTrashBelow', { title: e.title, n: below })
			: t('entries.confirmTrash', { title: e.title });
		if (!confirm(question)) return;
		await action.run(async () => {
			await deleteEntry(e.id);
			load();
		});
	}
</script>

<PageTop crumbs={sectionCrumbs(schema, section)}>
	{#snippet actions()}
		{#if sec.kind !== 'single' && canNew(schema, section, 'edit')}
			<button
				class="primary"
				onclick={() => go(entryPath(section, 'new'))}
			>
				<Icon name="add" />
				{t('entries.new')}
			</button>
		{/if}
	{/snippet}
</PageTop>

{#if loads.error || action.error}
	<p class="error">{loads.error || action.error}</p>
{/if}

<div class="card list" bind:this={rows}>
	<div class="search">
		<span class="field">
			<SearchInput
				bind:value={search}
				placeholder={t('entries.search')}
			/>
		</span>
		<span class="note total">{t('common.entries', { n: total })}</span>
	</div>
	{#if entries.length}
		<div class="row columns overline" class:types>
			<span>{t('common.title')}</span>
			<span>{t('entries.status')}</span>
			{#if types}<span class="hide-small">{t('entries.type')}</span>{/if}
			<span class="hide-small">{t('entries.updated')}</span>
			<span></span>
		</div>
	{/if}
	{#each entries as e, i (e.id)}
		<div
			class="row"
			class:types
			data-sort
			data-menu-area
			class:sort-dragging={sort.moving(i)}
			role="listitem"
		>
			<span class="name" style:padding-left="{depths[i] * 1.3125}rem">
				{#if canMove}
					<DragHandle
						sortable={sort}
						index={i}
						label={t('entries.move', { title: e.title })}
					/>
				{/if}
				<a class="title" href={entryPath(section, e.id)}>
					{e.title}
				</a>
			</span>
			<span class="status">
				<EntryBadges
					{schema}
					published={published[e.id] ?? {}}
					draft={drafts.includes(e.id)}
					dates={e}
				/>
			</span>
			{#if types}
				<span class="type hide-small">
					<SchemaIcon
						icon={componentOf(schema, e.type)?.icon}
						size={14}
					/>
					{componentName(schema, e.type)}
				</span>
			{/if}
			<span class="date hide-small">{date(e.updatedAt)}</span>
			<Menu
				items={actions(e)}
				label={t('entries.actionsFor', { title: e.title })}
			/>
		</div>
	{:else}
		<p class="empty">
			{search ? t('ui.nothingFound') : t('entries.noneYet')}
		</p>
	{/each}
	{#if total > entries.length}
		<p class="empty">
			{t('entries.shownOf', { shown: entries.length, total })}
		</p>
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
		display: flex;
		align-items: center;
		gap: 0.375rem;
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
