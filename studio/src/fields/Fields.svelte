<script lang="ts">
	import type { Component, Schema, Values } from '../core/schema';
	import type { Refs } from '../entries/api';
	import { isShown } from './conditions';
	import FieldLabel from './FieldLabel.svelte';
	import { fieldComponent } from './registry';

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

	// kinds whose editor has no place for the errors inside its value
	// (a cell of a table, a link of a navigation, a text), they show
	// under the field: `#2 url: …`
	const ROWS = ['table', 'navigation', 'texts'];

	function inside(field: Component['fields'][number], path: string) {
		if (!ROWS.includes(field.kind)) return [];
		return Object.entries(errors)
			.filter(([p]) => p.startsWith(path + '.'))
			.map(([p, message]) => {
				const at = p
					.slice(path.length + 1)
					.split('.')
					.map(s => (/^\d+$/.test(s) ? `#${+s + 1}` : s))
					.join(' ');
				return `${at}: ${message}`;
			});
	}

	// those of the tab whose conditions hold, live while editing
	const shown = $derived(
		component.fields.filter(
			f =>
				(!only || only.includes(f.handle)) &&
				isShown(component, f.handle, values),
		),
	);
</script>

<div class="fields" class:nested={prefix !== ''}>
	{#each shown as field (field.handle)}
		{@const path = prefix + field.handle}
		{@const Input = fieldComponent(field)}
		<!-- the part of the row it takes, every field full width in narrow
		places -->
		<div
			class="field"
			class:part={(field.width ?? 100) < 100}
			style:--span={(field.width ?? 100) / 25}
		>
			{#if field.kind !== 'note'}
				<FieldLabel
					{schema}
					{component}
					{field}
					{path}
					changed={changed.has(path)}
				/>
			{/if}
			<div class="value">
				<Input
					{schema}
					{field}
					{values}
					{errors}
					{refs}
					{path}
					{changed}
				/>
				{#if errors[path]}<p class="error">{errors[path]}</p>{/if}
				{#each inside(field, path) as e (e)}<p class="error">
						{e}
					</p>{/each}
			</div>
		</div>
	{/each}
</div>

<style lang="scss">
	// a row has four quarters, a field takes as many as its width says
	// when there is room for them
	.fields {
		container-type: inline-size;
		display: grid;
		grid-template-columns: repeat(4, minmax(0, 1fr));
		gap: 0.875rem 1.25rem;
	}

	.field {
		grid-column: 1 / -1;
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
		min-width: 0;
	}

	@container (min-width: 36rem) {
		.field {
			grid-column: span var(--span);
		}
	}

	// inside a block: the label on the left, compact
	.nested {
		gap: 0.625rem 1.25rem;

		.field {
			display: grid;
			grid-template-columns: 8.125rem minmax(0, 1fr);
			gap: 1.25rem;
			align-items: center;

			// at the top of tall fields like blocks or rich text, the first
			// line of a field is 2.125rem high
			> :global(label) {
				align-self: start;
				flex-direction: column;
				align-items: flex-start;
				gap: 1px;
				padding-top: 0.4375rem;
			}

			// a note has no label, it sits where the values are
			> .value:only-child {
				grid-column: 2;
			}
		}
	}

	// fields that share a row keep their label above
	@container (min-width: 36rem) {
		.nested .part {
			display: flex;
			align-items: stretch;
			gap: 0.375rem;

			> .value:only-child {
				grid-column: auto;
			}
		}
	}

	@include max-tablet {
		.nested .field {
			display: flex;
			align-items: stretch;
			gap: 0.375rem;

			> .value:only-child {
				grid-column: auto;
			}
		}
	}
</style>
