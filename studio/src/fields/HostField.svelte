<script lang="ts">
	import type { Field, Values } from '../core/schema';
	import { t } from '../i18n/index.svelte';
	import { synced } from '../core/synced.svelte';
	import { hostedComponent } from './registry';

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

	// a field takes the object its value is in
	const values = synced<Values>(
		() => ({ [field.handle]: value ?? undefined }),
		v => onchange(v[field.handle] ?? null),
	);

	const Input = $derived(hostedComponent(field));
</script>

{#if Input}
	<Input {field} values={values.value} path={field.handle} />
{:else}
	<span class="error">{t('fields.notHosted', { kind: field.kind })}</span>
{/if}
