<script lang="ts">
	import type { Field, Refs, Schema, Values } from '../api';
	import Fields from './Fields.svelte';

	/** The fields of another component under one name, always there. */
	let {
		schema,
		field,
		values,
		errors,
		refs,
		path,
		changed,
	}: {
		schema: Schema;
		field: Field;
		values: Values;
		errors: Record<string, string>;
		refs: Refs;
		path: string;
		changed: Set<string>;
	} = $props();

	const comp = $derived(
		schema.components.find(
			c => field.kind === 'group' && c.handle === field.component,
		),
	);

	// the group is an object, also when it was never filled in
	$effect.pre(() => {
		if (!values[field.handle]) values[field.handle] = {};
	});
</script>

{#if comp && values[field.handle]}
	<div class="group">
		<Fields
			{schema}
			component={comp}
			values={values[field.handle]}
			{errors}
			{refs}
			prefix="{path}."
			{changed}
		/>
	</div>
{/if}

<style>
	.group {
		padding: 12px 14px;
		background: #f4f4f5;
	}
</style>
