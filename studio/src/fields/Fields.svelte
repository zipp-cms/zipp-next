<script lang="ts">
	import type { Component, Field, Refs, Schema, Values } from '../api';
	import BlocksField from './BlocksField.svelte';
	import RelationField from './RelationField.svelte';
	import AssetField from './AssetField.svelte';
	import LinkField from './LinkField.svelte';
	import GroupField from './GroupField.svelte';
	import MarkdownField from './MarkdownField.svelte';
	import RichTextField from './RichTextField.svelte';
	import CustomField from './CustomField.svelte';
	import DropdownField from './DropdownField.svelte';
	import TextsField from './TextsField.svelte';
	import SeoField from './SeoField.svelte';
	import CheckboxesField from './CheckboxesField.svelte';
	import TableField from './TableField.svelte';
	import NavigationField from './NavigationField.svelte';
	import { isAdmin } from '../lib/access';

	let {
		schema,
		component,
		values,
		errors,
		refs,
		prefix = '',
		changed = new Set(),
	}: {
		schema: Schema;
		component: Component;
		values: Values;
		errors: Record<string, string>;
		refs: Refs;
		prefix?: string;
		/** fields to mark as changed */
		changed?: Set<string>;
	} = $props();

	/** Entry fields that are the same in all sites, so editors know. */
	function shared(field: Field) {
		return (
			component.entry && !field.translatable && schema.sites.length > 1
		);
	}
</script>

<!-- the old Zipp grid: labels on the left, fields on the right -->
<div class="fields" class:nested={prefix !== ''}>
	{#each component.fields as field (field.handle)}
		{@const path = prefix + field.handle}
		<label for={path}>
			{#if changed.has(path)}<span
					class="changed"
					title="Changed"
				></span>{/if}{field.label}{#if field.required}<span class="req">
					*
				</span>{/if}
			{#if shared(field)}<span class="shared">all sites</span>{/if}
			<!-- for admins the handle, like Craft's "show field handles" -->
			{#if isAdmin(schema)}
				<a
					class="handle"
					href="/admin/setup/{component.entry
						? 'types'
						: 'blocks'}/{component.handle}#{field.handle}"
					title="The field in Setup"
				>
					{field.handle}
				</a>
			{/if}
		</label>
		<div class="value">
			{#if field.kind === 'boolean'}
				<input
					id={path}
					type="checkbox"
					bind:checked={values[field.handle]}
				/>
			{:else if field.kind === 'text' && field.multiline}
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
			{:else if field.kind === 'group'}
				<GroupField
					{schema}
					{field}
					{values}
					{errors}
					{refs}
					{path}
					{changed}
				/>
			{:else if field.kind === 'markdown'}
				<MarkdownField {field} {values} id={path} />
			{:else if field.kind === 'richText'}
				<RichTextField {schema} {field} {values} {refs} />
			{:else if field.kind === 'link'}
				<LinkField {schema} {field} {values} {refs} />
			{:else if field.kind === 'checkboxes'}
				<CheckboxesField {field} {values} />
			{:else if field.kind === 'table'}
				<TableField {field} {values} />
			{:else if field.kind === 'navigation'}
				<NavigationField {schema} {field} {values} {refs} />
			{:else if field.kind === 'seo'}
				<SeoField {field} {values} />
			{:else if field.kind === 'texts'}
				<TextsField {schema} {field} {values} />
			{:else if field.kind === 'dropdown'}
				<DropdownField {field} {values} />
			{:else if field.kind === 'custom'}
				<CustomField {field} {values} />
			{:else if field.kind === 'assets'}
				<AssetField {field} {values} />
			{:else if field.kind === 'blocks'}
				<BlocksField
					{schema}
					{field}
					{values}
					{errors}
					{refs}
					{path}
					{changed}
				/>
			{/if}

			{#if errors[path]}<p class="error">{errors[path]}</p>{/if}
		</div>
	{/each}
</div>

<style lang="scss">
	.fields {
		display: grid;
		grid-template-columns: 200px minmax(0, 1fr);
		gap: 10px 30px;
		align-items: center;
	}

	.nested {
		grid-template-columns: 130px minmax(0, 1fr);
	}

	// at the top of tall fields like blocks or rich text, the first line
	// of a field is 37px high
	label {
		align-self: start;
		padding-top: 9px;
	}

	.shared {
		display: block;
		margin-top: 2px;
		font-size: 12px;
		color: var(--muted);
	}

	// like Craft: a dot in front of changed fields
	.changed {
		display: inline-block;
		width: 8px;
		height: 8px;
		margin-right: 6px;
		border-radius: 50%;
		background: #d98e04;
	}

	.req {
		color: var(--danger);
		margin-left: 2px;
	}

	@media (max-width: 800px) {
		.fields {
			grid-template-columns: 1fr;
			gap: 6px;
		}

		.value {
			margin-bottom: 10px;
		}
	}

	/* the handle for developers, links to the field in Setup */
	.handle {
		display: block;
		margin-top: 2px;
		font-family: monospace;
		font-size: 12px;
		color: var(--muted);
		text-decoration: none;

		&:hover {
			text-decoration: underline;
		}
	}
</style>
