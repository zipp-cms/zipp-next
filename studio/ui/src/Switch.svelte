<script lang="ts">
	import type { Snippet } from 'svelte';

	/**
	 * A switch for yes or no, its text on the right: a boolean field,
	 * Published. A real checkbox with the role of a switch, so Space
	 * and a disabled fieldset around it work like on any input.
	 */
	let {
		checked = $bindable(false),
		onchange,
		id,
		disabled = false,
		children,
	}: {
		checked?: boolean;
		/** only when clicked, a value isn't written on mount */
		onchange?: (checked: boolean) => void;
		id?: string;
		disabled?: boolean;
		children?: Snippet;
	} = $props();
</script>

<label class="switch">
	<input
		{id}
		type="checkbox"
		role="switch"
		bind:checked
		{disabled}
		onchange={e => onchange?.(e.currentTarget.checked)}
	/>
	<span class="track" aria-hidden="true"></span>
	{#if children}<span class="text">{@render children()}</span>{/if}
</label>

<style lang="scss">
	@use '../vars' as *;

	// as high as an input, so it lines up with the label of its field
	.switch {
		position: relative;
		display: inline-flex;
		align-items: center;
		gap: 0.625rem;
		min-height: 2.125rem;
		align-self: flex-start;
		cursor: pointer;
	}

	// the input stays for the keyboard and screen readers, the track
	// shows it
	input {
		@include visually-hidden;
	}

	.track {
		position: relative;
		flex-shrink: 0;
		width: 2.25rem;
		height: 1.25rem;
		border-radius: 0.625rem;
		background: var(--border-strong);
		transition: background-color 0.15s ease;

		&::after {
			content: '';
			position: absolute;
			top: 2px;
			left: 2px;
			width: 1rem;
			height: 1rem;
			border-radius: 50%;
			background: var(--surface);
			box-shadow: 0 1px 2px rgba(16, 24, 40, 0.2);
			transition: transform 0.15s ease;
		}
	}

	input:checked + .track {
		background: var(--accent);

		&::after {
			transform: translateX(1rem);
		}
	}

	input:focus-visible + .track {
		@include focus-ring;
	}

	@include hover {
		.switch:hover input:not(:disabled):not(:checked) + .track {
			background: var(--subtle);
		}

		.switch:hover input:not(:disabled):checked + .track {
			background: var(--accent-hover);
		}
	}

	.switch:has(input:disabled) {
		cursor: default;

		.track {
			opacity: 0.5;
		}
	}
</style>
