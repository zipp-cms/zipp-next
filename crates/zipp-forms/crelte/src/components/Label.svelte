<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { FieldData } from '../types';
	import { getForm } from '../context';

	/** The label of a field, `*` (or `required`) after a required one. */
	let { field, required }: { field: FieldData; required?: Snippet } =
		$props();

	const form = getForm();
</script>

<label for={form.inputId(field)}>
	{field.label}{#if field.required}
		{#if required}{@render required()}{:else}<span class="required">
				*
			</span>{/if}
	{/if}
</label>
