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
	import CheckboxesField from './CheckboxesField.svelte';
	import TableField from './TableField.svelte';
	import NavigationField from './NavigationField.svelte';
	import { isAdmin } from '../lib/access';
	import Icon from '../lib/Icon.svelte';

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

	/**
	 * Entry fields with their own value in each site get an icon, like
	 * Craft's; the others are the same in all sites.
	 */
	function translated(field: Field) {
		return component.entry && field.translatable && schema.sites.length > 1;
	}
</script>

<!-- on the page the label above its field, inside a block on its left -->
<div class="fields" class:nested={prefix !== ''}>
	{#each shown as field (field.handle)}
		{@const path = prefix + field.handle}
		<label for={path}>
			<span class="name">
				{#if changed.has(path)}<span
						class="changed"
						title="Changed"
					></span>{/if}{field.label}{#if field.required}<span
						class="req"
					>
						*
					</span>{/if}
			</span>
			{#if translated(field)}
				<span
					class="translated"
					title="Translated: each site has its own"
				>
					<Icon name="translate" size={13} />
				</span>
			{/if}
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
					checked={values[field.handle] ?? false}
					onchange={e =>
						(values[field.handle] = e.currentTarget.checked)}
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
		label {
			align-self: start;
			flex-direction: column;
			align-items: flex-start;
			gap: 1px;
			padding-top: 7px;
		}
	}

	label {
		display: flex;
		align-items: baseline;
		gap: 6px;
		flex-wrap: wrap;
		font-size: var(--fs-md);
		font-weight: 600;
		color: var(--text);
	}

	.translated {
		display: inline-flex;
		align-self: center;
		color: var(--subtle);
	}

	// like Craft: a dot in front of changed fields
	.changed {
		display: inline-block;
		width: 7px;
		height: 7px;
		margin-right: 4px;
		border-radius: 50%;
		background: var(--changed);
	}

	.req {
		margin-left: 2px;
		color: var(--danger);
	}

	@media (max-width: 800px) {
		.nested {
			grid-template-columns: 1fr;
			gap: 6px;
		}
	}

	/* the handle for developers, links to the field in Setup */
	.handle {
		font-family: var(--mono);
		font-size: var(--fs-xs);
		font-weight: 400;
		color: var(--subtle);
		text-decoration: none;

		&:hover {
			color: var(--accent);
		}
	}
</style>
