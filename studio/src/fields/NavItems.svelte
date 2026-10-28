<script lang="ts">
	import NavItems from './NavItems.svelte';
	import LinkField from './LinkField.svelte';
	import Icon from '../lib/Icon.svelte';
	import { moved } from '../lib/list';
	import type { Field, LinkKind, NavItem, Refs, Schema } from '../api';

	/**
	 * The items of a level of a navigation, each a link with the items
	 * under it, up to `depth` levels.
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

	// the link editor of an item, it edits the item but not its children
	const linkField = $derived<Field>({
		handle: 'link',
		label: 'Link',
		required: false,
		indexed: false,
		translatable: false,
		kind: 'link',
		to,
		labels: true,
		newTab: true,
	});
	function holder(i: number) {
		return {
			get link() {
				const { children: _, ...link } = items[i];
				return link;
			},
			set link(link: NavItem) {
				const children = items[i].children;
				replace(i, { ...link, ...(children ? { children } : {}) });
			},
		};
	}

	function replace(i: number, item: NavItem) {
		onchange(items.map((x, j) => (j === i ? item : x)));
	}

	const move = (i: number, by: number) => onchange(moved(items, i, by));
</script>

{#each items as item, i (i)}
	<div class="item">
		<div class="head">
			<div class="link">
				<LinkField
					{schema}
					field={linkField}
					values={holder(i)}
					{refs}
				/>
			</div>
			<div class="actions">
				<button
					type="button"
					class="link"
					disabled={i === 0}
					onclick={() => move(i, -1)}
				>
					↑
				</button>
				<button
					type="button"
					class="link"
					disabled={i === items.length - 1}
					onclick={() => move(i, 1)}
				>
					↓
				</button>
				<button
					type="button"
					class="link"
					title="Remove with the items under it"
					onclick={() => onchange(items.filter((_, j) => j !== i))}
				>
					<Icon name="delete" />
				</button>
			</div>
		</div>
		{#if depth > 1}
			<div class="children">
				<NavItems
					{schema}
					{refs}
					items={item.children ?? []}
					{to}
					depth={depth - 1}
					onchange={children =>
						replace(i, {
							...item,
							children: children.length ? children : undefined,
						})}
				/>
			</div>
		{/if}
	</div>
{/each}
<button
	type="button"
	class="link add"
	onclick={() => onchange([...items, { type: to[0], value: null }])}
>
	<Icon name="add" /> Add a link
</button>

<style>
	.item {
		margin-bottom: 10px;
		padding: 10px;
		border-left: 3px solid var(--line);
		background: rgba(0, 0, 0, 0.02);
	}

	.head {
		display: flex;
		gap: 10px;
		align-items: flex-start;
	}

	.head .link {
		flex: 1;
	}

	.actions {
		display: flex;
		gap: 4px;
	}

	.children {
		margin: 10px 0 0 16px;
	}

	.add {
		display: inline-flex;
		align-items: center;
		gap: 6px;
	}
</style>
