import {
	Extension,
	Mark,
	Node,
	type NodeViewRendererProps,
} from '@tiptap/core';
import { Plugin } from '@tiptap/pm/state';
import { mount, unmount, type Component } from 'svelte';
import type { Schema, Style } from '../../core/schema';
import type { Refs } from '../../entries/api';
import type { Sortable } from 'zipp-ui/sortable';
import BlockNode from './BlockNode.svelte';
import ButtonNode from './ButtonNode.svelte';

/**
 * An attribute kept as JSON in a `data-` attribute of the html: copying
 * and pasting go through html, also inside one editor, and keep it.
 */
function jsonAttr(name: string, fallback: unknown) {
	return {
		default: fallback,
		parseHTML: (el: HTMLElement) => {
			const v = el.getAttribute(`data-${name}`);
			try {
				return v === null ? fallback : JSON.parse(v);
			} catch {
				return fallback;
			}
		},
		renderHTML: (attrs: Record<string, unknown>) => ({
			[`data-${name}`]: JSON.stringify(attrs[name]),
		}),
	};
}

/**
 * A link like the link field (entry by id, url, email, phone), stored in
 * `attrs.link`; zipp resolves it to a url when queried.
 */
export const ZippLink = Mark.create({
	name: 'link',
	inclusive: false,
	addAttributes() {
		return { link: jsonAttr('link', null) };
	},
	parseHTML() {
		return [{ tag: 'a[data-link]' }];
	},
	renderHTML({ HTMLAttributes }) {
		return ['a', { ...HTMLAttributes, class: 'rt-link' }, 0];
	},
});

/**
 * The style of an element if it is one of the field's: copied in the
 * editor it is `data-style`, from a website a class. The editor doesn't
 * give the site's classes, they could be ones of the studio.
 */
function styleOf(el: HTMLElement, styles: Style[]) {
	const st = styles.find(
		st => el.dataset.style === st.class || el.classList.contains(st.class),
	);
	return st?.class ?? null;
}

/**
 * The block styles of a field: a class on a paragraph or heading,
 * `attrs.class`. Pasted html keeps only the field's own.
 */
export const BlockStyles = Extension.create<{ styles: Style[] }>({
	name: 'blockStyles',
	addOptions() {
		return { styles: [] };
	},
	addGlobalAttributes() {
		const styles = this.options.styles.filter(st => !st.inline);
		return [
			{
				types: ['paragraph', 'heading'],
				attributes: {
					class: {
						default: null,
						parseHTML: el => styleOf(el, styles),
						renderHTML: attrs =>
							attrs.class
								? {
										class: 'rt-style',
										'data-style': attrs.class,
									}
								: {},
					},
				},
			},
		];
	},
});

/** An inline style of a field: a mark with a class on a part of the text. */
export const InlineStyle = Mark.create<{ styles: Style[] }>({
	name: 'style',
	addOptions() {
		return { styles: [] };
	},
	addAttributes() {
		return { class: { default: null } };
	},
	parseHTML() {
		const styles = this.options.styles.filter(st => st.inline);
		return [
			{
				tag: 'span',
				getAttrs: el => {
					const c = styleOf(el, styles);
					return c ? { class: c } : false;
				},
			},
		];
	},
	renderHTML({ HTMLAttributes }) {
		const style = HTMLAttributes.class;
		return ['span', { class: 'rt-mark', 'data-style': style }, 0];
	},
});

/**
 * `refs` as a function: TipTap copies plain objects of the options, the
 * node views need the entry's own, where picked entries go.
 */
type Options = {
	schema: Schema;
	refs: () => Refs;
	/** of the field, buttons link only to entries of them */
	sections: string[];
	/** moves the nodes of the text, see RichTextField */
	sortable: () => Sortable;
};

/** What `svelteView` gives the component of a node with attrs `A`. */
export type NodeProps<A> = {
	attrs: A;
	schema: Schema;
	refs: Refs;
	sections: string[];
	editable: boolean;
	update: (attrs: Record<string, unknown>) => void;
	remove: () => void;
	insertLine: (where: 'before' | 'after') => void;
	sortable: Sortable;
	/** its place among the nodes of the text */
	index: () => number;
};

/**
 * A node edited by a Svelte component outside of the text: it gets the
 * attrs, `update` to change them, `remove`, and `insertLine` for an empty
 * line before or after it with the cursor in it.
 */
function svelteView(component: Component<NodeProps<any>>, options: Options) {
	return ({ node, getPos, editor }: NodeViewRendererProps) => {
		const dom = document.createElement('div');
		dom.contentEditable = 'false';
		dom.className = 'rt-node';

		const props = $state({
			attrs: node.attrs,
			schema: options.schema,
			refs: options.refs(),
			sections: options.sections,
			sortable: options.sortable(),
			// its place among the nodes of the text, it changes with them
			index: () => editor.state.doc.resolve(getPos() ?? 0).index(),
			editable: editor.isEditable,
			update: (attrs: Record<string, unknown>) => {
				const pos = getPos();
				if (pos === undefined) return;
				editor.view.dispatch(
					editor.view.state.tr.setNodeMarkup(pos, undefined, attrs),
				);
			},
			remove: () => {
				const pos = getPos();
				if (pos === undefined) return;
				editor.view.dispatch(
					editor.view.state.tr.delete(pos, pos + node.nodeSize),
				);
			},
			insertLine: (where: 'before' | 'after') => {
				const pos = getPos();
				if (pos === undefined) return;
				const at = where === 'before' ? pos : pos + node.nodeSize;
				editor
					.chain()
					.insertContentAt(at, { type: 'paragraph' })
					.setTextSelection(at + 1)
					.focus()
					.run();
			},
		});
		const app = mount(component, { target: dom, props });
		// read only can change later (a conflict, publishing): the field
		// sends a transaction after setEditable
		const follow = () => (props.editable = editor.isEditable);
		editor.on('transaction', follow);

		return {
			dom,
			update(n: typeof node) {
				if (n.type !== node.type) return false;
				node = n;
				props.attrs = n.attrs;
				return true;
			},
			destroy: () => {
				editor.off('transaction', follow);
				unmount(app);
			},
			// the inputs inside belong to the component, not to the text
			stopEvent: () => true,
			ignoreMutation: () => true,
		};
	};
}

/**
 * A button, `attrs.link` a link with a label; buttons next to each other
 * are one row on the website.
 */
export const Button = Node.create<Options>({
	name: 'button',
	group: 'block',
	atom: true,
	// moved by its DragHandle, not by the browser's drag and drop
	draggable: false,
	addOptions() {
		return {
			schema: null as any,
			refs: () => ({}),
			sections: [] as string[],
			sortable: null as any,
		};
	},
	addAttributes() {
		return { link: jsonAttr('link', { type: 'entry', value: null }) };
	},
	parseHTML() {
		return [{ tag: 'div[data-button]' }];
	},
	renderHTML({ HTMLAttributes }) {
		return ['div', { ...HTMLAttributes, 'data-button': '' }];
	},
	addNodeView() {
		return svelteView(ButtonNode, this.options);
	},
});

/**
 * A block of the schema in the text, `attrs` like a block: id, type,
 * fields. Not called `block`, ProseMirror would read the `block+` of the
 * document as this node instead of the group.
 */
export const Block = Node.create<Options>({
	name: 'embed',
	group: 'block',
	atom: true,
	// moved by its DragHandle, not by the browser's drag and drop
	draggable: false,
	addOptions() {
		return {
			schema: null as any,
			refs: () => ({}),
			sections: [] as string[],
			sortable: null as any,
		};
	},
	addAttributes() {
		return {
			// a pasted block is a new one, saving gives it an id
			id: {
				default: null,
				parseHTML: () => null,
				renderHTML: () => ({}),
			},
			type: jsonAttr('type', null),
			fields: jsonAttr('fields', {}),
		};
	},
	parseHTML() {
		return [{ tag: 'div[data-embed]' }];
	},
	renderHTML({ HTMLAttributes }) {
		return ['div', { ...HTMLAttributes, 'data-embed': '' }];
	},
	addNodeView() {
		return svelteView(BlockNode, this.options);
	},
});

/**
 * An empty line at the end, always: "Continue writing …" shows there,
 * after a block or button too. Saving leaves it out (see `trimmed`).
 */
export const EndLine = Extension.create({
	name: 'endLine',
	addProseMirrorPlugins() {
		return [
			new Plugin({
				appendTransaction: (_, __, state) => {
					const last = state.doc.lastChild;
					if (last?.type.name === 'paragraph' && !last.content.size)
						return null;
					const line = state.schema.nodes.paragraph.create();
					return state.tr
						.insert(state.doc.content.size, line)
						.setMeta('addToHistory', false);
				},
			}),
		];
	},
});
