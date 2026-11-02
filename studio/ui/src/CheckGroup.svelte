<script lang="ts">
	import Icon from './Icon.svelte';

	/**
	 * Some of a few choices, as a row of chips that wraps: a checkboxes
	 * field. A chosen one is on the accent with a check. Each is a real
	 * checkbox, so Tab, Space and a disabled fieldset work as usual.
	 */
	let {
		options,
		picked,
		ontoggle,
		label,
	}: {
		options: { value: string; label: string }[];
		picked: string[];
		ontoggle: (value: string, on: boolean) => void;
		label?: string;
	} = $props();
</script>

<div class="group" role="group" aria-label={label}>
	{#each options as o (o.value)}
		{@const on = picked.includes(o.value)}
		<label class="option" class:on>
			<input
				type="checkbox"
				checked={on}
				onchange={e => ontoggle(o.value, e.currentTarget.checked)}
			/>
			<span class="tick" aria-hidden="true">
				{#if on}<Icon name="check" size={12} />{/if}
			</span>
			{o.label}
		</label>
	{/each}
</div>

<style lang="scss">
	@use '../vars' as *;

	.group {
		display: flex;
		flex-wrap: wrap;
		gap: 0.375rem;
	}

	.option {
		position: relative;
		display: inline-flex;
		align-items: center;
		gap: 0.4375rem;
		min-height: 2.125rem;
		padding: 0.3125rem 0.875rem 0.3125rem 0.625rem;
		border: 1px solid var(--border-strong);
		border-radius: 1.0625rem;
		background: var(--surface);
		font-size: var(--fs-md);
		font-weight: 500;
		cursor: pointer;
		transition:
			background-color 0.15s ease,
			border-color 0.15s ease;

		&.on {
			border-color: var(--accent);
			background: var(--accent);
			color: var(--on-accent);
		}

		&:has(input:focus-visible) {
			@include focus-ring;
		}

		&:has(input:disabled) {
			cursor: default;
			opacity: 0.6;
		}
	}

	@include hover {
		.option:hover:not(.on):not(:has(input:disabled)) {
			border-color: var(--accent);
			background: var(--accent-soft);
		}

		.option.on:hover:not(:has(input:disabled)) {
			border-color: var(--accent-hover);
			background: var(--accent-hover);
		}
	}

	// the input stays for the keyboard and screen readers, the tick
	// shows it
	input {
		@include visually-hidden;
	}

	.tick {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 0.875rem;
		height: 0.875rem;
		border: 1px solid var(--border-strong);
		border-radius: var(--radius-sm);
		background: var(--input);

		.on & {
			border-color: var(--on-accent);
			background: var(--on-accent);
			color: var(--accent);
		}
	}
</style>
