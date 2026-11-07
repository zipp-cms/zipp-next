<script lang="ts">
	import type { FieldOf, Values } from '../core/schema';
	import { valueOf } from '../core/schema';
	import SchemaIcon from '../core/SchemaIcon.svelte';
	import Segmented from 'zipp-ui/Segmented.svelte';

	/**
	 * The options side by side like the studio's own choices, each with
	 * the color (a dot) and the icon of its option if it has them. A click
	 * picks one, a click on the picked one clears the field unless it is
	 * required.
	 */
	let { field, values }: { field: FieldOf<'buttonGroup'>; values: Values } =
		$props();

	const value = $derived(valueOf(values, field) ?? null);

	function pick(v: string | null) {
		values[field.handle] = v === value && !field.required ? null : v;
	}
</script>

<Segmented options={field.options} {value} onchange={pick} label={field.label}>
	{#snippet item({ value: v, label })}
		{@const o = field.options.find(o => o.value === v)}
		{#if o?.color}<span class="dot" style:background={o.color}></span>{/if}
		{#if o?.icon}<SchemaIcon icon={o.icon} />{/if}
		{label}
	{/snippet}
</Segmented>

<style lang="scss">
	.dot {
		flex-shrink: 0;
		width: 0.625rem;
		height: 0.625rem;
		border-radius: 50%;
	}
</style>
