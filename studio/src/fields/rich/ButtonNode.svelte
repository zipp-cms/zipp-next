<script lang="ts">
	import type { Link, Schema } from '../../core/schema';
	import type { Refs } from '../../entries/api';
	import LinkInput from '../link/LinkInput.svelte';
	import NodeFrame from './NodeFrame.svelte';
	import DragHandle from 'zipp-ui/DragHandle.svelte';
	import type { Sortable } from 'zipp-ui/sortable';
	import { BUTTON_LINKS } from '../link/link';

	/**
	 * A button in a rich text: a link with a label, like a link field;
	 * buttons next to each other are one row on the website.
	 */
	let {
		attrs,
		schema,
		refs,
		editable,
		update,
		remove,
		insertLine,
		sortable,
		index,
	}: {
		attrs: { link: Link };
		schema: Schema;
		refs: Refs;
		editable: boolean;
		update: (attrs: Record<string, unknown>) => void;
		remove: () => void;
		insertLine: (where: 'before' | 'after') => void;
		sortable?: Sortable;
		index: () => number;
	} = $props();
</script>

<NodeFrame {editable} oninsert={insertLine}>
	<fieldset disabled={!editable}>
		<LinkInput
			{schema}
			{refs}
			link={attrs.link}
			to={BUTTON_LINKS}
			labels
			onchange={link => update({ link })}
			onremove={remove}
			removeTitle="Delete button"
		>
			{#snippet handle()}
				{#if editable && sortable}
					<DragHandle {sortable} {index} label="Move the button" />
				{/if}
			{/snippet}
		</LinkInput>
	</fieldset>
</NodeFrame>
