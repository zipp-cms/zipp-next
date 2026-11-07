<script lang="ts">
	import { untrack } from 'svelte';
	import type { Block, FieldOf, Schema, Values } from '../core/schema';
	import { valueOf } from '../core/schema';
	import type { Refs } from '../entries/api';
	import BlockMenu from './BlockMenu.svelte';
	import BlockFields from './BlockFields.svelte';
	import ItemActions from 'zipp-ui/ItemActions.svelte';
	import ItemCard from 'zipp-ui/ItemCard.svelte';
	import DragHandle from 'zipp-ui/DragHandle.svelte';
	import SortLine from 'zipp-ui/SortLine.svelte';
	import { listSortable } from 'zipp-ui/sortable';
	import {
		blockMissing,
		blockSummary,
		focus,
		newBlockId,
		withNewIds,
	} from './blocks.svelte';
	import { rememberOpen, wasOpen } from './opened';
	import { inserted, isFull, removed, rowKeys } from '../core/list';
	import { componentOf, defaults } from '../core/schema';
	import SchemaIcon from '../core/SchemaIcon.svelte';
	import { t } from '../i18n/index.svelte';

	/**
	 * Blocks as cards that open: the type, a summary when closed, the
	 * actions on hover; a + between two adds one there.
	 */
	let {
		schema,
		field,
		values,
		errors,
		refs,
		path,
		changed = new Set(),
	}: {
		schema: Schema;
		field: FieldOf<'blocks'>;
		values: Values;
		errors: Record<string, string>;
		refs: Refs;
		path: string;
		changed?: Set<string>;
	} = $props();

	const types = $derived(
		schema.components.filter(c => field.types.includes(c.handle)),
	);
	const max = $derived(field.max);
	const blocks = $derived(valueOf(values, field) ?? []);
	const full = $derived(isFull(blocks.length, max));
	const set = (next: Block[]) => (values[field.handle] = next);

	// a block in this list, also one of an old draft without an id
	const { key } = rowKeys<Block>();

	// as it was in this browser, else a long list starts closed; a new
	// block opens
	let open = $state<Record<number, boolean>>({});
	const startOpen = untrack(() => blocks.length <= 2);
	const isOpen = (b: Block) => open[key(b)] ?? wasOpen(b.id) ?? startOpen;
	function setOpen(b: Block, to: boolean) {
		open[key(b)] = to;
		rememberOpen(b.id, to);
	}

	function setAll(to: boolean) {
		for (const b of blocks) setOpen(b, to);
	}

	/** Inserts a block and opens it: the one in the state, a proxy. */
	function insert(block: Block, at: number) {
		set(inserted(blocks, block, at));
		setOpen(blocks[at], true);
	}

	const add = (type: string, at = blocks.length) =>
		insert({ id: newBlockId(), type, fields: defaults(schema, type) }, at);

	function duplicate(i: number) {
		const copy: Block = $state.snapshot(blocks[i]);
		withNewIds(schema, copy);
		insert(copy, i + 1);
	}

	/** Off and on again; on is no key, like blocks that never were off. */
	function switchBlock(b: Block) {
		if (b.enabled === false) delete b.enabled;
		else b.enabled = false;
	}

	/** Whether a block or something inside it has an error. */
	const hasError = (i: number) =>
		Object.keys(errors).some(
			p => p === `${path}.${i}` || p.startsWith(`${path}.${i}.`),
		);

	// the outline of the editor opens a block and scrolls to it; an ask
	// from before this field mounted is old, another editor's
	let seen = untrack(() => focus.at);
	$effect(() => {
		if (focus.at === seen) return;
		seen = focus.at;
		if (focus.path !== path) return;
		// only when asked, not when the blocks change later
		const b = untrack(() => blocks[focus.index]);
		if (!b) return;
		setOpen(b, true);
		setTimeout(() =>
			document
				.getElementById(`block-${path}-${focus.index}`)
				?.scrollIntoView({ behavior: 'smooth', block: 'start' }),
		);
	});

	// dragging a block, or its handle's menu and arrow keys
	let list = $state<HTMLElement>();
	const sort = listSortable(
		() => list,
		() => blocks,
		set,
	);
</script>

<div class="blocks" bind:this={list}>
	{#if blocks.length > 1}
		<div class="tools">
			<span>{t('fields.blocks', { n: blocks.length })}</span>
			<button type="button" class="link" onclick={() => setAll(false)}>
				{t('fields.collapseAll')}
			</button>
			<button type="button" class="link" onclick={() => setAll(true)}>
				{t('fields.expandAll')}
			</button>
		</div>
	{/if}

	{#each blocks as block, i (key(block))}
		{@const comp = componentOf(schema, block.type)}
		{#if i > 0 && !full}
			<BlockMenu {types} inline onpick={type => add(type, i)} />
		{/if}
		<div class="block" data-sort class:sort-dragging={sort.moving(i)}>
			{#snippet kindIcon()}<SchemaIcon
					icon={comp?.icon}
					size={12}
				/>{/snippet}
			<ItemCard
				kind={types.length === 1 &&
				blockSummary(comp, block.fields, refs)
					? undefined
					: (comp?.name ?? block.type)}
				kindIcon={comp?.icon ? kindIcon : undefined}
				bind:open={() => isOpen(block), v => setOpen(block, v)}
				invalid={hasError(i)}
				changed={changed.has(`${path}.${i}`)}
				off={block.enabled === false}
				id="block-{path}-{i}"
			>
				{#snippet summary()}
					{@const missing = blockMissing(comp, block.fields)}
					{#if missing}
						<span class="missing">{missing}</span>
					{:else}
						{blockSummary(comp, block.fields, refs)}
					{/if}
				{/snippet}
				{#snippet handle()}
					<DragHandle
						sortable={sort}
						index={i}
						label={t('fields.moveBlock')}
					/>
				{/snippet}
				{#snippet actions()}
					<ItemActions
						onswitch={() => switchBlock(block)}
						off={block.enabled === false}
						onduplicate={full ? undefined : () => duplicate(i)}
						onremove={() => set(removed(blocks, i))}
					/>
				{/snippet}
				{#if errors[`${path}.${i}`]}
					<p class="error">{errors[`${path}.${i}`]}</p>
				{/if}
				{#if comp}
					<BlockFields
						{schema}
						component={comp}
						values={block.fields}
						{errors}
						{refs}
						prefix="{path}.{i}."
						{changed}
						id={block.id}
					/>
				{/if}
			</ItemCard>
		</div>
	{/each}
	<SortLine sortable={sort} />

	{#if !full}
		<BlockMenu {types} onpick={type => add(type)} />
	{/if}
</div>

<style lang="scss">
	// the space between blocks like between the cards of the sidebar,
	// the + to add one in its middle
	.blocks {
		position: relative;
		display: flex;
		flex-direction: column;
	}

	.block + .block,
	.block + :global(.inline) {
		margin-top: 0.75rem;
	}

	// the bar further from the last block than they are from each other,
	// none above it while the field is empty
	.block ~ :global(.bar) {
		margin-top: 0.875rem;
	}

	.tools {
		margin-bottom: 0.375rem;
		display: flex;
		align-items: center;
		gap: 0.25rem;
		color: var(--muted);
		font-size: var(--fs-sm);

		span {
			margin-right: auto;
		}
	}

	.missing {
		color: var(--danger);
	}
</style>
