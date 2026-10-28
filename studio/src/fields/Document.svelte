<script lang="ts">
	import { tick } from 'svelte';
	import type { Block, Component, Field, Refs, Schema, Values } from '../api';
	import AutoText from './AutoText.svelte';
	import BlockMenu from './BlockMenu.svelte';
	import Fields from './Fields.svelte';

	/**
	 * A blocks field edited like a document. Blocks with a single text
	 * field (multiline or a heading) are lines of text: Enter starts a new
	 * one, Backspace at the start joins it with the one before and `/`
	 * turns an empty one into another block. Other blocks show their fields as a card. The
	 * handle on the left moves, deletes and converts blocks.
	 */
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
		field.kind === 'blocks'
			? field.types.map(t => schema.components.find(c => c.handle === t)!)
			: [],
	);
	const max = $derived(field.kind === 'blocks' ? field.max : null);
	const blocks = $derived<Block[]>(values[field.handle] ?? []);
	const full = $derived(max !== null && blocks.length >= max);
	// what Enter creates: a line of text, a paragraph before a heading
	const lineType = $derived(
		types.find(t => isLine(t) && !t.fields[0].heading) ??
			types.find(isLine) ??
			null,
	);

	function comp(b: Block) {
		return schema.components.find(c => c.handle === b.type);
	}

	/** A block that is only text to write, like a paragraph or a heading. */
	function isLine(c: Component | undefined) {
		const f = c?.fields.length === 1 ? c.fields[0] : null;
		return f?.kind === 'text' && (f.multiline || f.heading);
	}

	function lineField(b: Block) {
		return comp(b)!.fields[0];
	}

	// new blocks have no id until they are saved
	const keys = new WeakMap<Block, number>();
	let nextKey = 0;
	function key(b: Block) {
		if (!keys.has(b)) keys.set(b, nextKey++);
		return keys.get(b)!;
	}

	let lines = $state<Record<number, HTMLTextAreaElement>>({});
	let rows = $state<Record<number, HTMLElement>>({});

	/** The open block menu: where it inserts or which block it replaces. */
	let menu = $state<{
		at: number;
		replace: boolean;
		query: string;
		index: number;
		search: boolean;
	} | null>(null);
	/** The block whose handle menu is open. */
	let actions = $state<number | null>(null);
	let dragFrom = $state<number | null>(null);
	let dropAt = $state<number | null>(null);

	const found = $derived(
		menu
			? types.filter(t =>
					t.name.toLowerCase().includes(menu!.query.toLowerCase()),
				)
			: [],
	);

	function set(list: Block[]) {
		values[field.handle] = list;
	}

	function newBlock(type: string, text = ''): Block {
		const c = schema.components.find(c => c.handle === type)!;
		const fields: Values = {};
		if (isLine(c) && text) fields[c.fields[0].handle] = text;
		return { type, fields };
	}

	/** Focuses block i, a line at the given caret position (end if null). */
	async function focus(i: number, caret: number | null = null) {
		await tick();
		const b = blocks[i];
		if (!b) return;
		const line = lines[key(b)];
		if (line) {
			line.focus();
			const pos = caret ?? line.value.length;
			line.setSelectionRange(pos, pos);
			return;
		}
		rows[key(b)]?.querySelector<HTMLElement>('textarea, input')?.focus();
	}

	function insert(at: number, block: Block) {
		if (full) return;
		set([...blocks.slice(0, at), block, ...blocks.slice(at)]);
	}

	function remove(i: number) {
		set(blocks.filter((_, j) => j !== i));
	}

	function move(from: number, to: number) {
		const list = [...blocks];
		const [b] = list.splice(from, 1);
		list.splice(to > from ? to - 1 : to, 0, b);
		set(list);
	}

	/** The `+` next to a block or below the last one. */
	function add(at: number) {
		actions = null;
		if (full) return;
		if (lineType) {
			insert(at, newBlock(lineType.handle, '/'));
			menu = { at, replace: true, query: '', index: 0, search: false };
			focus(at);
		} else {
			menu = { at, replace: false, query: '', index: 0, search: true };
		}
	}

	function pick(c: Component) {
		if (!menu) return;
		const { at, replace } = menu;
		menu = null;
		if (replace) {
			const list = [...blocks];
			list[at] = newBlock(c.handle);
			set(list);
		} else {
			insert(at, newBlock(c.handle));
		}
		focus(at, 0);
	}

	/** Turns a line into another kind of line and keeps its text. */
	function turnInto(i: number, c: Component) {
		actions = null;
		const text = blocks[i].fields[lineField(blocks[i]).handle] ?? '';
		const list = [...blocks];
		list[i] = newBlock(c.handle, text);
		set(list);
		focus(i);
	}

	function oninput(i: number, e: Event) {
		const text = (e.target as HTMLTextAreaElement).value;
		if (text.startsWith('/') && !text.includes(' ')) {
			const query = text.slice(1);
			if (menu?.at !== i || menu.query !== query) {
				menu = { at: i, replace: true, query, index: 0, search: false };
			}
		} else if (menu?.at === i) {
			menu = null;
		}
	}

	function onkeydown(i: number, e: KeyboardEvent) {
		const el = e.target as HTMLTextAreaElement;
		const b = blocks[i];
		const handle = lineField(b).handle;

		if (menu?.at === i && !menu.search) {
			if (e.key === 'ArrowDown' || e.key === 'ArrowUp') {
				e.preventDefault();
				const by = e.key === 'ArrowDown' ? 1 : -1;
				const n = found.length || 1;
				menu.index = (menu.index + by + n) % n;
				return;
			}
			if (e.key === 'Enter') {
				e.preventDefault();
				if (found[menu.index]) pick(found[menu.index]);
				return;
			}
			if (e.key === 'Escape') {
				menu = null;
				return;
			}
		}

		const start = el.selectionStart;
		const end = el.selectionEnd;
		const text = el.value;

		if (e.key === 'Enter' && !e.shiftKey && lineType && !full) {
			// the text after the caret moves into the new line
			e.preventDefault();
			b.fields[handle] = text.slice(0, start);
			insert(i + 1, newBlock(lineType.handle, text.slice(end)));
			focus(i + 1, 0);
		} else if (e.key === 'Backspace' && start === 0 && end === 0) {
			const prev = blocks[i - 1];
			if (!text) {
				e.preventDefault();
				remove(i);
				if (i > 0) focus(i - 1);
			} else if (prev && isLine(comp(prev))) {
				e.preventDefault();
				const ph = lineField(prev).handle;
				const before = prev.fields[ph] ?? '';
				prev.fields[ph] = before + text;
				remove(i);
				focus(i - 1, before.length);
			}
		} else if (e.key === 'ArrowUp' && start === 0 && i > 0) {
			e.preventDefault();
			focus(i - 1);
		} else if (e.key === 'ArrowDown' && end === text.length) {
			if (i < blocks.length - 1) {
				e.preventDefault();
				focus(i + 1, 0);
			}
		}
	}

	function ondragover(i: number, e: DragEvent) {
		if (dragFrom === null) return;
		e.preventDefault();
		const row = e.currentTarget as HTMLElement;
		const r = row.getBoundingClientRect();
		dropAt = e.clientY < r.top + r.height / 2 ? i : i + 1;
	}

	function ondrop(e: DragEvent) {
		e.preventDefault();
		if (dragFrom !== null && dropAt !== null) move(dragFrom, dropAt);
		dragFrom = null;
		dropAt = null;
	}
</script>

<svelte:window
	onclick={() => {
		actions = null;
		if (menu?.search) menu = null;
	}}
/>

<div class="doc">
	{#each blocks as block, i (key(block))}
		{@const c = comp(block)}
		{@const k = key(block)}
		<div
			class="row"
			class:drop-before={dropAt === i}
			class:drop-after={dropAt === i + 1 && i === blocks.length - 1}
			bind:this={rows[k]}
			ondragover={e => ondragover(i, e)}
			{ondrop}
			role="listitem"
		>
			<div class="gutter">
				<button
					class="icon"
					title="Add a block below"
					onclick={e => {
						e.stopPropagation();
						add(i + 1);
					}}
				>
					+
				</button>
				<button
					class="icon handle"
					title="Drag to move, click for more"
					draggable="true"
					ondragstart={() => (dragFrom = i)}
					ondragend={() => {
						dragFrom = null;
						dropAt = null;
					}}
					onclick={e => {
						e.stopPropagation();
						actions = actions === i ? null : i;
					}}
				>
					⋮⋮
				</button>
				{#if actions === i}
					<div class="popup actions">
						<button
							disabled={i === 0}
							onclick={() => move(i, i - 1)}
						>
							Move up
						</button>
						<button
							disabled={i === blocks.length - 1}
							onclick={() => move(i, i + 2)}
						>
							Move down
						</button>
						{#if isLine(c)}
							{#each types.filter(t => isLine(t) && t !== c) as t (t.handle)}
								<button onclick={() => turnInto(i, t)}>
									Turn into {t.name}
								</button>
							{/each}
						{/if}
						<button class="danger" onclick={() => remove(i)}>
							Delete
						</button>
					</div>
				{/if}
			</div>

			<div class="content">
				{#if c && isLine(c)}
					{@const f = c.fields[0]}
					<AutoText
						bind:el={lines[k]}
						bind:value={block.fields[f.handle]}
						placeholder={i === blocks.length - 1 && !f.heading
							? "Type '/' for blocks"
							: c.name}
						heading={f.heading}
						maxlength={f.kind === 'text' ? f.max : null}
						onkeydown={e => onkeydown(i, e)}
						oninput={e => oninput(i, e)}
					/>
					{#if errors[`${path}.${i}.${f.handle}`]}
						<p class="error">
							{c.name}: {errors[`${path}.${i}.${f.handle}`]}
						</p>
					{/if}
				{:else if c}
					<div class="card">
						<div class="type">{c.name}</div>
						<Fields
							{schema}
							component={c}
							values={block.fields}
							{errors}
							{refs}
							prefix="{path}.{i}."
						/>
					</div>
				{/if}
				{#if errors[`${path}.${i}`]}
					<p class="error">{errors[`${path}.${i}`]}</p>
				{/if}

				{#if menu?.at === i && !menu.search}
					<BlockMenu
						items={found}
						bind:index={menu.index}
						onpick={pick}
					/>
				{:else if menu?.search && menu.at === i + 1 && i < blocks.length - 1}
					<BlockMenu
						items={found}
						search
						bind:query={menu.query}
						bind:index={menu.index}
						onpick={pick}
					/>
				{/if}
			</div>
		</div>
	{/each}

	{#if !full}
		<div class="row end">
			<button
				class="add"
				onclick={e => {
					e.stopPropagation();
					add(blocks.length);
				}}
			>
				{blocks.length
					? '+ Add a block'
					: "Empty, click to write or type '/'"}
			</button>
			{#if menu?.search && menu.at === blocks.length}
				<BlockMenu
					items={found}
					search
					bind:query={menu.query}
					bind:index={menu.index}
					onpick={pick}
				/>
			{/if}
		</div>
	{/if}
</div>

<style lang="scss">
	.doc {
		display: flex;
		flex-direction: column;
	}

	.row {
		position: relative;
		display: flex;
		gap: 0.25rem;
		padding: 0.05rem 0;
		border-top: 2px solid transparent;
		border-bottom: 2px solid transparent;

		&.drop-before {
			border-top-color: var(--accent);
		}

		&.drop-after {
			border-bottom-color: var(--accent);
		}

		&:hover > .gutter,
		&:focus-within > .gutter {
			opacity: 1;
		}
	}

	.gutter {
		position: relative;
		display: flex;
		align-items: flex-start;
		justify-content: flex-end;
		width: 3.25rem;
		margin-left: -3.5rem;
		padding-right: 0.25rem;
		box-sizing: border-box;
		padding-top: 0.15rem;
		opacity: 0;
		transition: opacity 0.1s;

		.icon {
			padding: 0.1rem 0.3rem;
			border-radius: 0.25rem;
			line-height: 1.2;
		}

		.handle {
			cursor: grab;
			letter-spacing: -0.15em;
		}
	}

	.content {
		position: relative;
		flex: 1;
		min-width: 0;
	}

	.card {
		margin: 0.35rem 0;
		padding: 0.5rem 0.75rem 0.6rem;
		background: var(--side);
		border-radius: 0.4rem;

		.type {
			font-size: 0.8rem;
			color: var(--muted);
			margin-bottom: 0.2rem;
		}
	}

	.popup {
		position: absolute;
		z-index: 4;
		top: 100%;
		left: 0;
		min-width: 12rem;
		padding: 0.3rem;
		background: white;
		border: 1px solid var(--line);
		border-radius: 0.4rem;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
		display: flex;
		flex-direction: column;

		button {
			border: none;
			text-align: left;
			padding: 0.35rem 0.6rem;
		}
	}

	.end {
		margin-top: 0.2rem;
	}

	.add {
		border: none;
		padding: 0.25rem 0;
		color: var(--muted);
		background: none;
		opacity: 0.6;

		&:hover {
			opacity: 1;
		}
	}

	@media (max-width: 700px) {
		.gutter {
			opacity: 1;
			margin-left: 0;
			width: auto;
			flex-direction: column;
		}
	}
</style>
