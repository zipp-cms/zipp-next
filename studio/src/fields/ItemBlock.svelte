<script lang="ts">
	import type { Block, FieldOf, Schema, Values } from '../core/schema';
	import { componentOf } from '../core/schema';
	import type { Refs } from '../entries/api';
	import BlockFields from './BlockFields.svelte';

	/**
	 * The fields of the block `id` in the items of a custom type, what
	 * `blocks.mount` of its module shows; nothing once it left them.
	 */
	let {
		schema,
		field,
		values,
		errors,
		refs,
		path,
		changed,
		id,
	}: {
		schema: Schema;
		field: FieldOf<'custom'>;
		values: Values;
		errors: Record<string, string>;
		refs: Refs;
		path: string;
		changed: Set<string>;
		id: string;
	} = $props();

	const items = $derived(
		(values[field.handle] as { block?: Block }[] | null) ?? [],
	);
	const at = $derived(items.findIndex(i => i?.block?.id === id));
	const block = $derived(items[at]?.block);
	const component = $derived(block && componentOf(schema, block.type));
</script>

{#if block && component}
	<BlockFields
		{schema}
		{component}
		values={block.fields}
		{errors}
		{refs}
		prefix="{path}.{at}.block."
		{changed}
		{id}
	/>
{/if}
