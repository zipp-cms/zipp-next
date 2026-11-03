<script lang="ts">
	import type { FieldOf, Values } from '../core/schema';
	import { valueOf } from '../core/schema';

	/**
	 * A row of buttons, each in the color of its option. A click picks
	 * one, a click on the picked one clears the field unless it is
	 * required.
	 */
	let { field, values }: { field: FieldOf<'buttonGroup'>; values: Values } =
		$props();

	const value = $derived(valueOf(values, field) ?? null);

	function pick(v: string) {
		values[field.handle] = v === value && !field.required ? null : v;
	}
</script>

<div class="group" role="radiogroup" aria-label={field.label}>
	{#each field.options as o (o.value)}
		<button
			type="button"
			role="radio"
			aria-checked={o.value === value}
			class:on={o.value === value}
			style:--color={o.color}
			onclick={() => pick(o.value)}
		>
			<span class="dot"></span>
			{o.label}
		</button>
	{/each}
</div>

<style lang="scss">
	.group {
		display: flex;
		flex-wrap: wrap;
		gap: 0.375rem;
	}

	button {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		font: inherit;
		font-size: var(--fs-base);
		border: 1px solid transparent;
		border-radius: var(--radius-sm);
		background: var(--input);
		color: inherit;
		cursor: pointer;
		transition:
			background-color 0.2s ease,
			color 0.2s ease;

		// the picked one keeps its color, the studio's hover of buttons
		// would make it grey
		@include hover {
			&:hover:not(:disabled) {
				border-color: var(--color);
			}

			&.on:hover:not(:disabled) {
				background: var(--color);
				color: var(--on-accent);
				filter: brightness(0.9);
			}
		}

		&:disabled {
			cursor: default;
			opacity: 0.8;
		}
	}

	.dot {
		width: 0.625rem;
		height: 0.625rem;
		border-radius: 50%;
		background: var(--color);
	}

	.on {
		background: var(--color);
		color: var(--on-accent);

		.dot {
			background: var(--on-accent);
		}
	}
</style>
