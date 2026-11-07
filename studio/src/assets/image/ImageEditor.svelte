<script lang="ts">
	import { originalFile, type Edits } from '../api';
	import { NO_EDITS, cropsOutside, noEdits, transparentEdges } from './edits';
	import Icon from 'zipp-ui/Icon.svelte';
	import Checkbox from 'zipp-ui/Checkbox.svelte';
	import Segmented from 'zipp-ui/Segmented.svelte';
	import BackgroundPicker from './BackgroundPicker.svelte';
	import CropStage from './CropStage.svelte';
	import SizeOptions from './SizeOptions.svelte';
	import { drawEdited } from './draw';
	import { t } from '../../i18n/index.svelte';
	import {
		arrowStep,
		flipped,
		focalAt,
		movedFocal,
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
		id,
		edits = $bindable(),
		focal = $bindable(),
		editable,
	}: {
		/** the asset, its uploaded file is drawn */
		id: number;
		edits: Edits;
		focal: { x: number; y: number };
		editable: boolean;
	} = $props();

	type Mode = 'focal' | 'crop' | 'size';
	let mode = $state<Mode>('focal');
	const modes: { value: Mode; label: string; icon: string }[] = $derived([
		{ value: 'focal', label: t('assets.focalPoint'), icon: 'focal' },
		{ value: 'crop', label: t('assets.crop'), icon: 'crop' },
		{ value: 'size', label: t('assets.size'), icon: 'size' },
	]);

	let img = $state<HTMLImageElement | null>(null);
	// with the session, the public url has the edits; once loaded the
	// image draws without its url
	$effect(() => {
		originalFile(id).then(blob => {
			const i = new Image();
			i.onload = () => {
				img = i;
				URL.revokeObjectURL(i.src);
			};
			i.src = URL.createObjectURL(blob);
		});
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

	const ratios: [string, number | null][] = $derived([
		[t('assets.free'), null],
		[t('assets.original'), 0],
		[t('assets.square'), 1],
		['16:9', 16 / 9],
		['4:3', 4 / 3],
		['3:2', 3 / 2],
	]);
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

	// the point also moves with the arrow keys, when it has the focus
	function moveFocal(e: KeyboardEvent) {
		const step = arrowStep(e);
		if (!editable || !step) return;
		e.preventDefault();
		focal = movedFocal(focal, step.dx, step.dy);
	}

	const percent = (v: number) => Math.round(v * 100);
</script>

<div class="editor">
	{#if editable}
		<div class="tools" role="toolbar" aria-label={t('assets.editImage')}>
			<Segmented
				options={modes}
				value={mode}
				onchange={m => (mode = m)}
				label={t('assets.tool')}
			>
				{#snippet item(o)}
					<Icon
						name={modes.find(m => m.value === o.value)!.icon}
						size={15}
					/>
					{o.label}
				{/snippet}
			</Segmented>
			<span class="toolbar-sep"></span>
			<button
				type="button"
				class="icon"
				title={t('assets.rotateLeft')}
				aria-label={t('assets.rotateLeft')}
				onclick={() => rotate(-90)}
			>
				<Icon name="rotate-left" />
			</button>
			<button
				type="button"
				class="icon"
				title={t('assets.rotateRight')}
				aria-label={t('assets.rotateRight')}
				onclick={() => rotate(90)}
			>
				<Icon name="rotate-right" />
			</button>
			<button
				type="button"
				class="icon"
				class:active={edits.flipH}
				title={t('assets.flipH')}
				aria-label={t('assets.flipH')}
				onclick={() => (edits = flipped(edits, 'flipH'))}
			>
				<Icon name="flip-h" />
			</button>
			<button
				type="button"
				class="icon"
				class:active={edits.flipV}
				title={t('assets.flipV')}
				aria-label={t('assets.flipV')}
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
				title={t('assets.resetTitle')}
			>
				<Icon name="reset" size={14} />
				{t('assets.reset')}
			</button>
		</div>
	{/if}

	{#if mode === 'crop'}
		<div class="options">
			<Segmented
				options={ratios.map(([label, value]) => ({
					value: String(value),
					label,
				}))}
				value={String(ratio)}
				onchange={v => {
					ratio = v === 'null' ? null : +v;
					applyRatio();
				}}
				label={t('assets.ratio')}
			/>
			{#if ratio !== null && ratio !== 0 && ratio !== 1}
				<Checkbox bind:checked={portrait} onchange={applyRatio}>
					{t('assets.portrait')}
				</Checkbox>
			{/if}
			{#if outside}
				<span class="toolbar-sep"></span>
				<BackgroundPicker bind:edits />
			{/if}
		</div>
		<p class="note">{t('assets.cropNote')}</p>
	{:else if mode === 'size'}
		<SizeOptions bind:edits resized={size.resized} />
	{/if}
	{#if mode !== 'focal' && transparentEdges(edits)}
		<p class="note">{t('assets.transparentNote')}</p>
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
					<!-- for the keyboard a slider, read only without edit -->
					<span
						class="point"
						role="slider"
						tabindex={editable ? 0 : -1}
						aria-label={t('assets.focalPoint')}
						aria-readonly={!editable}
						aria-valuenow={percent(focal.x)}
						aria-valuetext={t('assets.focalAt', {
							x: percent(focal.x),
							y: percent(focal.y),
						})}
						style:left="{focal.x * 100}%"
						style:top="{focal.y * 100}%"
						onkeydown={moveFocal}
					></span>
				{/if}
			</div>
		{/if}
		{#if !img}<p class="empty">{t('common.loading')}</p>{/if}
	</div>
	{#if mode === 'focal' && editable}
		<p class="note">{t('assets.focalNote')}</p>
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

		&:focus-visible {
			outline: 2px solid var(--accent);
			outline-offset: 2px;
		}
	}
</style>
