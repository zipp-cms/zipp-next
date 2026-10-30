<script lang="ts">
	import { untrack } from 'svelte';
	import type { FieldOf, Schema, Values } from '../core/schema';
	import { valueOf } from '../core/schema';
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
		field: FieldOf<'group'>;
		values: Values;
		errors: Record<string, string>;
		refs: Refs;
		path: string;
		changed: Set<string>;
	} = $props();

	const comp = $derived(componentOf(schema, field.component));

	// a group never filled in gets its object with the first value, not
	// when it shows: opening an entry changes nothing
	let empty = $state<Values>({});
	// other values (a restored revision, another site's): the fallback
	// starts empty again, it would bring back what was typed before
	let owner = untrack(() => values);
	$effect.pre(() => {
		if (values === owner) return;
		owner = values;
		empty = {};
	});
	const group = $derived(valueOf(values, field) ?? empty);
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

<style lang="scss">
	/* a white box like an open block, its fields on the left */
	.group {
		padding: 0.875rem;
	}
</style>
