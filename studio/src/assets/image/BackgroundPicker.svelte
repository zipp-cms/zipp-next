<script lang="ts">
	import type { Edits } from '../api';

	/** What is around the image, past the crop and on a canvas. */
	let { edits = $bindable() }: { edits: Edits } = $props();

	const background = $derived(edits.background ?? 'transparent');
	// the color of the color button, kept while transparent
	// svelte-ignore state_referenced_locally
	let color = $state(
		edits.background && edits.background !== 'transparent'
			? edits.background
			: '#ffffff',
	);
</script>

<div class="background" role="group" aria-label="Background">
	<span class="note">Background</span>
	<button
		type="button"
		class:on={background === 'transparent'}
		onclick={() => (edits = { ...edits, background: null })}
	>
		<span class="swatch checkerboard"></span>
		Transparent
	</button>
	<!-- the button opens the color picker -->
	<label class="light color" class:on={background !== 'transparent'}>
		<span class="swatch" style:background={color}></span>
		{color}
		<input
			type="color"
			value={color}
			onclick={() => (edits = { ...edits, background: color })}
			oninput={e => {
				color = e.currentTarget.value;
				edits = { ...edits, background: color };
			}}
		/>
	</label>
</div>

<style lang="scss">
	.background {
		display: flex;
		align-items: center;
		gap: 4px;
	}

	.on {
		color: var(--accent);
		background: var(--accent-soft);
		border-color: var(--accent-soft);
	}

	// a label that looks like the buttons, it opens the color picker
	.color {
		position: relative;
		font-family: var(--mono);
		cursor: pointer;

		input {
			position: absolute;
			inset: 0;
			width: 100%;
			height: 100%;
			opacity: 0;
			cursor: pointer;
		}
	}

	.swatch {
		width: 14px;
		height: 14px;
		border-radius: var(--radius-sm);
		box-shadow: inset 0 0 0 1px var(--border-strong);

		&.checkerboard {
			--check: 3.5px;
			--check-color: var(--border-strong);
		}
	}
</style>
