<script lang="ts">
	import type { Edits } from '../api';
	import Checkbox from 'zipp-ui/Checkbox.svelte';
	import BackgroundPicker from './BackgroundPicker.svelte';
	import { withCanvas, withResize, type Size } from './edits';
	import { t } from '../../i18n/index.svelte';

	/** The size of the image, and a canvas around it. */
	let { edits = $bindable(), resized }: { edits: Edits; resized: Size } =
		$props();
</script>

<div class="options">
	<label class="stacked">
		{t('assets.width')}
		<input
			type="number"
			min="1"
			max="10000"
			value={resized.w}
			onchange={e =>
				(edits = withResize(
					edits,
					+e.currentTarget.value,
					edits.resize?.height ?? null,
				))}
		/>
	</label>
	<label class="stacked">
		{t('assets.height')}
		<input
			type="number"
			min="1"
			max="10000"
			value={resized.h}
			disabled={!edits.resize}
			onchange={e =>
				(edits = withResize(
					edits,
					edits.resize?.width ?? resized.w,
					+e.currentTarget.value,
				))}
		/>
	</label>
	{#if edits.resize}
		<button
			type="button"
			class="link"
			onclick={() => (edits = withResize(edits, null, null))}
		>
			{t('assets.originalSize')}
		</button>
	{/if}
	<Checkbox
		checked={!!edits.canvas}
		onchange={on => (edits = withCanvas(edits, on, resized))}
	>
		{t('assets.canvas')}
	</Checkbox>
	{#if edits.canvas}
		<label class="stacked">
			{t('assets.canvasWidth')}
			<input
				type="number"
				min="1"
				max="10000"
				bind:value={edits.canvas.width}
			/>
		</label>
		<label class="stacked">
			{t('assets.canvasHeight')}
			<input
				type="number"
				min="1"
				max="10000"
				bind:value={edits.canvas.height}
			/>
		</label>
		<BackgroundPicker bind:edits />
	{/if}
</div>

<style lang="scss">
	.options {
		display: flex;
		flex-wrap: wrap;
		align-items: flex-end;
		gap: 0.625rem;
	}

	input[type='number'] {
		width: 6.125rem;
	}

	// a box as high as the inputs next to it, around the Checkbox
	.options :global(label.check) {
		align-self: auto;
		padding: 0.4375rem 0.75rem;
		border: 1px solid var(--border-strong);
		border-radius: var(--radius);
		background: var(--input);
		white-space: nowrap;

		&:has(:checked) {
			border-color: var(--accent);
			background: var(--accent-soft);
		}
	}
</style>
