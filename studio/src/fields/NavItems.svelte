<script lang="ts">
	import NavItems from './NavItems.svelte';
	import LinkInput from './LinkInput.svelte';
	import AddButton from '../ui/AddButton.svelte';
	import ItemActions from '../ui/ItemActions.svelte';
	import ItemCard from '../ui/ItemCard.svelte';
	import { linkLabel } from './link';
	import { inserted, moved, removed, replaced } from '../core/list';
	import type { Link, LinkKind, NavItem, Schema } from '../core/schema';
	import type { Refs } from '../entries/api';

	/**
	 * The items of a level of a navigation, each a card with its link and
	 * the items under it, up to `depth` levels.
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
		depth: number;
		onchange: (items: NavItem[]) => void;
	} = $props();

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
</script>

<div class="items">
	{#each items as item, i (i)}
		<ItemCard kind="Link">
			{#snippet summary()}{linkLabel(item, refs)}{/snippet}
			{#snippet actions()}
				<ItemActions
					index={i}
					count={items.length}
					onmove={by => onchange(moved(items, i, by))}
					onremove={() => onchange(removed(items, i))}
					removeTitle="Remove with the items under it"
				/>
			{/snippet}
			<LinkInput
				{schema}
				{refs}
				link={linkOf(item)}
				{to}
				labels
				onchange={link => setLink(i, link)}
			/>
			{#if depth > 1}
				<div class="children">
					<NavItems
						{schema}
						{refs}
						items={item.children ?? []}
						{to}
						depth={depth - 1}
						onchange={children => setChildren(i, children)}
					/>
				</div>
			{/if}
		</ItemCard>
	{/each}
	<div>
		<AddButton
			onclick={() =>
				onchange(inserted(items, { type: to[0], value: null }))}
		>
			Add a link
		</AddButton>
	</div>
</div>

<style lang="scss">
	.items {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	/* the second level indented */
	.children {
		margin: 0.75rem 0 0 0.375rem;
		padding-left: 0.75rem;
		border-left: 2px solid var(--accent-soft);
	}
</style>
