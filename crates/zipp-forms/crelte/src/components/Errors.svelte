<script lang="ts">
	import type { FieldData } from '../types';
	import { getForm } from '../context';

	/** What is wrong with a field's value after sending. */
	let { field }: { field: FieldData } = $props();

	const form = getForm();
	const error = $derived(form.errors[field.handle]);
</script>

{#if error}
	<p class="error" id="{form.inputId(field)}-errors">{error}</p>
{/if}
