<script lang="ts">
	import type { FieldOf, Values } from '../core/schema';
	import { t } from '../i18n/index.svelte';
	import { valueOf } from '../core/schema';
	import Segmented from 'zipp-ui/Segmented.svelte';

	/**
	 * One of the options: up to three as buttons, more as a select. A
	 * click on the picked button clears it unless the field is required.
	 */
	let {
		field,
		values,
		path,
	}: { field: FieldOf<'dropdown'>; values: Values; path: string } = $props();

	const options = $derived(field.options);
	const value = $derived(valueOf(values, field) ?? null);

	function pick(v: string | null) {
		values[field.handle] = v === value && !field.required ? null : v;
	}
</script>

{#if options.length <= 3}
	<Segmented {options} {value} onchange={pick} label={field.label} />
{:else}
	<select
		id={path}
		value={value ?? ''}
		onchange={e => (values[field.handle] = e.currentTarget.value || null)}
	>
		<option value="" disabled={field.required}>
			{field.required ? t('fields.chooseOption') : t('common.none')}
		</option>
		{#each options as o (o.value)}
			<option value={o.value}>{o.label}</option>
		{/each}
	</select>
{/if}

<style lang="scss">
	select {
		width: auto;
		min-width: 12.5rem;
	}
</style>
