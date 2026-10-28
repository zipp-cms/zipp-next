<script lang="ts">
	import type { Component, Field, Refs, Schema, Values } from '../api';
	import BlocksField from './BlocksField.svelte';
	import RelationField from './RelationField.svelte';
	import AssetField from './AssetField.svelte';

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

	/** Entry fields that are the same in all sites, so editors know. */
	function shared(field: Field) {
		return (
			component.entry && !field.translatable && schema.sites.length > 1
		);
	}
</script>

<div class="fields">
	{#each component.fields as field (field.handle)}
		{@const path = prefix + field.handle}
		<div class="field">
			{#if field.kind === 'boolean'}
				<label class="check">
					<input
						type="checkbox"
						bind:checked={values[field.handle]}
					/>
					{field.label}
				</label>
			{:else}
				<label for={path}>
					{field.label}{#if field.required}<span class="req">
							*
						</span>{/if}
					{#if shared(field)}<span class="shared">
							all sites
						</span>{/if}
				</label>
			{/if}

			{#if field.kind === 'text' && field.multiline}
				<textarea
					id={path}
					maxlength={field.max ?? undefined}
					bind:value={values[field.handle]}
				></textarea>
			{:else if field.kind === 'text'}
				<input
					id={path}
					maxlength={field.max ?? undefined}
					bind:value={values[field.handle]}
				/>
			{:else if field.kind === 'number'}
				<input
					id={path}
					type="number"
					min={field.min ?? undefined}
					max={field.max ?? undefined}
					bind:value={values[field.handle]}
				/>
			{:else if field.kind === 'relation'}
				<RelationField {schema} {field} {values} {refs} />
			{:else if field.kind === 'assets'}
				<AssetField {field} {values} />
			{:else if field.kind === 'blocks'}
				<BlocksField {schema} {field} {values} {errors} {refs} {path} />
			{/if}

			{#if errors[path]}<p class="error">{errors[path]}</p>{/if}
		</div>
	{/each}
</div>

<style lang="scss">
	.fields {
		display: flex;
		flex-direction: column;
		gap: 1.1rem;
	}

	label {
		display: block;
		font-weight: 500;
		margin-bottom: 0.35rem;
	}

	.check {
		display: flex;
		gap: 0.4rem;
		align-items: center;

		input {
			width: auto;
		}
	}

	.shared {
		margin-left: 0.4rem;
		font-size: 0.75rem;
		font-weight: normal;
		color: var(--muted);
	}

	.req {
		color: var(--danger);
		margin-left: 0.15rem;
	}
</style>
