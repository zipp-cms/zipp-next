<script lang="ts">
	import DragHandle from 'zipp-ui/DragHandle.svelte';
	import SortLine from 'zipp-ui/SortLine.svelte';
	import { Sortable, placeIn, type Drop } from 'zipp-ui/sortable';
	import LinkAdd from './LinkAdd.svelte';
	import LinkEditor from './LinkEditor.svelte';
	import LinkRow from './LinkRow.svelte';
	import { hasTarget, linkSummary, opensInNewTab } from './link';
	import type { Link, LinkKind, NavItem, Schema } from '../../core/schema';
	import type { Refs } from '../../entries/api';

	/**
	 * The links of a navigation as rows in one card, the ones under a link
	 * indented, up to `depth` levels: a row per link, its settings under it
	 * while editing. The handle moves a link, also a level up or down.
	 */
	let {
		schema,
		refs,
		items,
		to,
		depth,
		onchange,
	}: {
		schema: Schema;
		refs: Refs;
		items: NavItem[];
		to: LinkKind[];
		/** the levels */
		depth: number;
		onchange: (items: NavItem[]) => void;
	} = $props();

	type Row = { item: NavItem; depth: number; path: number[] };

	/** The tree as rows, a link before the ones under it. */
	function flatten(list: NavItem[], d = 0, path: number[] = []): Row[] {
		return list.flatMap((item, i) => [
			{ item, depth: d, path: [...path, i] },
			...flatten(item.children ?? [], d + 1, [...path, i]),
		]);
	}

	const rows = $derived(flatten(items));
	// the row whose settings are open
	let editing = $state<number | null>(null);

	/** A copy of the items to change, with the rows of the copy. */
	function copy(): [NavItem[], Row[]] {
		const next = $state.snapshot(items) as NavItem[];
		return [next, flatten(next)];
	}

	/** The list a row is in, of its parent or the top. */
	function siblings(list: NavItem[], path: number[]): NavItem[] {
		let level = list;
		for (const i of path.slice(0, -1)) level = level[i].children!;
		return level;
	}

	/** Takes `item` out of its list, an empty list of children goes. */
	function take(list: NavItem[], row: Row) {
		const sibs = siblings(list, row.path);
		sibs.splice(row.path.at(-1)!, 1);
		if (!sibs.length && row.path.length > 1) {
			delete siblings(list, row.path.slice(0, -1))[row.path.at(-2)!]
				.children;
		}
	}

	function move(from: number, drop: Drop) {
		const [next, copied] = copy();
		const to = placeIn(
			rows.map(r => r.depth),
			from,
			drop,
		);
		const item = copied[from].item;
		const parent = to.parent === null ? null : copied[to.parent].item;
		take(next, copied[from]);
		const list = parent ? (parent.children ??= []) : next;
		list.splice(to.index, 0, item);
		editing = null;
		onchange(next);
	}

	let list = $state<HTMLElement>();
	const sort = new Sortable({
		container: () => list,
		depths: () => rows.map(r => r.depth),
		maxDepth: () => depth - 1,
		indent: 36,
		// the padding of a row, the levels are indented in it
		inset: 12,
		onmove: move,
	});

	/** The link of an item, without its children. */
	function linkOf(item: NavItem): Link {
		const { children: _, ...link } = item;
		return link;
	}

	function setLink(i: number, link: Link) {
		const [next, copied] = copy();
		const children = copied[i].item.children;
		const sibs = siblings(next, copied[i].path);
		sibs[copied[i].path.at(-1)!] = {
			...link,
			...(children ? { children } : {}),
		};
		onchange(next);
	}

	function remove(i: number) {
		const what = rows[i].item.children?.length
			? ' with the links under it'
			: '';
		if (hasTarget(rows[i].item) && !confirm(`Remove the link${what}?`))
			return;
		const [next, copied] = copy();
		take(next, copied[i]);
		editing = null;
		onchange(next);
	}

	function add(type: LinkKind) {
		// the new one's row, before the items change
		const at = rows.length;
		onchange([...items, { type, value: null }]);
		editing = at;
	}
</script>

<div class="items" bind:this={list}>
	{#each rows as row, i (row.path.join('.'))}
		{@const link = linkOf(row.item)}
		{@const newTab = opensInNewTab(schema, link, true)}
		<div
			data-sort
			class="row"
			class:sort-dragging={sort.moving(i)}
			style:--depth={row.depth}
		>
			<div class="item" class:editing={editing === i}>
				<LinkRow
					{link}
					summary={linkSummary(schema, link, refs)}
					{newTab}
					editing={editing === i}
					onedit={() => (editing = editing === i ? null : i)}
					menu={[
						{
							label: 'Edit',
							icon: 'edit',
							onclick: () => (editing = i),
						},
						{
							label: 'Delete',
							icon: 'delete',
							onclick: () => remove(i),
							danger: true,
						},
					]}
				>
					{#snippet handle()}
						<DragHandle
							sortable={sort}
							index={i}
							label="Move the link"
						/>
					{/snippet}
				</LinkRow>
			</div>
			{#if editing === i}
				<div class="panel">
					<LinkEditor
						{schema}
						{refs}
						{link}
						{to}
						labels
						{newTab}
						onchange={l => setLink(i, l)}
						ondone={() => (editing = null)}
					/>
				</div>
			{/if}
		</div>
	{/each}
	<SortLine sortable={sort} />
</div>

<div class="add">
	<LinkAdd {to} onadd={add} />
	<span class="note">
		{depth > 1 ? `Up to ${depth} levels` : 'One level'}
	</span>
</div>

<style lang="scss">
	// all levels in one card, a line between the links like the Pages list
	.items {
		position: relative;
		border: 1px solid var(--line);
		border-radius: var(--radius-lg);
		background: var(--surface);

		&:not(:has(.row)) {
			display: none;
		}
	}

	.row + .row {
		border-top: 1px solid var(--line);
	}

	// a link under another is indented in its row, the line goes across
	.item {
		padding: 0.625rem 0.75rem 0.625rem
			calc(0.75rem + var(--depth, 0) * 2.25rem);

		&.editing {
			background: var(--surface-2);
		}
	}

	.panel {
		margin: 0 0.75rem 0.75rem calc(3.5rem + var(--depth, 0) * 2.25rem);
		padding: 1rem;
		border: 1px solid var(--accent);
		border-radius: var(--radius-lg);
		background: var(--surface-2);
	}

	.add {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-top: 0.75rem;
	}
</style>
