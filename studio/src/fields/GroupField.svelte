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

	// a group never filled in gets its object with the first value, not
	// when it shows: opening an entry changes nothing
	const empty = $state<Values>({});
	const group = $derived<Values>(values[field.handle] ?? empty);
	$effect(() => {
		if (!values[field.handle] && Object.keys(empty).length)
			values[field.handle] = empty;
	});
</script>

{#if comp}
	<div class="group box">
		<Fields
			{schema}
			component={comp}
			values={group}
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
