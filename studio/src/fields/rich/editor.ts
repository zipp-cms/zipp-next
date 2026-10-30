import { Editor } from '@tiptap/core';
import StarterKit from '@tiptap/starter-kit';
import { Placeholder } from '@tiptap/extensions';
import type { FieldOf, Link, Schema } from '../../core/schema';
import type { Refs } from '../../entries/api';
import { Block, Button, ZippLink } from './extensions.svelte';

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
	content: unknown;
	onChange: (doc: unknown) => void;
	onTransaction: () => void;
}): Editor {
	const { opts } = o;
	return new Editor({
		element: o.element,
		editable: o.editable,
		editorProps: { attributes: { lang: o.language } },
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
			// css shows it only at the end, see .tiptap
			Placeholder.configure({
				placeholder: ({ editor }) =>
					editor.isEmpty ? 'Write …' : 'Continue writing …',
				showOnlyCurrent: false,
			}),
			Button.configure({ schema: o.schema, refs: () => o.refs }),
			Block.configure({ schema: o.schema, refs: () => o.refs }),
		],
		onUpdate: ({ editor }) => o.onChange(trimmed(editor.getJSON())),
		onTransaction: o.onTransaction,
		enableContentCheck: true,
		onContentError: ({ error }) => console.error('rich text:', error),
	});
}

const chain = (editor: Editor) => editor.chain().focus();

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

export function insertBlock(editor: Editor, type: string) {
	chain(editor)
		.insertContent({ type: 'embed', attrs: { id: null, type, fields: {} } })
		.run();
}

export function setFormat(editor: Editor, level: number) {
	if (level)
		chain(editor)
			.setHeading({ level: level as any })
			.run();
	else chain(editor).setParagraph().run();
}
