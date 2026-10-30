<script lang="ts">
	import type { Field, Link, Schema, Values } from '../core/schema';
	import type { Refs } from '../entries/api';
	import LinkInput from './LinkInput.svelte';

	/** A link field of an entry or block, see LinkInput. */
	let {
		schema,
		field,
		values,
		refs,
	}: {
		schema: Schema;
		field: Extract<Field, { kind: 'link' }>;
		values: Values;
		refs: Refs;
	} = $props();

	const link = $derived<Link>(
		values[field.handle] ?? { type: field.to[0], value: null },
	);
</script>

<LinkInput
	{schema}
	{refs}
	{link}
	to={field.to}
	labels={field.labels}
	newTab={field.newTab}
	onchange={l => (values[field.handle] = l)}
/>
