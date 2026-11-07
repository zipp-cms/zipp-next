<script lang="ts">
	import type { Component, Schema, Values } from '../core/schema';
	import type { Refs } from '../entries/api';
	import { isShown } from './conditions';
	import FieldLabel from './FieldLabel.svelte';
	import { fieldComponent } from './registry';
	import { hasControl, labelId } from './labels';
	import FieldGroup from 'zipp-ui/FieldGroup.svelte';

	/**
	 * The fields of an entry type or block with their labels and errors:
	 * on the page the label above its field, inside a block or a card on
	 * its left, a field with a width then narrower in its own row.
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
		card = false,
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
		/** in a card like a group, a tab or type of `card` in the code */
		card?: boolean;
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

{#if card}
	<FieldGroup>{@render list()}</FieldGroup>
{:else}
	{@render list()}
{/if}

{#snippet list()}
	<div class="fields" class:nested={card || prefix !== ''}>
		{#each shown as field (field.handle)}
			{@const path = prefix + field.handle}
			{@const Input = fieldComponent(field)}
			<!-- the part of the row it takes (in a block: of the value column),
		every field full width in narrow places -->
			<div
				class="field"
				class:part={(field.width ?? 100) < 100}
				class:note={field.kind === 'note'}
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
				<div
					class="value"
					role={hasControl(field) ? undefined : 'group'}
					aria-labelledby={hasControl(field) || field.kind === 'note'
						? undefined
						: labelId(path)}
				>
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
{/snippet}

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

	@include container-min(36rem) {
		.field {
			grid-column: span var(--span);
		}
	}

	// inside a block: the label on the left, compact; every field in its
	// own row, so the label column lines up
	.nested {
		gap: 0.625rem 1.25rem;

		.field {
			grid-column: 1 / -1;
			display: grid;
			grid-template-columns: $label-col minmax(0, 1fr);
			gap: $label-gap;
			align-items: start;

			// every kind's label (a <label> or the <span> naming a group) at
			// the top of its row, level with the first line of the value,
			// which is 2.125rem high: the handle on the name's line, under it
			// only when the column is too narrow, the instructions below
			> :global(.label) {
				column-gap: 0.375rem;
				row-gap: 1px;
				padding-top: 0.4375rem;
			}

			> :global(.label .instructions) {
				margin-top: 0.25rem;
			}

			// a note has no label, it sits where the values are
			> .value:only-child {
				grid-column: 2;
			}
		}
	}

	// a width is the share of the value column it takes, enough for a
	// date at least and for choices in one line where they fit; an
	// error's or a note's text doesn't widen it
	@include container-min(26rem) {
		.nested .part > .value {
			width: max(var(--span) * 25%, min(100%, 14rem));
		}

		.nested .part:not(.note) > .value {
			min-width: fit-content;

			> .error {
				contain: inline-size;
			}
		}
	}

	// the label above its value again, like on the page: the handle on
	// the name's line, the instructions under it
	@include fields-narrow {
		.nested .field {
			display: flex;
			align-items: stretch;
			gap: 0.375rem;

			> :global(.label) {
				gap: 0.375rem;
				padding-top: 0;
			}

			> :global(.label .instructions) {
				margin-top: 0;
			}
		}
	}
</style>
