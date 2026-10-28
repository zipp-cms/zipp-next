<script lang="ts">
	import type { Block, Field, Refs, Schema, Values } from '../api';
	import BlockMenu from './BlockMenu.svelte';
	import Fields from './Fields.svelte';

	let {
		schema,
		field,
		values,
		errors,
		refs,
		path,
	}: {
		schema: Schema;
		field: Field;
		values: Values;
		errors: Record<string, string>;
		refs: Refs;
		path: string;
	} = $props();

	const types = $derived(
		schema.components.filter(
			c => field.kind === 'blocks' && field.types.includes(c.handle),
		),
	);
	const max = $derived(field.kind === 'blocks' ? field.max : null);
	const blocks = $derived<Block[]>(values[field.handle] ?? []);

	// blocks without an id are new, they get one on save
	let keys = new WeakMap<Block, number>();
	let nextKey = 0;
	function key(b: Block) {
		if (!keys.has(b)) keys.set(b, nextKey++);
		return keys.get(b)!;
	}

	let collapsed = $state<Record<number, boolean>>({});

	function add(type: string) {
		values[field.handle] = [...blocks, { type, fields: {} }];
	}

	function move(i: number, by: number) {
		const list = [...blocks];
		[list[i], list[i + by]] = [list[i + by], list[i]];
		values[field.handle] = list;
	}

	function remove(i: number) {
		values[field.handle] = blocks.filter((_, j) => j !== i);
	}

	/** The first text of a block, shown when it is collapsed. */
	function summary(b: Block) {
		const text = Object.values(b.fields).find(v => typeof v === 'string');
		return text ? text.slice(0, 60) : '';
	}
</script>

<div class="blocks">
	{#each blocks as block, i (key(block))}
		{@const comp = schema.components.find(c => c.handle === block.type)}
		{@const k = key(block)}
		<div class="block">
			<div class="head">
				<button
					class="icon name"
					onclick={() => (collapsed[k] = !collapsed[k])}
				>
					{collapsed[k] ? '▸' : '▾'}
					{comp?.name ?? block.type}
					{#if collapsed[k]}<span class="muted">
							{summary(block)}
						</span>{/if}
				</button>
				<button
					class="icon"
					title="Move up"
					disabled={i === 0}
					onclick={() => move(i, -1)}
				>
					↑
				</button>
				<button
					class="icon"
					title="Move down"
					disabled={i === blocks.length - 1}
					onclick={() => move(i, 1)}
				>
					↓
				</button>
				<button class="icon" title="Remove" onclick={() => remove(i)}>
					×
				</button>
			</div>
			{#if errors[`${path}.${i}`]}
				<p class="error">{errors[`${path}.${i}`]}</p>
			{/if}
			{#if comp && !collapsed[k]}
				<div class="body">
					<Fields
						{schema}
						component={comp}
						values={block.fields}
						{errors}
						{refs}
						prefix="{path}.{i}."
					/>
				</div>
			{/if}
		</div>
	{/each}

	{#if max === null || blocks.length < max}
		<BlockMenu {types} onpick={add} />
	{/if}
</div>

<style lang="scss">
	.blocks {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.block {
		border: 1px solid var(--line);
		border-radius: 0.4rem;
	}

	.head {
		display: flex;
		align-items: center;
		padding: 0.3rem 0.4rem;
		background: var(--side);
		border-radius: 0.4rem 0.4rem 0 0;

		.name {
			flex: 1;
			text-align: left;
			color: var(--text);
			font-weight: 500;
			display: flex;
			gap: 0.5rem;
		}
	}

	.body {
		padding: 0.9rem;
	}

	.muted {
		color: var(--muted);
		font-weight: normal;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}
</style>
