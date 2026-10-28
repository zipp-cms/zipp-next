<script lang="ts">
	import { moved } from '../lib/list';
	import type { Block, Field, Refs, Schema, Values } from '../api';
	import BlockMenu from './BlockMenu.svelte';
	import Fields from './Fields.svelte';
	import Icon from '../lib/Icon.svelte';

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
		field: Field;
		values: Values;
		errors: Record<string, string>;
		refs: Refs;
		path: string;
		changed?: Set<string>;
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

	const move = (i: number, by: number) =>
		(values[field.handle] = moved(blocks, i, by));

	function remove(i: number) {
		values[field.handle] = blocks.filter((_, j) => j !== i);
	}

	/** The first text of a block, shown when it is collapsed. */
	function summary(b: Block) {
		const text = Object.values(b.fields).find(v => typeof v === 'string');
		return text ? text.slice(0, 60) : '';
	}
</script>

<!-- like the repeat fields of the old Zipp: one after the other, the
	actions on the right -->
<div class="blocks">
	{#each blocks as block, i (key(block))}
		{@const comp = schema.components.find(c => c.handle === block.type)}
		{@const k = key(block)}
		<div class="block">
			<div class="head">
				<button
					class="link name"
					onclick={() => (collapsed[k] = !collapsed[k])}
				>
					{#if changed.has(`${path}.${i}`)}<span
							class="dot"
							title="New"
						></span>{/if}{comp?.name ?? block.type}
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
					<Icon name="delete" />
				</button>
			</div>
			{#if errors[`${path}.${i}`]}
				<p class="error">{errors[`${path}.${i}`]}</p>
			{/if}
			{#if comp && !collapsed[k]}
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
	}

	.block {
		padding: 10px 0 16px;
		border-bottom: 2px solid var(--line);

		&:first-child {
			padding-top: 0;
		}
	}

	.head {
		display: flex;
		align-items: center;
		margin-bottom: 8px;

		.name {
			flex: 1;
			gap: 10px;
			padding: 4px 0;
			color: var(--text);
			font-weight: 600;
			text-align: left;
			min-width: 0;
		}
	}

	.dot {
		display: inline-block;
		width: 8px;
		height: 8px;
		margin-right: -4px;
		border-radius: 50%;
		background: #d98e04;
	}

	.muted {
		color: var(--muted);
		font-weight: normal;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}
</style>
