<script lang="ts">
	import type { Field, Values } from '../api';
	import AssetField from '../fields/AssetField.svelte';
	import CheckboxesField from '../fields/CheckboxesField.svelte';
	import DropdownField from '../fields/DropdownField.svelte';
	import MarkdownField from '../fields/MarkdownField.svelte';
	import TableField from '../fields/TableField.svelte';

	/**
	 * A field of the studio inside a plugin's module, see `host.ts`: the
	 * value comes in and goes out through `onchange`.
	 */
	let {
		field,
		value,
		onchange,
	}: {
		field: Field;
		value: unknown;
		onchange: (v: unknown) => void;
	} = $props();

	let values = $state<Values>({});
	// the value from outside, unless it is the one sent
	let sent: unknown = undefined;
	$effect.pre(() => {
		if (value !== sent) values[field.handle] = value ?? undefined;
	});
	$effect(() => {
		const v = $state.snapshot(values[field.handle]);
		if (JSON.stringify(v) === JSON.stringify(value)) return;
		sent = v;
		onchange(v ?? null);
	});
</script>

{#if field.kind === 'assets'}
	<AssetField {field} {values} />
{:else if field.kind === 'dropdown'}
	<DropdownField {field} {values} />
{:else if field.kind === 'checkboxes'}
	<CheckboxesField {field} {values} />
{:else if field.kind === 'table'}
	<TableField {field} {values} />
{:else if field.kind === 'markdown'}
	<MarkdownField {field} {values} id={field.handle} />
{:else if field.kind === 'text' && field.multiline}
	<textarea bind:value={values[field.handle]}></textarea>
{:else if field.kind === 'text'}
	<input bind:value={values[field.handle]} />
{:else}
	<span class="error">{field.kind} fields can't be mounted by plugins</span>
{/if}
