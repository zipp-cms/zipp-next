<script lang="ts">
	import { untrack } from 'svelte';
	import type { Schema, Values } from '../../core/schema';
	import type { Refs } from '../../entries/api';
	import Fields from '../Fields.svelte';
	import EmbedCard from './EmbedCard.svelte';
	import DragHandle from 'zipp-ui/DragHandle.svelte';
	import type { Sortable } from 'zipp-ui/sortable';
	import { blockSummary } from '../blocks.svelte';
	import { synced } from '../../core/synced.svelte';
	import { componentOf } from '../../core/schema';

	/**
	 * A block inside a rich text: a card with its summary, its fields like
	 * in a blocks field when it opens. A new one opens.
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
		attrs: { id: string | null; type: string; fields: Values };
		schema: Schema;
		refs: Refs;
		editable: boolean;
		update: (attrs: Record<string, unknown>) => void;
		remove: () => void;
		insertLine: (where: 'before' | 'after') => void;
		sortable?: Sortable;
		index: () => number;
	} = $props();

	const comp = $derived(componentOf(schema, attrs.type));
	// the ids of its inputs, unique with several blocks in the text
	const uid = $props.id();
	// changes from the text (undo) come in, changes of the fields go out
	const fields = synced(
		() => attrs.fields,
		f => update({ ...attrs, fields: f }),
	);
	let editing = $state(
		untrack(() => !Object.keys(attrs.fields ?? {}).length),
	);
</script>

<EmbedCard
	kind={comp?.name ?? attrs.type}
	{editable}
	bind:editing
	onremove={remove}
	oninsert={insertLine}
>
	{#snippet handle()}
		{#if editable && sortable}
			<DragHandle {sortable} {index} label="Move the block" />
		{/if}
	{/snippet}
	{#snippet preview()}
		{blockSummary(comp, fields.value) || 'Empty'}
	{/snippet}
	{#if comp}
		<fieldset disabled={!editable}>
			<Fields
				{schema}
				component={comp}
				values={fields.value}
				errors={{}}
				{refs}
				prefix="{uid}."
			/>
		</fieldset>
	{/if}
</EmbedCard>
