<script lang="ts">
	import { untrack } from 'svelte';
	import type { Refs, Schema, Values } from '../../api';
	import Fields from '../Fields.svelte';
	import EmbedCard from './EmbedCard.svelte';
	import { blockSummary } from '../../lib/blocks.svelte';

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
	}: {
		attrs: { id: string | null; type: string; fields: Values };
		schema: Schema;
		refs: Refs;
		editable: boolean;
		update: (attrs: Record<string, unknown>) => void;
		remove: () => void;
		insertLine: (where: 'before' | 'after') => void;
	} = $props();

	// attrs are reactive proxies, structuredClone can't copy them
	const clone = <T,>(v: T): T => JSON.parse(JSON.stringify(v));

	const comp = $derived(schema.components.find(c => c.handle === attrs.type));
	let fields = $state<Values>(untrack(() => clone(attrs.fields)));
	let last = JSON.stringify(untrack(() => attrs.fields));
	let editing = $state(
		untrack(() => !Object.keys(attrs.fields ?? {}).length),
	);

	// changes from the text (undo) come in, changes of the fields go out
	$effect(() => {
		const outer = JSON.stringify(attrs.fields);
		if (outer !== last) {
			last = outer;
			fields = clone(attrs.fields);
		}
	});
	$effect(() => {
		const inner = JSON.stringify($state.snapshot(fields));
		if (inner === last) return;
		last = inner;
		update({ ...attrs, fields: $state.snapshot(fields) });
	});
</script>

<EmbedCard
	kind={comp?.name ?? attrs.type}
	{editable}
	bind:editing
	onremove={remove}
	oninsert={insertLine}
>
	{#snippet preview()}
		<span class="summary">
			{blockSummary(schema, comp, fields) || 'Empty'}
		</span>
	{/snippet}
	{#if comp}
		<fieldset disabled={!editable}>
			<Fields
				{schema}
				component={comp}
				values={fields}
				errors={{}}
				{refs}
				prefix="rt."
			/>
		</fieldset>
	{/if}
</EmbedCard>

<style>
	.summary {
		overflow: hidden;
		color: var(--text);
		font-weight: 500;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	fieldset {
		margin: 0;
		padding: 0;
		border: none;
	}
</style>
