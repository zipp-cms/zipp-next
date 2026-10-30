<script lang="ts">
	import NavItems from './NavItems.svelte';
	import LinkField from './LinkField.svelte';
	import AddButton from '../lib/AddButton.svelte';
	import ItemActions from '../lib/ItemActions.svelte';
	import ItemCard from '../lib/ItemCard.svelte';
	import { linkField, linkLabel } from '../lib/link';
	import { inserted, moved, removed, replaced } from '../lib/list';
	import type { LinkKind, NavItem, Refs, Schema } from '../api';

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

	const field = $derived(linkField(to));

	/** The link of an item for the link field, without its children. */
	function holder(i: number) {
		return {
			get link() {
				const { children: _, ...link } = items[i];
				return link;
			},
			set link(link: NavItem) {
				const children = items[i].children;
				onchange(
					replaced(items, i, {
						...link,
						...(children ? { children } : {}),
					}),
				);
			},
		};
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
			<LinkField {schema} {field} values={holder(i)} {refs} />
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

<style>
	.items {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	/* the second level indented */
	.children {
		margin: 12px 0 0 6px;
		padding-left: 12px;
		border-left: 2px solid var(--accent-soft);
	}
</style>
