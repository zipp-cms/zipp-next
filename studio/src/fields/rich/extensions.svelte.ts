import { Mark, Node, type NodeViewRendererProps } from '@tiptap/core';
import { mount, unmount, type Component } from 'svelte';
import type { Refs, Schema } from '../../api';
import BlockNode from './BlockNode.svelte';
import ButtonsNode from './ButtonsNode.svelte';

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

type Options = { schema: Schema; refs: Refs };

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
			refs: options.refs,
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

/** A row of buttons, `attrs.links` are links with labels. */
export const Buttons = Node.create<Options>({
	name: 'buttons',
	group: 'block',
	atom: true,
	draggable: true,
	addOptions() {
		return { schema: null as any, refs: {} };
	},
	addAttributes() {
		return { links: { default: [] } };
	},
	parseHTML() {
		return [{ tag: 'div[data-buttons]' }];
	},
	renderHTML() {
		return ['div', { 'data-buttons': '' }];
	},
	addNodeView() {
		return svelteView(ButtonsNode, this.options);
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
		return { schema: null as any, refs: {} };
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
