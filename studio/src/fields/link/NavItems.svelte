<script lang="ts">
	import DragHandle from 'zipp-ui/DragHandle.svelte';
	import SortLine from 'zipp-ui/SortLine.svelte';
	import { Sortable, type Drop } from 'zipp-ui/sortable';
	import LinkAdd from './LinkAdd.svelte';
	import LinkEditor from './LinkEditor.svelte';
	import LinkRow from './LinkRow.svelte';
	import { hasTarget, opensInNewTab } from './link';
	import { flatten, linkOf, moveRow, removeRow, setRowLink } from './navTree';
	import type { LinkKind, NavItem, Schema } from '../../core/schema';
	import type { Refs } from '../../entries/api';
	import { t } from '../../i18n/index.svelte';

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

	const rows = $derived(flatten(items));
	// the row whose settings are open
	let editing = $state<number | null>(null);

	/** Changes a copy of the items and sets it. */
	function change(f: (next: NavItem[]) => void) {
		const next = $state.snapshot(items) as NavItem[];
		f(next);
		onchange(next);
	}

	function move(from: number, drop: Drop) {
		editing = null;
		change(next => moveRow(next, from, drop));
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

	function remove(i: number) {
		const question = rows[i].item.children?.length
			? t('fields.confirmRemoveLinkChildren')
			: t('fields.confirmRemoveLink');
		if (hasTarget(rows[i].item) && !confirm(question)) return;
		editing = null;
		change(next => removeRow(next, i));
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
		<div
			data-sort
			class="row"
			class:sort-dragging={sort.moving(i)}
			style:--depth={row.depth}
		>
			<div class="item" class:editing={editing === i}>
				<LinkRow
					{schema}
					{refs}
					{link}
					editing={editing === i}
					onedit={() => (editing = editing === i ? null : i)}
					menu={[
						{
							label: t('common.edit'),
							icon: 'edit',
							onclick: () => (editing = i),
						},
						{
							label: t('common.delete'),
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
							label={t('fields.moveLink')}
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
						newTab={opensInNewTab(schema, link, true)}
						onchange={l => change(next => setRowLink(next, i, l))}
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
		{depth > 1
			? t('fields.upToLevels', { n: depth })
			: t('fields.oneLevel')}
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

	// a link under another is indented in its row, the line goes across;
	// the click area of the LinkRow
	.item {
		position: relative;
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
	}

	// no space above the button while there are no links
	.items:has(.row) + .add {
		margin-top: 0.75rem;
	}
</style>
