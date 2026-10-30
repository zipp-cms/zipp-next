<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import { onDestroy, onMount } from 'svelte';
	import type { FieldOf, Schema, Values } from '../core/schema';
	import { valueOf } from '../core/schema';
	import type { Refs } from '../entries/api';
	import { editedSite } from '../core/editing';
	import { Last } from '../core/synced.svelte';
	import Toolbar from './rich/Toolbar.svelte';
	import LinkBubble from './rich/LinkBubble.svelte';
	import { EMPTY, createEditor, moveNode, trimmed } from './rich/editor';
	import { Sortable } from 'zipp-ui/sortable';
	import SortLine from 'zipp-ui/SortLine.svelte';

	/**
	 * Rich text with TipTap. The toolbar has only what the schema allows
	 * for this field; buttons and blocks show as cards in the text that
	 * open to be edited. The empty line at the end invites to continue.
	 */
	let {
		schema,
		field,
		values,
		refs,
	}: {
		schema: Schema;
		field: FieldOf<'richText'>;
		values: Values;
		refs: Refs;
	} = $props();

	// what the toolbar offers
	const opts = $derived(field);
	const blockTypes = $derived(
		schema.components.filter(c => opts.blocks.includes(c.handle)),
	);

	let el: HTMLDivElement;
	let editor = $state<Editor | null>(null);
	// the toolbar follows the selection
	let tick = $state(0);
	// the content as last sent or received, to not send it back
	const last = new Last();
	// for the spell check of the browser
	const edited = editedSite();
	const language = $derived(
		schema.sites.find(s => s.handle === edited)?.language ?? '',
	);

	// moving buttons and blocks: the nodes of the text are the rows
	const sort = new Sortable({
		container: () => el,
		rows: () =>
			editor ? ([...editor.view.dom.children] as HTMLElement[]) : [],
		depths: () =>
			editor ? Array(editor.state.doc.childCount).fill(0) : [],
		// the empty line at the end stays last
		keepLast: () => 1,
		onmove: (from, drop) => editor && moveNode(editor, from, drop.index),
	});

	onMount(() => {
		editor = createEditor({
			sortable: sort,
			element: el,
			opts,
			schema,
			refs,
			editable: !el.closest('fieldset[disabled]'),
			language,
			// a plain copy, TipTap doesn't take Svelte's proxies
			content: $state.snapshot(values[field.handle]),
			onChange: doc => {
				if (last.changed(doc)) values[field.handle] = doc;
			},
			// later: a transaction can come while Svelte renders (a blur)
			onTransaction: () => queueMicrotask(() => tick++),
		});
		last.changed(trimmed(editor.getJSON()));
	});

	// content that changes from outside (load, restore) goes into the editor
	$effect(() => {
		const doc = valueOf(values, field) ?? EMPTY;
		if (!editor || !last.changed(trimmed(doc))) return;
		editor.commands.setContent($state.snapshot(doc), { emitUpdate: false });
	});

	onDestroy(() => editor?.destroy());
</script>

<div class="rich" class:readonly={editor && !editor.isEditable}>
	{#if editor?.isEditable}
		<Toolbar {editor} {tick} {opts} {blockTypes} {schema} {refs} />
	{/if}
	<div class="editor" bind:this={el}><SortLine sortable={sort} /></div>
	{#if editor && opts.links}
		<LinkBubble {editor} {tick} {schema} {refs} />
	{/if}
</div>

<style lang="scss">
	.rich {
		position: relative;
		background: var(--input);
		border: 1px solid var(--border-strong);
		border-radius: var(--radius);

		&:focus-within {
			border-color: var(--accent);
			outline: 1px solid var(--accent);
		}
	}

	.editor {
		position: relative;
		padding: 0.75rem;
		min-height: 5.25rem;

		:global(.tiptap) {
			outline: none;
			min-height: 4.375rem;
		}

		// the invitation to write: in an empty text, else only at the end
		:global(.tiptap p.is-editor-empty:first-child::before),
		:global(.tiptap > p.is-empty:last-child:not(:first-child)::before) {
			content: attr(data-placeholder);
			float: left;
			height: 0;
			color: var(--subtle);
			pointer-events: none;
		}

		:global(.tiptap > * + *) {
			margin-top: 0.7em;
		}

		:global(.tiptap p),
		:global(.tiptap h2),
		:global(.tiptap h3),
		:global(.tiptap ul),
		:global(.tiptap ol) {
			margin-bottom: 0;
		}

		:global(.rt-link) {
			color: var(--accent);
			text-decoration: underline;
		}

		:global(.ProseMirror-selectednode) {
			@include focus-ring(0);
			border-radius: var(--radius);
		}
	}

	.readonly {
		opacity: 0.7;
	}
</style>
