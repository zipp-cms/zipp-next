<script lang="ts">
	import type { Component, Schema, Values } from '../core/schema';
	import type { Refs } from '../entries/api';
	import FieldLabel from './FieldLabel.svelte';
	import { FIELDS } from './registry';

	/**
	 * The fields of an entry type or block with their labels and errors:
	 * on the page the label above its field, inside a block on its left.
	 */
	let {
		schema,
		component,
		values,
		errors,
		refs,
		prefix = '',
		changed = new Set(),
		only = null,
	}: {
		schema: Schema;
		component: Component;
		values: Values;
		errors: Record<string, string>;
		refs: Refs;
		prefix?: string;
		/** fields to mark as changed */
		changed?: Set<string>;
		/** the handles of the fields to show, those of a tab */
		only?: string[] | null;
	} = $props();

	const shown = $derived(
		only
			? component.fields.filter(f => only.includes(f.handle))
			: component.fields,
	);
</script>

<div class="fields" class:nested={prefix !== ''}>
	{#each shown as field (field.handle)}
		{@const path = prefix + field.handle}
		{@const Input = FIELDS[field.kind]}
		<FieldLabel
			{schema}
			{component}
			{field}
			{path}
			changed={changed.has(path)}
		/>
		<div class="value">
			<Input {schema} {field} {values} {errors} {refs} {path} {changed} />
			{#if errors[path]}<p class="error">{errors[path]}</p>{/if}
		</div>
	{/each}
</div>

<style lang="scss">
	.fields {
		display: flex;
		flex-direction: column;
		gap: 6px;

		.value {
			margin-bottom: 14px;
		}
	}

	// inside a block: the label on the left, compact
	.nested {
		display: grid;
		grid-template-columns: 130px minmax(0, 1fr);
		gap: 10px 20px;
		align-items: center;

		.value {
			margin-bottom: 0;
		}

		// at the top of tall fields like blocks or rich text, the first
		// line of a field is 34px high
		> :global(label) {
			align-self: start;
			flex-direction: column;
			align-items: flex-start;
			gap: 1px;
			padding-top: 7px;
		}
	}

	@media (max-width: 800px) {
		.nested {
			grid-template-columns: 1fr;
			gap: 6px;
		}
	}
</style>
