<script lang="ts">
	import { untrack } from 'svelte';
	import type { Edits } from '../api';
	import { drawTurned } from './draw';
	import {
		WHOLE,
		dragged,
		fitScale,
		placeOf,
		withCrop,
		type Corner,
		type Rect,
		type Size,
	} from './edits';

	/**
	 * Cropping: the whole stage is the area, the image in its middle; the
	 * crop may reach past the image. The scale only changes between drags,
	 * so the image doesn't move under the pointer.
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

	let drag: {
		kind: 'move' | Corner;
		x: number;
		y: number;
		from: Rect;
	} | null = null;

	function down(e: PointerEvent, kind: 'move' | Corner) {
		if (!editable) return;
		e.preventDefault();
		e.stopPropagation();
		(e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
		drag = { kind, x: e.clientX, y: e.clientY, from: { ...rect } };
	}

	function move(e: PointerEvent) {
		if (!drag) return;
		const dx = (e.clientX - drag.x) / place.w;
		const dy = (e.clientY - drag.y) / place.h;
		const c = dragged(drag.kind, drag.from, dx, dy, place, want, turned);
		edits = withCrop(edits, c);
	}

	function up() {
		if (!drag) return;
		drag = null;
		refit++;
	}

	/** Where a fraction rect is on the stage. */
	const at = (r: Rect) => ({
		left: `${place.left + r.x * place.w}px`,
		top: `${place.top + r.y * place.h}px`,
		width: `${r.w * place.w}px`,
		height: `${r.h * place.h}px`,
	});
</script>

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
			role="presentation"
		>
			{#each ['nw', 'ne', 'sw', 'se'] as const as h (h)}
				<span
					class="handle {h}"
					onpointerdown={e => down(e, h)}
					role="presentation"
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
	}
</style>
