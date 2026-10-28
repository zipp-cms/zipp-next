<script lang="ts">
	import type { Field, Values } from '../api';

	/**
	 * One of the options: up to three as buttons, more as a select. A
	 * click on the picked button clears it unless the field is required.
	 */
	let { field, values }: { field: Field; values: Values } = $props();

	const options = $derived(field.kind === 'dropdown' ? field.options : []);
	const value = $derived(values[field.handle] ?? null);

	function pick(v: string | null) {
		values[field.handle] = v === value && !field.required ? null : v;
	}
</script>

{#if options.length <= 3}
	<div class="buttons" role="radiogroup" aria-label={field.label}>
		{#each options as o (o.value)}
			<button
				type="button"
				role="radio"
				aria-checked={o.value === value}
				class:on={o.value === value}
				onclick={() => pick(o.value)}
			>
				{o.label}
			</button>
		{/each}
	</div>
{:else}
	<select
		value={value ?? ''}
		onchange={e => (values[field.handle] = e.currentTarget.value || null)}
	>
		<option value="" disabled={field.required}>
			{field.required ? 'Choose…' : 'None'}
		</option>
		{#each options as o (o.value)}
			<option value={o.value}>{o.label}</option>
		{/each}
	</select>
{/if}

<style lang="scss">
	.buttons {
		display: flex;
		flex-wrap: wrap;
		gap: 6px;
	}

	button {
		padding: 8px 12px;
		border-color: transparent;
		background: var(--input);
		color: inherit;
		transition:
			background-color 0.2s ease,
			color 0.2s ease;

		&:hover:not(:disabled) {
			border-color: var(--dark);
		}
	}

	.on {
		background: var(--dark);
		color: white;
	}

	select {
		width: auto;
		min-width: 200px;
	}
</style>
