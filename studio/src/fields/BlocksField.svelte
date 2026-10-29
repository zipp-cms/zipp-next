<script lang="ts">
	import { untrack } from 'svelte';
	import { moved } from '../lib/list';
	import type { Block, Field, Refs, Schema, Values } from '../api';
	import BlockMenu from './BlockMenu.svelte';
	import Fields from './Fields.svelte';
	import Icon from '../lib/Icon.svelte';
	import { blockSummary, focus } from '../lib/blocks.svelte';

	/**
	 * Blocks as rows that open: the type, a summary when closed, the
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
	const full = $derived(max !== null && blocks.length >= max);

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
	const isOpen = (b: Block) => open[key(b)] ?? startOpen;

	function setAll(to: boolean) {
		for (const b of blocks) open[key(b)] = to;
	}

	function add(type: string, at = blocks.length) {
		const block: Block = { type, fields: {} };
		open[key(block)] = true;
		const next = [...blocks];
		next.splice(at, 0, block);
		values[field.handle] = next;
	}

	const move = (i: number, by: number) =>
		(values[field.handle] = moved(blocks, i, by));

	function duplicate(i: number) {
		const copy: Block = $state.snapshot(blocks[i]);
		delete copy.id;
		open[key(copy)] = true;
		const next = [...blocks];
		next.splice(i + 1, 0, copy);
		values[field.handle] = next;
	}

	function remove(i: number) {
		values[field.handle] = blocks.filter((_, j) => j !== i);
	}

	/** Whether a block or something inside it has an error. */
	const hasError = (i: number) =>
		Object.keys(errors).some(
			p => p === `${path}.${i}` || p.startsWith(`${path}.${i}.`),
		);

	// the outline of the editor opens a block and scrolls to it
	let rows: HTMLElement[] = [];
	$effect(() => {
		if (focus.path !== path || !focus.at) return;
		const b = blocks[focus.index];
		if (!b) return;
		open[key(b)] = true;
		setTimeout(() =>
			rows[focus.index]?.scrollIntoView({
				behavior: 'smooth',
				block: 'start',
			}),
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
		{@const comp = schema.components.find(c => c.handle === block.type)}
		{@const shown = isOpen(block)}
		{#if i > 0 && !full}
			<BlockMenu {types} inline onpick={t => add(t, i)} />
		{/if}
		<div
			class="block"
			class:open={shown}
			class:invalid={hasError(i)}
			bind:this={rows[i]}
			id="block-{path}-{i}"
		>
			<div class="head">
				<button
					type="button"
					class="toggle"
					aria-expanded={shown}
					onclick={() => (open[key(block)] = !shown)}
				>
					{#if changed.has(`${path}.${i}`)}<span
							class="dot"
							title="New"
						></span>{/if}
					<span class="type">{comp?.name ?? block.type}</span>
					{#if !shown}
						<span class="summary">
							{blockSummary(schema, comp, block.fields)}
						</span>
					{/if}
					{#if hasError(i)}
						<span class="flag" title="Has errors">
							<Icon name="alert" size={14} />
						</span>
					{/if}
				</button>
				<span class="actions">
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
						disabled={i === blocks.length - 1}
						onclick={() => move(i, 1)}
					>
						<Icon name="arrow-down" size={15} />
					</button>
					{#if !full}
						<button
							type="button"
							class="icon"
							title="Duplicate"
							onclick={() => duplicate(i)}
						>
							<Icon name="copy" size={15} />
						</button>
					{/if}
					<button
						type="button"
						class="icon delete"
						title="Remove"
						onclick={() => remove(i)}
					>
						<Icon name="delete" size={15} />
					</button>
				</span>
				<button
					type="button"
					class="icon chevron"
					title={shown ? 'Collapse' : 'Expand'}
					onclick={() => (open[key(block)] = !shown)}
				>
					<Icon name={shown ? 'chevron-down' : 'chevron-right'} />
				</button>
			</div>
			{#if errors[`${path}.${i}`]}
				<p class="error">{errors[`${path}.${i}`]}</p>
			{/if}
			{#if comp && shown}
				<div class="body">
					<Fields
						{schema}
						component={comp}
						values={block.fields}
						{errors}
						{refs}
						prefix="{path}.{i}."
						{changed}
					/>
				</div>
			{/if}
		</div>
	{/each}

	{#if !full}
		<BlockMenu {types} onpick={t => add(t)} />
	{/if}
</div>

<style lang="scss">
	.blocks {
		display: flex;
		flex-direction: column;
	}

	.tools {
		display: flex;
		align-items: center;
		gap: 4px;
		margin-bottom: 6px;
		color: var(--muted);
		font-size: 12px;

		span {
			margin-right: auto;
		}
	}

	.block {
		margin: 3px 0;
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: 6px;
		scroll-margin-top: 64px;

		&.invalid {
			border-color: var(--danger);
		}
	}

	.head {
		display: flex;
		align-items: center;
		gap: 2px;
		min-height: 38px;
		padding: 0 6px 0 4px;
	}

	.open .head {
		border-bottom: 1px solid var(--line);
	}

	.toggle {
		display: flex;
		align-items: center;
		gap: 8px;
		flex: 1;
		min-width: 0;
		padding: 6px;
		border: none;
		background: none;
		text-align: left;

		&:hover:not(:disabled) {
			background: none;
		}
	}

	.type {
		flex-shrink: 0;
		padding: 1px 7px;
		border-radius: 4px;
		background: var(--accent-soft);
		color: var(--accent);
		font-size: 11px;
		font-weight: 600;
	}

	.summary {
		overflow: hidden;
		color: var(--text-2);
		font-weight: 400;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.flag {
		display: inline-flex;
		color: var(--danger);
	}

	// the actions show on hover and for the keyboard
	.actions {
		display: flex;
		opacity: 0;
		transition: opacity 0.15s;
	}

	.head:hover .actions,
	.actions:focus-within {
		opacity: 1;
	}

	.delete:hover:not(:disabled) {
		color: var(--danger);
	}

	.body {
		padding: 14px 14px 16px;
	}

	@media (hover: none) {
		.actions {
			opacity: 1;
		}
	}
</style>
