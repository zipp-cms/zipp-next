<script lang="ts">
	import { untrack } from 'svelte';
	import type { Edits } from '../api';
	import { drawTurned } from './draw';
	import {
		WHOLE,
		arrowStep,
		dragged,
		fitScale,
		placeOf,
		withCrop,
		type Handle,
		type Rect,
		type Size,
	} from './edits';
	import { t } from '../../i18n/index.svelte';

	/**
	 * Cropping: the whole stage is the area, the image in its middle; the
	 * crop may reach past the image. Its corners and the middles of its
	 * sides resize it (the sides only without a ratio), Alt resizes around
	 * its middle, and edges near the image's border snap to it. The scale
	 * only changes between drags, so the image doesn't move under the
	 * pointer.
	 */
	let {
		img,
		edits = $bindable(),
		turned,
		want,
		editable,
	}: {
		img: HTMLImageElement;
		edits: Edits;
		turned: Size;
		/** the ratio of the crop, null is free */
		want: number | null;
		editable: boolean;
	} = $props();

	const rect = $derived(edits.crop ?? WHOLE);
	const background = $derived(edits.background ?? 'transparent');

	let view = $state<HTMLCanvasElement>();
	// drawn again only when it turns or flips, not with every move of
	// the crop (edits is a new object then): a primitive stops the effect
	const turn = $derived(`${edits.rotate}|${edits.flipH}|${edits.flipV}`);
	$effect(() => {
		void turn;
		if (view) untrack(() => drawTurned(view!, img, edits));
	});

	let area = $state({ w: 0, h: 0 });
	let scale = $state(0);
	let refit = $state(0);
	$effect(() => {
		void [area.w, area.h, turned.w, turned.h, refit];
		untrack(() => (scale = fitScale(area, turned, rect)));
	});
	const place = $derived(placeOf(area, turned, scale));

	/** how close in pixels an edge snaps to the image's border */
	const SNAP = 8;

	let drag: {
		kind: 'move' | Handle;
		x: number;
		y: number;
		from: Rect;
		/** where the pointer is now, for Alt pressed without moving */
		nowX: number;
		nowY: number;
	} | null = null;

	function down(e: PointerEvent, kind: 'move' | Handle) {
		if (!editable) return;
		e.preventDefault();
		e.stopPropagation();
		(e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
		const [x, y] = [e.clientX, e.clientY];
		drag = { kind, x, y, from: { ...rect }, nowX: x, nowY: y };
	}

	function apply(center: boolean) {
		if (!drag) return;
		const dx = (drag.nowX - drag.x) / place.w;
		const dy = (drag.nowY - drag.y) / place.h;
		const c = dragged(drag.kind, drag.from, dx, dy, place, want, turned, {
			center,
			snap: SNAP,
		});
		edits = withCrop(edits, c);
	}

	function move(e: PointerEvent) {
		if (!drag) return;
		drag.nowX = e.clientX;
		drag.nowY = e.clientY;
		apply(e.altKey);
	}

	// Alt pressed or let go during a drag changes it at once
	function altKey(e: KeyboardEvent) {
		if (drag && e.key === 'Alt') {
			e.preventDefault();
			apply(e.type === 'keydown');
		}
	}

	function up() {
		if (!drag) return;
		drag = null;
		refit++;
	}

	// the arrow keys move the crop or the handle that has the focus
	function onkeydown(e: KeyboardEvent, kind: 'move' | Handle) {
		const step = arrowStep(e);
		if (!editable || !step) return;
		e.preventDefault();
		e.stopPropagation();
		const c = dragged(kind, rect, step.dx, step.dy, place, want, turned, {
			center: e.altKey,
		});
		edits = withCrop(edits, c);
		refit++;
	}

	const percent = (v: number) => Math.round(v * 100);
	// for screen readers, the crop and its corners
	const where = $derived(
		t('assets.cropAt', {
			x: percent(rect.x),
			y: percent(rect.y),
			w: percent(rect.w),
			h: percent(rect.h),
		}),
	);
	const labels = $derived({
		nw: t('assets.cornerNw'),
		ne: t('assets.cornerNe'),
		sw: t('assets.cornerSw'),
		se: t('assets.cornerSe'),
		n: t('assets.sideN'),
		e: t('assets.sideE'),
		s: t('assets.sideS'),
		w: t('assets.sideW'),
	});
	// the sides only without a ratio, a ratio moves both
	const handles = $derived<Handle[]>(
		want === null
			? ['nw', 'n', 'ne', 'e', 'se', 's', 'sw', 'w']
			: ['nw', 'ne', 'se', 'sw'],
	);

	/** Where a fraction rect is on the stage. */
	const at = (r: Rect) => ({
		left: `${place.left + r.x * place.w}px`,
		top: `${place.top + r.y * place.h}px`,
		width: `${r.w * place.w}px`,
		height: `${r.h * place.h}px`,
	});
</script>

<svelte:window onkeydown={altKey} onkeyup={altKey} />

<div class="area" bind:clientWidth={area.w} bind:clientHeight={area.h}>
	{#if scale > 0}
		{@const box = at(rect)}
		{#if background !== 'transparent'}
			<div
				class="fill"
				style:background
				style:left={box.left}
				style:top={box.top}
				style:width={box.width}
				style:height={box.height}
			></div>
		{/if}
		<canvas
			bind:this={view}
			style:left="{place.left}px"
			style:top="{place.top}px"
			style:width="{place.w}px"
			style:height="{place.h}px"
		></canvas>
		<div
			class="crop"
			style:left={box.left}
			style:top={box.top}
			style:width={box.width}
			style:height={box.height}
			onpointerdown={e => down(e, 'move')}
			onpointermove={move}
			onpointerup={up}
			onpointercancel={up}
			onlostpointercapture={up}
			role="slider"
			tabindex={editable ? 0 : -1}
			aria-label={t('assets.crop')}
			aria-valuenow={percent(rect.x)}
			aria-valuetext={where}
			onkeydown={e => onkeydown(e, 'move')}
		>
			{#each handles as h (h)}
				<span
					class="handle {h}"
					class:side={h.length === 1}
					onpointerdown={e => down(e, h)}
					role="slider"
					tabindex={editable ? 0 : -1}
					aria-label={labels[h]}
					aria-valuenow={percent(rect.x)}
					aria-valuetext={where}
					onkeydown={e => onkeydown(e, h)}
				></span>
			{/each}
		</div>
	{/if}
</div>

<style lang="scss">
	.area {
		position: relative;
		width: 100%;
		height: 60vh;
		overflow: hidden;
	}

	canvas {
		position: absolute;
		display: block;
		outline: 1px dashed var(--backdrop);
	}

	.fill {
		position: absolute;
	}

	// the part that stays, the rest darkened around it
	.crop {
		position: absolute;
		box-shadow: 0 0 0 9999px rgba(16, 24, 40, 0.45);
		outline: 2px solid white;
		cursor: move;
		touch-action: none;

		&:focus-visible {
			outline-color: var(--accent);
		}
	}

	// inside the corners, the frame cuts what is outside the image
	.handle {
		position: absolute;
		width: 0.875rem;
		height: 0.875rem;
		background: var(--surface);
		border: 2px solid var(--accent);
		border-radius: var(--radius-sm);
		touch-action: none;

		&:focus-visible {
			outline: 2px solid var(--accent);
			outline-offset: 1px;
		}

		&.nw {
			left: -2px;
			top: -2px;
			cursor: nwse-resize;
		}

		&.ne {
			right: -2px;
			top: -2px;
			cursor: nesw-resize;
		}

		&.sw {
			left: -2px;
			bottom: -2px;
			cursor: nesw-resize;
		}

		&.se {
			right: -2px;
			bottom: -2px;
			cursor: nwse-resize;
		}

		// the middle of a side, a bar along it
		&.side {
			width: 1.5rem;
			height: 0.5rem;
		}

		&.n,
		&.s {
			left: calc(50% - 0.75rem);
			cursor: ns-resize;
		}

		&.n {
			top: -2px;
		}

		&.s {
			bottom: -2px;
		}

		&.e,
		&.w {
			top: calc(50% - 0.75rem);
			width: 0.5rem;
			height: 1.5rem;
			cursor: ew-resize;
		}

		&.e {
			right: -2px;
		}

		&.w {
			left: -2px;
		}
	}
</style>
