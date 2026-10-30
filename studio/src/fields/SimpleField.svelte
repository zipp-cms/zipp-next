<script lang="ts">
	import type { FieldOf, Values } from '../core/schema';

	/** A field that is one input: text, number, boolean. */
	let {
		field,
		values,
		path,
	}: {
		field: FieldOf<'text' | 'number' | 'boolean'>;
		values: Values;
		path: string;
	} = $props();
</script>

{#if field.kind === 'boolean'}
	<!-- writes only when clicked, so opening an entry changes nothing -->
	<input
		id={path}
		type="checkbox"
		checked={!!values[field.handle]}
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
{:else}
	<input
		id={path}
		type="number"
		min={field.min ?? undefined}
		max={field.max ?? undefined}
		bind:value={values[field.handle]}
	/>
{/if}
