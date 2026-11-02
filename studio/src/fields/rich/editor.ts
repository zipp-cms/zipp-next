import { Editor } from '@tiptap/core';
import StarterKit from '@tiptap/starter-kit';
import { Placeholder } from '@tiptap/extensions';
import {
	defaults,
	type FieldOf,
	type Link,
	type Schema,
	type Style,
} from '../../core/schema';
import type { Refs } from '../../entries/api';
import {
	Block,
	BlockStyles,
	Button,
	EndLine,
	InlineStyle,
	ZippLink,
} from './extensions.svelte';
import type { Sortable } from 'zipp-ui/sortable';
import { t } from '../../i18n/index.svelte';

/** What a rich text field allows: its field. */
export type RichOptions = Omit<FieldOf<'richText'>, 'kind'>;

/** An empty text: TipTap needs a paragraph to write in. */
export const EMPTY = { type: 'doc', content: [{ type: 'paragraph' }] };

/**
 * Without the empty paragraphs at the end: TipTap keeps one after a card
 * to write on, it is no change of the content.
 */
export function trimmed(doc: any) {
	const content = [...(doc.content ?? [])];
	while (
		content.length &&
		content[content.length - 1].type === 'paragraph' &&
		!content[content.length - 1].content?.length
	)
		content.pop();
	return { ...doc, content };
}

/** The TipTap editor of a field, with only what the field allows. */
export function createEditor(o: {
	element: HTMLElement;
	opts: RichOptions;
	schema: Schema;
	refs: Refs;
	editable: boolean;
	/** of the site, so spellcheck checks the right one */
	language: string;
	/** the field's, the name of the text area for screen readers */
	label: string;
	content: unknown;
	onChange: (doc: unknown) => void;
	onTransaction: () => void;
	/** moves the buttons and blocks of the text */
	sortable: Sortable;
}): Editor {
	const { opts } = o;
	const nodeOptions = {
		schema: o.schema,
		refs: () => o.refs,
		sortable: () => o.sortable,
	};
	return new Editor({
		element: o.element,
		editable: o.editable,
		editorProps: {
			attributes: {
				lang: o.language,
				role: 'textbox',
				'aria-multiline': 'true',
				'aria-label': o.label,
			},
		},
		content: (o.content ?? EMPTY) as any,
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
			...(opts.styles?.length
				? [
						BlockStyles.configure({ styles: opts.styles }),
						InlineStyle.configure({ styles: opts.styles }),
					]
				: []),
			// css shows it only at the end, see .tiptap
			Placeholder.configure({
				placeholder: ({ editor }) =>
					editor.isEmpty
						? t('fields.writePlaceholder')
						: t('fields.continueWriting'),
				showOnlyCurrent: false,
			}),
			EndLine,
			Button.configure(nodeOptions),
			Block.configure(nodeOptions),
		],
		// a transaction, so the plugins add what they add: the empty line
		// at the end shows without a click
		onCreate: ({ editor }) =>
			editor.view.dispatch(
				editor.state.tr.setMeta('addToHistory', false),
			),
		onUpdate: ({ editor }) => o.onChange(trimmed(editor.getJSON())),
		onTransaction: o.onTransaction,
		enableContentCheck: true,
		onContentError: ({ error }) => console.error('rich text:', error),
	});
}

const chain = (editor: Editor) => editor.chain().focus();

/**
 * Moves the node `from` of the text before the node `to` of the text
 * without it (its count: to the end); one step to undo.
 */
export function moveNode(editor: Editor, from: number, to: number) {
	const { state } = editor;
	const start = (doc: typeof state.doc, i: number) => {
		let pos = 0;
		for (let k = 0; k < i; k++) pos += doc.child(k).nodeSize;
		return pos;
	};
	const node = state.doc.child(from);
	const at = start(state.doc, from);
	const tr = state.tr.delete(at, at + node.nodeSize);
	tr.insert(start(tr.doc, to), node);
	editor.view.dispatch(tr.scrollIntoView());
}

/** Sets the link of the selection, an empty one removes it. */
export function setLink(editor: Editor, link: Link) {
	if (link.value === null || link.value === '') {
		chain(editor).extendMarkRange('link').unsetMark('link').run();
	} else {
		chain(editor).extendMarkRange('link').setMark('link', { link }).run();
	}
}

/** A button without a target yet, to be filled in. */
export function insertButton(editor: Editor) {
	chain(editor)
		.insertContent({
			type: 'button',
			attrs: { link: { type: 'entry', value: null } },
		})
		.run();
}

/** A new block, with the defaults of its fields. */
export function insertBlock(editor: Editor, schema: Schema, type: string) {
	const fields = defaults(schema, type);
	chain(editor)
		.insertContent({ type: 'embed', attrs: { id: null, type, fields } })
		.run();
}

export function setFormat(editor: Editor, level: number) {
	if (level)
		chain(editor)
			.setHeading({ level: level as any })
			.run();
	else chain(editor).setParagraph().run();
}

/** Whether the selection has the style. */
export function hasStyle(editor: Editor, style: Style) {
	const attrs = { class: style.class };
	if (style.inline) return editor.isActive('style', attrs);
	return (
		editor.isActive('paragraph', attrs) || editor.isActive('heading', attrs)
	);
}

/**
 * Gives the selection the style or takes it away: an inline style the
 * selected text, a block style its paragraphs and headings.
 */
export function toggleStyle(editor: Editor, style: Style) {
	if (style.inline) {
		chain(editor).toggleMark('style', { class: style.class }).run();
		return;
	}
	const attrs = { class: hasStyle(editor, style) ? null : style.class };
	let c = chain(editor).updateAttributes('paragraph', attrs);
	// a field without headings has no such node
	if (editor.schema.nodes.heading) c = c.updateAttributes('heading', attrs);
	c.run();
}
