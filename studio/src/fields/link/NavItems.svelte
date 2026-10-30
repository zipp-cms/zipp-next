<script lang="ts">
	import NavItems from './NavItems.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import LinkAdd from './LinkAdd.svelte';
	import LinkEditor from './LinkEditor.svelte';
	import LinkRow from './LinkRow.svelte';
	import { hasTarget, linkSummary, opensInNewTab } from './link';
	import { inserted, moved, removed, replaced } from '../../core/list';
	import type { Link, LinkKind, NavItem, Schema } from '../../core/schema';
	import type { Refs } from '../../entries/api';

	/**
	 * A level of a navigation: a row per link, its settings under it while
	 * editing, the items under it indented, up to `depth` levels. An item
	 * moves under the one above it and out again.
	 */
	let {
		schema,
		refs,
		items,
		to,
		depth,
		onchange,
		onoutdent,
	}: {
		schema: Schema;
		refs: Refs;
		items: NavItem[];
		to: LinkKind[];
		/** the levels from here down */
		depth: number;
		onchange: (items: NavItem[]) => void;
		/** an item of this level moves out, next to its parent */
		onoutdent?: (i: number) => void;
	} = $props();

	let editing = $state<number | null>(null);

	/** The link of an item, without its children. */
	function linkOf(item: NavItem): Link {
		const { children: _, ...link } = item;
		return link;
	}

	function setLink(i: number, link: Link) {
		const children = items[i].children;
		onchange(
			replaced(items, i, { ...link, ...(children ? { children } : {}) }),
		);
	}

	function setChildren(i: number, children: NavItem[]) {
		const { children: _, ...item } = items[i];
		onchange(
			replaced(items, i, children.length ? { ...item, children } : item),
		);
	}

	/** The levels of an item with the ones under it. */
	const levels = (item: NavItem): number =>
		1 + Math.max(0, ...(item.children ?? []).map(levels));

	/** Under the item above, if the levels allow it. */
	const canIndent = (i: number) =>
		i > 0 && depth > 1 && levels(items[i]) < depth;

	function indent(i: number) {
		const parent = items[i - 1];
		const next = replaced(items, i - 1, {
			...parent,
			children: [...(parent.children ?? []), items[i]],
		});
		editing = null;
		onchange(removed(next, i));
	}

	/** A child of item `i` moves out, right after it. */
	function outdent(i: number, child: number) {
		const children = items[i].children ?? [];
		const { children: _, ...parent } = items[i];
		const rest = removed(children, child);
		const next = replaced(
			items,
			i,
			rest.length ? { ...parent, children: rest } : parent,
		);
		onchange(inserted(next, children[child], i + 1));
	}

	function move(i: number, by: number) {
		onchange(moved(items, i, by));
		if (editing === i) editing = i + by;
	}

	function remove(i: number) {
		const what = items[i].children?.length
			? ' with the links under it'
			: '';
		if (hasTarget(items[i]) && !confirm(`Remove the link${what}?`)) return;
		editing = null;
		onchange(removed(items, i));
	}

	function add(type: LinkKind) {
		// the new one's place, before the items change
		const at = items.length;
		onchange(inserted(items, { type, value: null }));
		editing = at;
	}
</script>

<div class="items" class:nested={!!onoutdent}>
	{#each items as item, i (i)}
		{@const link = linkOf(item)}
		{@const newTab = opensInNewTab(schema, link, true)}
		<div class="item" class:editing={editing === i}>
			<LinkRow {link} summary={linkSummary(schema, link, refs)} {newTab}>
				{#snippet actions()}
					{#if onoutdent}
						<button
							type="button"
							class="icon"
							title="Move out a level"
							onclick={() => onoutdent(i)}
						>
							<Icon name="outdent" size={15} />
						</button>
					{:else if depth > 1}
						<button
							type="button"
							class="icon"
							title="Move under the link above"
							disabled={!canIndent(i)}
							onclick={() => indent(i)}
						>
							<Icon name="indent" size={15} />
						</button>
					{/if}
					<button
						type="button"
						class="icon"
						title="Move up"
						disabled={i === 0}
						onclick={() => move(i, -1)}
					>
						<Icon name="arrow-up" size={15} />
					</button>
					<button
						type="button"
						class="icon"
						title="Move down"
						disabled={i === items.length - 1}
						onclick={() => move(i, 1)}
					>
						<Icon name="arrow-down" size={15} />
					</button>
					<button
						type="button"
						class="icon"
						class:on={editing === i}
						title="Edit the link"
						onclick={() => (editing = editing === i ? null : i)}
					>
						<Icon name="edit" size={15} />
					</button>
					<button
						type="button"
						class="icon remove"
						title="Remove the link"
						onclick={() => remove(i)}
					>
						<Icon name="delete" size={15} />
					</button>
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
		{#if item.children?.length}
			<NavItems
				{schema}
				{refs}
				items={item.children}
				{to}
				depth={depth - 1}
				onchange={children => setChildren(i, children)}
				onoutdent={child => outdent(i, child)}
			/>
		{/if}
	{/each}
</div>

{#if !onoutdent}
	<div class="add">
		<LinkAdd {to} onadd={add} />
		<span class="note">
			{depth > 1 ? `Up to ${depth} levels` : 'One level'}
		</span>
	</div>
{/if}

<style lang="scss">
	// all levels in one card
	.items:not(.nested) {
		padding: 0.375rem;
		border: 1px solid var(--line);
		border-radius: var(--radius-lg);
		background: var(--surface);

		&:empty {
			display: none;
		}
	}

	// the level under a link: indented, with a line
	.nested {
		margin-left: 1.5rem;
		padding-left: 0.75rem;
		border-left: 2px solid var(--line);
	}

	.item {
		padding: 0.375rem 0.5rem;
		border-radius: var(--radius);

		&.editing {
			background: var(--surface-2);
		}
	}

	.panel {
		margin: 0.25rem 0 0.5rem 2.75rem;
		padding: 1rem;
		border: 1px solid var(--accent);
		border-radius: var(--radius-lg);
		background: var(--surface-2);
	}

	button.on {
		background: var(--accent-soft);
		color: var(--accent);
	}

	.remove {
		color: var(--danger);
	}

	.add {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-top: 0.75rem;
	}
</style>
