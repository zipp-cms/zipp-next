<script lang="ts">
	import type { FieldData } from '../types';
	import { getForm } from '../context';

	/** The input of a field by its type, bound to the form's value. */
	let { field }: { field: FieldData } = $props();

	const form = getForm();
	const id = $derived(form.inputId(field));
	const error = $derived(form.errors[field.handle]);
	const described = $derived(
		[field.instructions && `${id}-instructions`, error && `${id}-errors`]
			.filter(Boolean)
			.join(' ') || undefined,
	);
	const types = { text: 'text', email: 'email', phone: 'tel' } as const;
	const autocomplete = { email: 'email', phone: 'tel' } as const;
</script>

{#if field.type === 'textarea'}
	<textarea
		{id}
		name={field.handle}
		placeholder={field.placeholder || undefined}
		required={field.required}
		aria-invalid={!!error}
		aria-describedby={described}
		bind:value={form.values[field.handle]}
	></textarea>
{:else}
	<input
		{id}
		type={types[field.type]}
		name={field.handle}
		placeholder={field.placeholder || undefined}
		required={field.required}
		autocomplete={field.type === 'text'
			? undefined
			: autocomplete[field.type]}
		aria-invalid={!!error}
		aria-describedby={described}
		bind:value={form.values[field.handle]}
	/>
{/if}
