<script lang="ts">
	import type { FieldOf, Schema, Values } from '../core/schema';
	import { valueOf } from '../core/schema';
	import type { Refs } from '../entries/api';
	import NavItems from './link/NavItems.svelte';

	/** A menu: links with their own label and the links under them. */
	let {
		schema,
		field,
		values,
		refs,
	}: {
		schema: Schema;
		field: FieldOf<'navigation'>;
		values: Values;
		refs: Refs;
	} = $props();
</script>

<NavItems
	{schema}
	{refs}
	items={valueOf(values, field) ?? []}
	to={field.to}
	depth={field.depth}
	onchange={items => (values[field.handle] = items.length ? items : null)}
/>
