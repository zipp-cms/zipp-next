<script lang="ts">
	import type { FieldOf, Values } from '../core/schema';

	/** A field that is one input: text, email, boolean. */
	let {
		field,
		values,
		path,
	}: {
		field: FieldOf<'text' | 'email' | 'boolean'>;
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
{:else if field.kind === 'email'}
	<input id={path} type="email" bind:value={values[field.handle]} />
{:else}
	<input
		id={path}
		maxlength={field.max ?? undefined}
		bind:value={values[field.handle]}
	/>
{/if}
