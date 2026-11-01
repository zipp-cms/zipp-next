<script lang="ts">
	import type { FieldOf, Values } from '../core/schema';
	import Switch from 'zipp-ui/Switch.svelte';

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
	<Switch
		id={path}
		checked={!!values[field.handle]}
		onchange={on => (values[field.handle] = on)}
	>
		<span class="muted">{values[field.handle] ? 'Yes' : 'No'}</span>
	</Switch>
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
