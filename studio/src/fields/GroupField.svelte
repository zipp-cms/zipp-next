<script lang="ts">
	import type { Field, Schema, Values } from '../core/schema';
	import type { Refs } from '../entries/api';
	import Fields from './Fields.svelte';
	import { componentOf } from '../core/schema';

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
		field.kind === 'group'
			? componentOf(schema, field.component)
			: undefined,
	);

	// the group is an object, also when it was never filled in
	$effect.pre(() => {
		if (!values[field.handle]) values[field.handle] = {};
	});
</script>

{#if comp && values[field.handle]}
	<div class="group box">
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
	/* a white box like an open block, its fields on the left */
	.group {
		padding: 14px;
	}
</style>
