import { Mark, Node, type NodeViewRendererProps } from '@tiptap/core';
import { mount, unmount, type Component } from 'svelte';
import type { Schema } from '../../core/schema';
import type { Refs } from '../../entries/api';
import BlockNode from './BlockNode.svelte';
import ButtonNode from './ButtonNode.svelte';

/**
 * A link like the link field (entry by id, url, email, phone), stored in
 * `attrs.link`; zipp resolves it to a url when queried.
 */
export const ZippLink = Mark.create({
	name: 'link',
	inclusive: false,
	addAttributes() {
		return { link: { default: null } };
	},
	parseHTML() {
		return [{ tag: 'a[data-link]' }];
	},
	renderHTML({ mark }) {
		return [
			'a',
			{ 'data-link': JSON.stringify(mark.attrs.link), class: 'rt-link' },
			0,
		];
	},
});

/**
 * `refs` as a function: TipTap copies plain objects of the options, the
 * node views need the entry's own, where picked entries go.
 */
type Options = { schema: Schema; refs: () => Refs };

/**
 * A node edited by a Svelte component outside of the text: it gets the
 * attrs, `update` to change them, `remove`, and `insertLine` for an empty
 * line before or after it with the cursor in it.
 */
function svelteView(component: Component<any>, options: Options) {
	return ({ node, getPos, editor }: NodeViewRendererProps) => {
		const dom = document.createElement('div');
		dom.contentEditable = 'false';
		dom.className = 'rt-node';

		const props = $state({
			attrs: node.attrs,
			schema: options.schema,
			refs: options.refs(),
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

		return {
			dom,
			update(n: typeof node) {
				if (n.type !== node.type) return false;
				node = n;
				props.attrs = n.attrs;
				return true;
			},
			destroy: () => unmount(app),
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
	draggable: true,
	addOptions() {
		return { schema: null as any, refs: () => ({}) };
	},
	addAttributes() {
		return { link: { default: { type: 'entry', value: null } } };
	},
	parseHTML() {
		return [{ tag: 'div[data-button]' }];
	},
	renderHTML() {
		return ['div', { 'data-button': '' }];
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
	draggable: true,
	addOptions() {
		return { schema: null as any, refs: () => ({}) };
	},
	addAttributes() {
		return {
			id: { default: null },
			type: { default: null },
			fields: { default: {} },
		};
	},
	parseHTML() {
		return [{ tag: 'div[data-embed]' }];
	},
	renderHTML() {
		return ['div', { 'data-embed': '' }];
	},
	addNodeView() {
		return svelteView(BlockNode, this.options);
	},
});
