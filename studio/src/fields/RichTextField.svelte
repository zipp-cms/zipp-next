<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import { onDestroy, onMount } from 'svelte';
	import type { Field, Schema, Values } from '../core/schema';
	import type { Refs } from '../entries/api';
	import { site } from '../core/router.svelte';
	import { Last } from '../core/synced.svelte';
	import Toolbar from './rich/Toolbar.svelte';
	import { EMPTY, createEditor, richOptions, trimmed } from './rich/editor';

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
	}: { schema: Schema; field: Field; values: Values; refs: Refs } = $props();

	const opts = $derived(richOptions(field));
	const blockTypes = $derived(
		schema.components.filter(c => opts.blocks.includes(c.handle)),
	);

	let el: HTMLDivElement;
	let editor = $state<Editor | null>(null);
	// the toolbar follows the selection
	let tick = $state(0);
	// the content as last sent or received, to not send it back
	const last = new Last();

	onMount(() => {
		editor = createEditor({
			element: el,
			opts,
			schema,
			refs,
			editable: !el.closest('fieldset[disabled]'),
			language:
				schema.sites.find(s => s.handle === site.current)?.language ??
				'',
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
		const doc = values[field.handle] ?? EMPTY;
		if (!editor || !last.changed(trimmed(doc))) return;
		editor.commands.setContent($state.snapshot(doc), { emitUpdate: false });
	});

	onDestroy(() => editor?.destroy());
</script>

<div class="rich" class:readonly={editor && !editor.isEditable}>
	{#if editor?.isEditable}
		<Toolbar {editor} {tick} {opts} {blockTypes} {schema} {refs} />
	{/if}
	<div class="editor" bind:this={el}></div>
</div>

<style lang="scss">
	.rich {
		background: var(--input);
		border: 1px solid var(--border-strong);
		border-radius: var(--radius);

		&:focus-within {
			border-color: var(--accent);
			outline: 1px solid var(--accent);
		}
	}

	.editor {
		padding: 4px 12px;
		min-height: 6rem;

		:global(.tiptap) {
			outline: none;
			min-height: 5rem;
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
			outline: 2px solid var(--accent);
			border-radius: var(--radius);
		}
	}

	.readonly {
		opacity: 0.7;
	}
</style>
