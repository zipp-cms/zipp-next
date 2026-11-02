<script lang="ts">
	import type { Snippet } from 'svelte';

	/**
	 * A field in a `FieldGroup` like the fields of a group or a block: its
	 * label on the left, above it in narrow places. `info` goes under the
	 * label, like a handle or a count of characters. Without a label the
	 * value sits where the values of the other rows are.
	 */
	let {
		label = '',
		for: id,
		info,
		children,
	}: {
		label?: string;
		/** the id of the input the label belongs to */
		for?: string;
		info?: Snippet;
		children: Snippet;
	} = $props();
</script>

<div class="field-row">
	{#if label}
		<label for={id}>
			<span>{label}</span>
			{#if info}<span class="info">{@render info()}</span>{/if}
		</label>
	{/if}
	<div class="value" class:alone={!label}>
		{@render children()}
	</div>
</div>

<style lang="scss">
	@use '../vars' as *;

	// the look of the studio's fields inside a block (fields/Fields.svelte)
	.field-row {
		display: grid;
		grid-template-columns: $label-col minmax(0, 1fr);
		gap: $label-gap;
		align-items: center;
		min-width: 0;
	}

	// at the top of tall fields, the first line of a field is 2.125rem high
	label {
		align-self: start;
		display: flex;
		flex-direction: column;
		gap: 1px;
		padding-top: 0.4375rem;
		font-size: var(--fs-md);
		font-weight: 600;
		color: var(--text);
	}

	.info {
		color: var(--muted);
		font-size: var(--fs-sm);
		font-weight: 400;
	}

	.value {
		min-width: 0;
	}

	.alone {
		grid-column: 2;
	}

	@include max-tablet {
		.field-row {
			display: flex;
			flex-direction: column;
			align-items: stretch;
			gap: 0.375rem;
		}

		label {
			padding-top: 0;
		}
	}
</style>
