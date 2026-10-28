<script lang="ts">
	import type { Component, Refs, Schema, Values } from '../api';
	import AutoText from './AutoText.svelte';
	import Document from './Document.svelte';
	import RelationField from './RelationField.svelte';

	/**
	 * Fields as properties: a label and the value next to it. A blocks
	 * field becomes a document below them.
	 */
	let {
		schema,
		component,
		values,
		errors,
		refs,
		prefix = '',
	}: {
		schema: Schema;
		component: Component;
		values: Values;
		errors: Record<string, string>;
		refs: Refs;
		prefix?: string;
	} = $props();
</script>

<div class="fields">
	{#each component.fields as field (field.handle)}
		{@const path = prefix + field.handle}
		{#if field.kind === 'blocks'}
			<div class="document">
				{#if component.fields.length > 1}
					<div class="section">{field.label}</div>
				{/if}
				<Document {schema} {field} {values} {errors} {refs} {path} />
			</div>
		{:else}
			<div class="prop">
				<label for={path}>
					{field.label}{#if field.required}<span class="req">
							*
						</span>{/if}
				</label>
				<div class="value">
					{#if field.kind === 'text'}
						<AutoText
							id={path}
							placeholder="Empty"
							multiline={field.multiline}
							heading={field.heading}
							maxlength={field.max}
							bind:value={values[field.handle]}
						/>
					{:else if field.kind === 'number'}
						<input
							id={path}
							type="number"
							placeholder="Empty"
							min={field.min ?? undefined}
							max={field.max ?? undefined}
							bind:value={values[field.handle]}
						/>
					{:else if field.kind === 'boolean'}
						<input
							id={path}
							type="checkbox"
							bind:checked={values[field.handle]}
						/>
					{:else if field.kind === 'relation'}
						<RelationField {schema} {field} {values} {refs} />
					{/if}
				</div>
			</div>
		{/if}
		{#if errors[path]}<p class="error">{errors[path]}</p>{/if}
	{/each}
</div>

<style lang="scss">
	.fields {
		display: flex;
		flex-direction: column;
		gap: 0.1rem;
	}

	.prop {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
		border-radius: 0.3rem;

		label {
			width: 9rem;
			flex-shrink: 0;
			padding: 0.3rem 0;
			color: var(--muted);
			font-size: 0.9rem;
		}

		.value {
			flex: 1;
			min-width: 0;
			padding: 0.05rem 0.4rem;
			border-radius: 0.3rem;

			&:focus-within {
				background: var(--side);
			}
		}

		input[type='number'] {
			border: none;
			outline: none;
			padding: 0.15rem 0;
			background: none;
		}

		input[type='checkbox'] {
			width: auto;
			margin: 0.45rem 0;
		}
	}

	.document {
		margin-top: 1.2rem;
	}

	.section {
		color: var(--muted);
		font-size: 0.8rem;
		text-transform: uppercase;
		letter-spacing: 0.04em;
		margin-bottom: 0.3rem;
	}

	.req {
		color: var(--danger);
		margin-left: 0.15rem;
	}

	@media (max-width: 700px) {
		.prop {
			flex-direction: column;
			gap: 0;

			label {
				width: auto;
				padding-bottom: 0;
			}

			.value {
				width: 100%;
				box-sizing: border-box;
				padding: 0;
			}
		}
	}
</style>
