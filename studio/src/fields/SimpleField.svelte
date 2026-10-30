<script lang="ts">
	import type { Field, Values } from '../core/schema';

	/** A field that is one input: text, number, boolean. */
	let {
		field,
		values,
		path,
	}: { field: Field; values: Values; path: string } = $props();
</script>

{#if field.kind === 'boolean'}
	<!-- writes only when clicked, so opening an entry changes nothing -->
	<input
		id={path}
		type="checkbox"
		checked={values[field.handle] ?? false}
		onchange={e => (values[field.handle] = e.currentTarget.checked)}
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
{/if}
