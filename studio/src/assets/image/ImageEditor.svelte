<script lang="ts">
	import {
		NO_EDITS,
		cropsOutside,
		noEdits,
		transparentEdges,
		type Edits,
	} from '../api';
	import Icon from 'zipp-ui/Icon.svelte';
	import Segmented from 'zipp-ui/Segmented.svelte';
	import BackgroundPicker from './BackgroundPicker.svelte';
	import CropStage from './CropStage.svelte';
	import SizeOptions from './SizeOptions.svelte';
	import { drawEdited } from './draw';
	import {
		flipped,
		focalAt,
		ratioCrop,
		rotated,
		sizes,
		wantedRatio,
		withCrop,
	} from './edits';

	/**
	 * Edits an image without changing the file, like Craft's image editor:
	 * crop with a rectangle, rotate, flip, resize, a canvas around it, the
	 * focal point. It draws the result the way the server makes it.
	 */
	let {
		src,
		edits = $bindable(),
		focal = $bindable(),
		editable,
	}: {
		/** the uploaded file */
		src: string;
		edits: Edits;
		focal: { x: number; y: number };
		editable: boolean;
	} = $props();

	type Mode = 'focal' | 'crop' | 'size';
	let mode = $state<Mode>('focal');
	const MODES: { value: Mode; label: string; icon: string }[] = [
		{ value: 'focal', label: 'Focal point', icon: 'focal' },
		{ value: 'crop', label: 'Crop', icon: 'crop' },
		{ value: 'size', label: 'Size', icon: 'size' },
	];

	let img = $state<HTMLImageElement | null>(null);
	$effect(() => {
		const i = new Image();
		i.onload = () => (img = i);
		i.src = src;
	});

	const size = $derived(
		sizes(
			img
				? { w: img.naturalWidth, h: img.naturalHeight }
				: { w: 1, h: 1 },
			edits,
		),
	);
	const outside = $derived(cropsOutside(edits));

	// the result, when not cropping
	let view = $state<HTMLCanvasElement>();
	$effect(() => {
		if (img && view) drawEdited(view, img, edits, outside);
	});

	const RATIOS: [string, number | null][] = [
		['Free', null],
		['Original', 0],
		['Square', 1],
		['16:9', 16 / 9],
		['4:3', 4 / 3],
		['3:2', 3 / 2],
	];
	let ratio = $state<number | null>(null);
	let portrait = $state(false);
	const want = $derived(wantedRatio(ratio, portrait, size.turned));

	function applyRatio() {
		if (want !== null)
			edits = withCrop(edits, ratioCrop(want, size.turned));
	}

	function rotate(by: number) {
		// a 16:9 crop turned is a 9:16 one
		if (ratio) portrait = !portrait;
		edits = rotated(edits, by);
	}

	function setFocal(e: MouseEvent) {
		if (editable && mode === 'focal' && view) focal = focalAt(e, view);
	}
</script>

<div class="editor">
	{#if editable}
		<div class="tools" role="toolbar" aria-label="Edit the image">
			<Segmented
				options={MODES}
				value={mode}
				onchange={m => (mode = m)}
				label="Tool"
			>
				{#snippet item(o)}
					<Icon
						name={MODES.find(m => m.value === o.value)!.icon}
						size={15}
					/>
					{o.label}
				{/snippet}
			</Segmented>
			<span class="sep"></span>
			<button
				type="button"
				class="icon"
				title="Rotate left"
				onclick={() => rotate(-90)}
			>
				<Icon name="rotate-left" />
			</button>
			<button
				type="button"
				class="icon"
				title="Rotate right"
				onclick={() => rotate(90)}
			>
				<Icon name="rotate-right" />
			</button>
			<button
				type="button"
				class="icon"
				class:active={edits.flipH}
				title="Flip horizontally"
				onclick={() => (edits = flipped(edits, 'flipH'))}
			>
				<Icon name="flip-h" />
			</button>
			<button
				type="button"
				class="icon"
				class:active={edits.flipV}
				title="Flip vertically"
				onclick={() => (edits = flipped(edits, 'flipV'))}
			>
				<Icon name="flip-v" />
			</button>
			<span class="size">{size.final.w} × {size.final.h}</span>
			<button
				type="button"
				class="link"
				disabled={noEdits(edits)}
				onclick={() => (edits = { ...NO_EDITS })}
				title="Back to the uploaded image"
			>
				<Icon name="reset" size={14} /> Reset
			</button>
		</div>
	{/if}

	{#if mode === 'crop'}
		<div class="options">
			<Segmented
				options={RATIOS.map(([label, value]) => ({
					value: String(value),
					label,
				}))}
				value={String(ratio)}
				onchange={v => {
					ratio = v === 'null' ? null : +v;
					applyRatio();
				}}
				label="Ratio"
			/>
			{#if ratio !== null && ratio !== 0 && ratio !== 1}
				<label class="check">
					<input
						type="checkbox"
						bind:checked={portrait}
						onchange={applyRatio}
					/>
					Portrait
				</label>
			{/if}
			{#if outside}
				<span class="sep"></span>
				<BackgroundPicker bind:edits />
			{/if}
		</div>
		<p class="note">Drag the crop past the image for room around it.</p>
	{:else if mode === 'size'}
		<SizeOptions bind:edits resized={size.resized} />
	{/if}
	{#if mode !== 'focal' && transparentEdges(edits)}
		<p class="note">
			Transparent needs a PNG: a JPEG is delivered as PNG then.
		</p>
	{/if}

	<div class="stage checkerboard" class:cropping={mode === 'crop'}>
		{#if mode === 'crop'}
			{#if img}
				<CropStage
					{img}
					bind:edits
					turned={size.turned}
					{want}
					{editable}
				/>
			{/if}
		{:else}
			<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
			<div
				class="frame"
				class:focal={mode === 'focal' && editable}
				onclick={setFocal}
			>
				<canvas bind:this={view}></canvas>
				{#if mode === 'focal'}
					<span
						class="point"
						style:left="{focal.x * 100}%"
						style:top="{focal.y * 100}%"
					></span>
				{/if}
			</div>
		{/if}
		{#if !img}<p class="empty">Loading…</p>{/if}
	</div>
	{#if mode === 'focal' && editable}
		<p class="note">
			Click on the image to set the focal point, crops of the sizes keep
			it in view.
		</p>
	{/if}
</div>

<style lang="scss">
	.editor {
		display: flex;
		flex-direction: column;
		gap: 0.625rem;
	}

	.tools,
	.options {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 0.25rem;
	}

	.tools {
		padding: 0.25rem;
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
	}

	.active {
		color: var(--accent);
	}

	.size {
		margin-left: auto;
		color: var(--muted);
		font-family: var(--mono);
		font-size: var(--fs-xs);
	}

	.note {
		margin: 0;
	}

	.stage {
		display: grid;
		place-items: center;
		padding: 1rem;
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);

		// the whole stage is the crop area
		&.cropping {
			display: block;
			padding: 0;
			overflow: hidden;
		}
	}

	.frame {
		position: relative;
		line-height: 0;

		&.focal {
			cursor: crosshair;
		}
	}

	canvas {
		display: block;
		max-width: 100%;
		max-height: 60vh;
		box-shadow: 0 0 0 1px var(--line);
	}

	.point {
		position: absolute;
		width: 1.225rem;
		height: 1.225rem;
		margin: -0.6125rem 0 0 -0.6125rem;
		border: 3px solid white;
		border-radius: 50%;
		box-shadow: 0 0 0 2px rgba(0, 0, 0, 0.5);
		pointer-events: none;
	}
</style>
