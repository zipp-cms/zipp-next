<script>
	/**
	 * A row of buttons, each in the color of its option. A click picks
	 * one, a click on the picked one clears the field unless it is
	 * required.
	 */
	let { field, value, onchange } = $props();

	function pick(v) {
		onchange(v === value && !field.required ? null : v);
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

<style>
	.group {
		display: flex;
		flex-wrap: wrap;
		gap: 6px;
	}

	button {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 8px 12px;
		font: inherit;
		font-size: 14px;
		border: 1px solid transparent;
		border-radius: 2px;
		background: var(--input, #e9e9ec);
		color: inherit;
		cursor: pointer;
		transition:
			background-color 0.2s ease,
			color 0.2s ease;
	}

	button:hover:not(:disabled) {
		border-color: var(--color);
	}

	.dot {
		width: 10px;
		height: 10px;
		border-radius: 50%;
		background: var(--color);
	}

	.on {
		background: var(--color);
		color: white;
	}

	.on .dot {
		background: white;
	}

	button:disabled {
		cursor: default;
		opacity: 0.8;
	}
</style>
