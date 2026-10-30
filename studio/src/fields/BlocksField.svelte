<script lang="ts">
	import { untrack } from 'svelte';
	import type { Block, FieldOf, Schema, Values } from '../core/schema';
	import { valueOf } from '../core/schema';
	import type { Refs } from '../entries/api';
	import BlockMenu from './BlockMenu.svelte';
	import Fields from './Fields.svelte';
	import ItemActions from '../ui/ItemActions.svelte';
	import ItemCard from '../ui/ItemCard.svelte';
	import { blockMissing, blockSummary, focus } from './blocks.svelte';
	import { inserted, isFull, moved, removed } from '../core/list';
	import { componentOf } from '../core/schema';

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

	// blocks without an id are new, they get one on save
	let keys = new WeakMap<Block, number>();
	let nextKey = 0;
	function key(b: Block) {
		if (!keys.has(b)) keys.set(b, nextKey++);
		return keys.get(b)!;
	}

	// a long list starts closed, a new block opens
	let open = $state<Record<number, boolean>>({});
	const startOpen = untrack(() => blocks.length <= 2);

	function setAll(to: boolean) {
		for (const b of blocks) open[key(b)] = to;
	}

	/** Inserts a block and opens it: the one in the state, a proxy. */
	function insert(block: Block, at: number) {
		set(inserted(blocks, block, at));
		open[key(blocks[at])] = true;
	}

	const add = (type: string, at = blocks.length) =>
		insert({ type, fields: {} }, at);

	function duplicate(i: number) {
		const copy: Block = $state.snapshot(blocks[i]);
		delete copy.id;
		insert(copy, i + 1);
	}

	/** Whether a block or something inside it has an error. */
	const hasError = (i: number) =>
		Object.keys(errors).some(
			p => p === `${path}.${i}` || p.startsWith(`${path}.${i}.`),
		);

	// the outline of the editor opens a block and scrolls to it
	$effect(() => {
		if (focus.path !== path || !focus.at) return;
		// only when asked, not when the blocks change later
		const b = untrack(() => blocks[focus.index]);
		if (!b) return;
		open[key(b)] = true;
		setTimeout(() =>
			document
				.getElementById(`block-${path}-${focus.index}`)
				?.scrollIntoView({ behavior: 'smooth', block: 'start' }),
		);
	});
</script>

<div class="blocks">
	{#if blocks.length > 1}
		<div class="tools">
			<span>{blocks.length} blocks</span>
			<button type="button" class="link" onclick={() => setAll(false)}>
				Collapse all
			</button>
			<button type="button" class="link" onclick={() => setAll(true)}>
				Expand all
			</button>
		</div>
	{/if}

	{#each blocks as block, i (key(block))}
		{@const comp = componentOf(schema, block.type)}
		{#if i > 0 && !full}
			<BlockMenu {types} inline onpick={t => add(t, i)} />
		{/if}
		<ItemCard
			kind={comp?.name ?? block.type}
			bind:open={
				() => open[key(block)] ?? startOpen, v => (open[key(block)] = v)
			}
			invalid={hasError(i)}
			changed={changed.has(`${path}.${i}`)}
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
			{#snippet actions()}
				<ItemActions
					index={i}
					count={blocks.length}
					onmove={by => set(moved(blocks, i, by))}
					onduplicate={full ? undefined : () => duplicate(i)}
					onremove={() => set(removed(blocks, i))}
				/>
			{/snippet}
			{#if errors[`${path}.${i}`]}
				<p class="error">{errors[`${path}.${i}`]}</p>
			{/if}
			{#if comp}
				<Fields
					{schema}
					component={comp}
					values={block.fields}
					{errors}
					{refs}
					prefix="{path}.{i}."
					{changed}
				/>
			{/if}
		</ItemCard>
	{/each}

	{#if !full}
		<BlockMenu {types} onpick={t => add(t)} />
	{/if}
</div>

<style lang="scss">
	.blocks {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.tools {
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
