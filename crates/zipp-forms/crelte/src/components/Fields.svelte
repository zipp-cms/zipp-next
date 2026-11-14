<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { FieldData } from '../types';
	import { getForm } from '../context';
	import Field from './Field.svelte';

	/**
	 * The fields of the page on screen in their order, each a `Field` or
	 * `field`, in rows by their widths: fields side by side that fit
	 * share a row, on narrow screens each has its own. The gap is
	 * `--zipp-form-gap` (1rem).
	 */
	let { field }: { field?: Snippet<[FieldData]> } = $props();

	const form = getForm();
</script>

<div class="zipp-form-fields">
	{#each form.fields as f (f.handle)}
		<div class="zipp-form-cell" style:--zipp-form-width={f.width / 100}>
			{#if field}
				{@render field(f)}
			{:else}
				<Field field={f} />
			{/if}
		</div>
	{/each}
</div>

<style>
	.zipp-form-fields {
		--gap: var(--zipp-form-gap, 1rem);
		display: flex;
		flex-wrap: wrap;
		gap: var(--gap);
	}

	.zipp-form-cell {
		flex: 0 0
			calc(
				var(--zipp-form-width) * 100% - var(--gap) *
					(1 - var(--zipp-form-width))
			);
		min-width: 0;
	}

	@media (max-width: 40rem) {
		.zipp-form-cell {
			flex-basis: 100%;
		}
	}
</style>
