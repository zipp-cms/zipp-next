<script lang="ts">
	import type { Column, Link, Schema } from '../core/schema';
	import type { Refs } from '../entries/api';
	import DatePicker from 'zipp-ui/DatePicker.svelte';
	import Switch from 'zipp-ui/Switch.svelte';
	import LinkInput from './link/LinkInput.svelte';

	/**
	 * A cell of a table, edited like the field of its column's kind but
	 * small enough for a row. `onchange` gets null for nothing.
	 */
	let {
		schema,
		refs,
		column,
		value,
		onchange,
	}: {
		/** links need it, a plugin's module has none */
		schema?: Schema;
		refs: Refs;
		column: Column;
		value: unknown;
		onchange: (value: unknown) => void;
	} = $props();
</script>

{#if column.kind === 'number'}
	<!-- a binding doesn't clear what is half typed, `1.` -->
	<div class="number">
		<input
			type="number"
			step={column.decimals === undefined
				? undefined
				: 10 ** -column.decimals}
			bind:value={() => value as number | null, v => onchange(v ?? null)}
		/>
		{#if column.unit}<span class="unit">{column.unit}</span>{/if}
	</div>
{:else if column.kind === 'boolean'}
	<!-- no is nothing, like the server stores it -->
	<Switch checked={value === true} onchange={c => onchange(c || null)} />
{:else if column.kind === 'date'}
	<DatePicker
		value={(value as string | null) ?? ''}
		label={column.label}
		onchange={v => onchange(v || null)}
	/>
{:else if column.kind === 'dropdown'}
	<select
		value={value ?? ''}
		onchange={e => onchange(e.currentTarget.value || null)}
	>
		<option value="">–</option>
		{#each column.options as o (o.value)}
			<option value={o.value}>{o.label}</option>
		{/each}
	</select>
{:else if column.kind === 'link' && schema}
	<div class="link">
		<LinkInput
			{schema}
			{refs}
			link={(value as Link | null) ?? { type: column.to[0], value: null }}
			to={column.to}
			onchange={l => onchange(l.value === null ? null : l)}
		/>
	</div>
{:else if column.kind === 'link'}
	<span class="muted">–</span>
{:else}
	<input
		type={column.kind === 'email' ? 'email' : 'text'}
		placeholder={column.kind === 'url' ? 'https://…' : ''}
		value={value ?? ''}
		oninput={e => onchange(e.currentTarget.value)}
	/>
{/if}

<style lang="scss">
	.number {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.unit {
		color: var(--muted);
		white-space: nowrap;
	}

	// a link's row and its settings need some room
	.link {
		min-width: 16rem;
	}
</style>
