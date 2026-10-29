<script lang="ts">
	import { Editor } from '@tiptap/core';
	import StarterKit from '@tiptap/starter-kit';
	import { onDestroy, onMount } from 'svelte';
	import type { Field, Link, Refs, Schema, Values } from '../api';
	import { Block, Buttons, ZippLink } from './rich/extensions.svelte';
	import LinkField from './LinkField.svelte';

	/**
	 * Rich text with TipTap. The toolbar has only what the schema allows
	 * for this field; buttons and blocks are edited in boxes in the text.
	 */
	let {
		schema,
		field,
		values,
		refs,
	}: { schema: Schema; field: Field; values: Values; refs: Refs } = $props();

	type Opts = {
		headings: number[];
		lists: boolean;
		links: boolean;
		buttons: boolean;
		blocks: string[];
	};
	const opts: Opts = $derived(
		field.kind === 'richText'
			? field
			: {
					headings: [],
					lists: false,
					links: false,
					buttons: false,
					blocks: [],
				},
	);
	const blockTypes = $derived(
		schema.components.filter(c => opts.blocks.includes(c.handle)),
	);

	let el: HTMLDivElement;
	let editor = $state<Editor | null>(null);
	// the toolbar follows the selection
	let tick = $state(0);
	let linkEdit = $state<{ link: Link } | null>(null);
	let last = '';

	const EMPTY = { type: 'doc', content: [] };

	onMount(() => {
		const disabled = !!el.closest('fieldset[disabled]');
		editor = new Editor({
			element: el,
			editable: !disabled,
			// a plain copy, TipTap doesn't take Svelte's proxies
			content: $state.snapshot(values[field.handle]) ?? EMPTY,
			extensions: [
				StarterKit.configure({
					heading: opts.headings.length
						? { levels: opts.headings as any }
						: false,
					bulletList: opts.lists ? {} : false,
					orderedList: opts.lists ? {} : false,
					listItem: opts.lists ? {} : false,
					listKeymap: opts.lists ? {} : false,
					blockquote: false,
					code: false,
					codeBlock: false,
					horizontalRule: false,
					strike: false,
					underline: false,
					link: false,
				}),
				...(opts.links ? [ZippLink] : []),
				Buttons.configure({ schema, refs }),
				Block.configure({ schema, refs }),
			],
			onUpdate: ({ editor }) => {
				const doc = editor.getJSON();
				last = JSON.stringify(doc);
				values[field.handle] = doc;
			},
			// later: a transaction can come while Svelte renders (a blur)
			onTransaction: () => queueMicrotask(() => tick++),
			enableContentCheck: true,
			onContentError: ({ error }) => console.error('rich text:', error),
		});
		last = JSON.stringify(editor.getJSON());
	});

	// content that changes from outside (load, restore) goes into the editor
	$effect(() => {
		const doc = values[field.handle] ?? EMPTY;
		const json = JSON.stringify(doc);
		if (!editor || json === last) return;
		last = json;
		editor.commands.setContent($state.snapshot(doc), { emitUpdate: false });
	});

	onDestroy(() => editor?.destroy());

	function active(name: string, attrs?: Record<string, unknown>) {
		void tick;
		return editor?.isActive(name, attrs) ?? false;
	}

	function chain() {
		return editor!.chain().focus();
	}

	function openLink() {
		const current = editor?.getAttributes('link').link;
		linkEdit = { link: current ?? { type: 'url', value: '' } };
	}

	function applyLink() {
		if (!linkEdit) return;
		const link = $state.snapshot(linkEdit.link);
		linkEdit = null;
		if (link.value === null || link.value === '') {
			chain().extendMarkRange('link').unsetMark('link').run();
		} else {
			chain().extendMarkRange('link').setMark('link', { link }).run();
		}
	}

	function insertButtons() {
		chain()
			.insertContent({
				type: 'buttons',
				attrs: { links: [{ type: 'entry', value: null }] },
			})
			.run();
	}

	function insertBlock(type: string) {
		chain()
			.insertContent({
				type: 'embed',
				attrs: { id: null, type, fields: {} },
			})
			.run();
	}

	// what the link popup edits, like a link field without its own label
	const linkField: Field = {
		handle: 'link',
		label: 'Link',
		required: false,
		indexed: false,
		translatable: false,
		kind: 'link',
		to: ['entry', 'url', 'email', 'phone'],
		labels: false,
		newTab: true,
	};
</script>

<div class="rich" class:readonly={editor && !editor.isEditable}>
	{#if editor?.isEditable}
		<div class="toolbar">
			<button title="Undo" onclick={() => chain().undo().run()}>↶</button>
			<button title="Redo" onclick={() => chain().redo().run()}>↷</button>
			<button
				title="Clear formatting"
				onclick={() => chain().unsetAllMarks().setParagraph().run()}
			>
				⌫
			</button>
			<span class="sep"></span>
			{#each opts.headings as level (level)}
				<button
					class:on={active('heading', { level })}
					title="Heading {level}"
					onclick={() =>
						chain()
							.toggleHeading({ level: level as any })
							.run()}
				>
					H{level}
				</button>
			{/each}
			<button
				class:on={active('bold')}
				title="Bold"
				onclick={() => chain().toggleBold().run()}
			>
				<b>B</b>
			</button>
			<button
				class:on={active('italic')}
				title="Italic"
				onclick={() => chain().toggleItalic().run()}
			>
				<i>I</i>
			</button>
			{#if opts.lists}
				<button
					class:on={active('bulletList')}
					title="List"
					onclick={() => chain().toggleBulletList().run()}
				>
					•≡
				</button>
				<button
					class:on={active('orderedList')}
					title="Numbered list"
					onclick={() => chain().toggleOrderedList().run()}
				>
					1≡
				</button>
			{/if}
			{#if opts.links}
				<button
					class:on={active('link')}
					title="Link"
					onclick={openLink}
				>
					Link
				</button>
			{/if}
			{#if opts.buttons || blockTypes.length}
				<span class="sep"></span>
			{/if}
			{#if opts.buttons}
				<button title="A row of buttons" onclick={insertButtons}>
					+ Buttons
				</button>
			{/if}
			{#each blockTypes as b (b.handle)}
				<button onclick={() => insertBlock(b.handle)}>
					+ {b.name}
				</button>
			{/each}
		</div>
	{/if}

	{#if linkEdit}
		<div class="link-edit">
			<LinkField {schema} field={linkField} values={linkEdit} {refs} />
			<div class="actions">
				<button
					onclick={() => {
						linkEdit = { link: { type: 'url', value: null } };
						applyLink();
					}}
				>
					Remove link
				</button>
				<button class="dark" onclick={applyLink}>Apply</button>
			</div>
		</div>
	{/if}

	<div class="editor" bind:this={el}></div>
</div>

<style lang="scss">
	.rich {
		background: var(--input);
		border: 1px solid var(--border-strong);
		border-radius: 5px;

		&:focus-within {
			border-color: var(--accent);
			outline: 1px solid var(--accent);
		}
	}

	// a light bar over the text
	.toolbar {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 2px;
		padding: 3px 4px;
		background: var(--surface-2);
		border-bottom: 1px solid var(--line);
		border-radius: 5px 5px 0 0;

		button {
			padding: 3px 7px;
			border: none;
			background: none;
			color: var(--text-2);
			font-size: 12px;

			&.on {
				background: var(--accent-soft);
				color: var(--accent);
			}

			&:hover:not(:disabled) {
				background: var(--hover);
			}
		}

		.sep {
			width: 1px;
			height: 16px;
			margin: 0 4px;
			background: var(--border);
		}
	}

	.link-edit {
		padding: 12px;
		background: white;
		border: 1px solid var(--line);

		.actions {
			display: flex;
			justify-content: flex-end;
			gap: 8px;
			margin-top: 10px;
		}
	}

	.editor {
		padding: 4px 12px;
		min-height: 6rem;

		:global(.tiptap) {
			outline: none;
			min-height: 5rem;
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
			color: #2f5bea;
			text-decoration: underline;
		}

		:global(.ProseMirror-selectednode) {
			outline: 2px solid var(--dark);
		}
	}

	.readonly {
		opacity: 0.7;
	}
</style>
