<script lang="ts">
	import type { FieldData } from '../types';
	import { getForm } from '../context';

	/** The instructions of a field, nothing without. */
	let { field }: { field: FieldData } = $props();

	const form = getForm();
</script>

{#if field.instructions}
	<p class="instructions" id="{form.inputId(field)}-instructions">
		{field.instructions}
	</p>
{/if}
