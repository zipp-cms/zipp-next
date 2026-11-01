<script lang="ts">
	import type { FieldOf, Values } from '../core/schema';

	/**
	 * A number with its unit after the input. With `decimals` it steps
	 * by them and shows them, `12.50`; leaving it rounds to them.
	 */
	let {
		field,
		values,
		path,
	}: {
		field: FieldOf<'number'>;
		values: Values;
		path: string;
	} = $props();

	let input: HTMLInputElement;
	const decimals = $derived(field.decimals);

	// also when the value comes from elsewhere (a revision, the AI), but
	// not while it is typed
	$effect(() => {
		const v = values[field.handle];
		if (
			decimals !== undefined &&
			typeof v === 'number' &&
			document.activeElement !== input
		) {
			input.value = v.toFixed(decimals);
		}
	});

	function onblur() {
		const v = values[field.handle];
		if (decimals === undefined || typeof v !== 'number') return;
		const rounded = Number(v.toFixed(decimals));
		// writes only what changes, opening and leaving makes no draft
		if (rounded !== v) values[field.handle] = rounded;
		input.value = rounded.toFixed(decimals);
	}
</script>

<div class="number">
	<input
		bind:this={input}
		id={path}
		type="number"
		min={field.min ?? undefined}
		max={field.max ?? undefined}
		step={decimals === undefined ? undefined : 10 ** -decimals}
		bind:value={values[field.handle]}
		{onblur}
	/>
	{#if field.unit}
		<span class="unit">{field.unit}</span>
	{/if}
</div>

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
</style>
