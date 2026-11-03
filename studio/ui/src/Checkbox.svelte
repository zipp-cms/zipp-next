<script lang="ts">
	import type { Snippet } from 'svelte';
	import Icon from './Icon.svelte';

	/**
	 * A checkbox with its label on the right and a hint under it, the
	 * tick of CheckGroup. A real checkbox, so Space, a form and a
	 * disabled fieldset work as usual.
	 */
	let {
		checked = $bindable(false),
		onchange,
		id,
		disabled = false,
		label,
		hint,
		children,
	}: {
		checked?: boolean;
		/** only when clicked, a value isn't written on mount */
		onchange?: (checked: boolean) => void;
		id?: string;
		disabled?: boolean;
		/** for screen readers when there is no text, e.g. in a table */
		label?: string;
		hint?: string;
		children?: Snippet;
	} = $props();
</script>

<!-- .check stays for the tests -->
<label class="check">
	<input
		{id}
		type="checkbox"
		aria-label={label}
		bind:checked
		{disabled}
		onchange={e => onchange?.(e.currentTarget.checked)}
	/>
	<span class="tick" aria-hidden="true">
		{#if checked}<Icon name="check" size={12} />{/if}
	</span>
	{#if children || hint}
		<span class="text">
			{@render children?.()}
			{#if hint}<span class="note">{hint}</span>{/if}
		</span>
	{/if}
</label>

<style lang="scss">
	@use '../vars' as *;

	.check {
		position: relative;
		display: inline-flex;
		align-items: flex-start;
		gap: 0.5rem;
		align-self: flex-start;
		cursor: pointer;
	}

	// the input stays for the keyboard and screen readers, the tick
	// shows it
	input {
		@include visually-hidden;
	}

	// in the middle of the first line, also with a hint under it
	.tick {
		@include check-tick;
		margin-top: calc((1lh - 0.875rem) / 2);
	}

	input:checked + .tick {
		@include check-tick-on;
	}

	input:focus-visible + .tick {
		@include focus-ring;
	}

	.text {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	@include hover {
		.check:hover input:not(:disabled):not(:checked) + .tick {
			border-color: var(--accent);
			background: var(--accent-soft);
		}

		.check:hover input:not(:disabled):checked + .tick {
			border-color: var(--accent-hover);
			background: var(--accent-hover);
		}
	}

	.check:has(input:disabled) {
		cursor: default;
		opacity: 0.6;
	}
</style>
