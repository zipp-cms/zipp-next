<script lang="ts">
	import type { Field, NavItem, Refs, Schema, Values } from '../api';
	import NavItems from './NavItems.svelte';

	/** A menu: links with their own label and the links under them. */
	let {
		schema,
		field,
		values,
		refs,
	}: { schema: Schema; field: Field; values: Values; refs: Refs } = $props();
</script>

{#if field.kind === 'navigation'}
	<NavItems
		{schema}
		{refs}
		items={(values[field.handle] ?? []) as NavItem[]}
		to={field.to}
		depth={field.depth}
		onchange={items => (values[field.handle] = items.length ? items : null)}
	/>
{/if}
