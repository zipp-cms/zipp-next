<script lang="ts">
	import { untrack } from 'svelte';
	import {
		NO_EDITS,
		cropsOutside,
		noEdits,
		transparentEdges,
		type Edits,
	} from '../api';
	import Icon from './Icon.svelte';

	/**
	 * Edits an image without changing the file, like Craft's image editor:
	 * crop with a rectangle, rotate, flip, resize, a canvas around it, the
	 * focal point. It draws the result the way the server makes it, in
	 * the same order: rotate, flip, crop, resize, canvas.
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

	let img = $state<HTMLImageElement | null>(null);
	$effect(() => {
		const i = new Image();
		i.onload = () => (img = i);
		i.src = src;
	});

	// the size after rotating, crop fractions are of it
	const turned = $derived(
		img
			? edits.rotate % 180
				? { w: img.naturalHeight, h: img.naturalWidth }
				: { w: img.naturalWidth, h: img.naturalHeight }
			: { w: 1, h: 1 },
	);
	const cropped = $derived.by(() => {
		const c = edits.crop;
		return c
			? {
					w: Math.max(1, Math.round(turned.w * c.w)),
					h: Math.max(1, Math.round(turned.h * c.h)),
				}
			: turned;
	});
	const resized = $derived.by(() => {
		const r = edits.resize;
		if (!r) return cropped;
		return {
			w: r.width,
			h:
				r.height ??
				Math.max(1, Math.round((cropped.h * r.width) / cropped.w)),
		};
	});
	/** the size of the edited image */
	const final = $derived(
		edits.canvas
			? { w: edits.canvas.width, h: edits.canvas.height }
			: resized,
	);

	/** The image rotated and flipped, what the crop is made of. */
	function turnedCanvas(): HTMLCanvasElement {
		const c = document.createElement('canvas');
		c.width = turned.w;
		c.height = turned.h;
		const ctx = c.getContext('2d')!;
		// the last transform applies first: rotate, then flip the turned
		// image, like the server
		ctx.translate(turned.w / 2, turned.h / 2);
		ctx.scale(edits.flipH ? -1 : 1, edits.flipV ? -1 : 1);
		ctx.rotate((edits.rotate * Math.PI) / 180);
		ctx.drawImage(img!, -img!.naturalWidth / 2, -img!.naturalHeight / 2);
		return c;
	}

	let view: HTMLCanvasElement | undefined = $state();

	const outside = $derived(cropsOutside(edits));
	/** around the image: past the crop and on the canvas */
	const background = $derived(edits.background ?? '#ffffff');

	/** Fills a canvas with the background, unless it is transparent. */
	function fill(ctx: CanvasRenderingContext2D, w: number, h: number) {
		if (background === 'transparent') return;
		ctx.fillStyle = background;
		ctx.fillRect(0, 0, w, h);
	}

	// draws the base for cropping or the whole result
	$effect(() => {
		if (!img || !view) return;
		void [edits.rotate, edits.flipH, edits.flipV, mode];
		const base = turnedCanvas();
		if (mode === 'crop') {
			// with room around, the crop may reach past the image
			const [px, py] = [pad * base.width, pad * base.height];
			view.width = base.width + 2 * px;
			view.height = base.height + 2 * py;
			const ctx = view.getContext('2d')!;
			fill(ctx, view.width, view.height);
			ctx.drawImage(base, px, py);
			ctx.setLineDash([8, 6]);
			ctx.lineWidth = Math.max(1, base.width / 400);
			ctx.strokeStyle = 'rgba(16, 24, 40, 0.5)';
			ctx.strokeRect(px, py, base.width, base.height);
			return;
		}
		// the crop, past the image the background
		const c = edits.crop ?? { x: 0, y: 0, w: 1, h: 1 };
		const part = document.createElement('canvas');
		part.width = cropped.w;
		part.height = cropped.h;
		const pctx = part.getContext('2d')!;
		if (outside) fill(pctx, part.width, part.height);
		pctx.drawImage(base, -c.x * base.width, -c.y * base.height);

		view.width = final.w;
		view.height = final.h;
		const ctx = view.getContext('2d')!;
		ctx.clearRect(0, 0, final.w, final.h);
		if (edits.canvas) fill(ctx, final.w, final.h);
		// the image fits into the canvas, centered
		let { w, h } = resized;
		if (edits.canvas && (w > final.w || h > final.h)) {
			const s = Math.min(final.w / w, final.h / h);
			w = Math.round(w * s);
			h = Math.round(h * s);
		}
		ctx.imageSmoothingQuality = 'high';
		ctx.drawImage(
			part,
			Math.floor((final.w - w) / 2),
			Math.floor((final.h - h) / 2),
			w,
			h,
		);
	});

	// --- crop ---

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
	/** width / height in pixels of the crop, null is free */
	const want = $derived(
		ratio === null
			? null
			: ratio === 0
				? turned.w / turned.h
				: portrait
					? 1 / ratio
					: ratio,
	);

	const rect = $derived(edits.crop ?? { x: 0, y: 0, w: 1, h: 1 });

	/**
	 * The room around the image while cropping, in images: a quarter, or
	 * more when the crop reaches further. It only grows while cropping,
	 * the image doesn't jump under the pointer.
	 */
	let pad = $state(0.25);
	function fitPad(shrink: boolean) {
		const r = rect;
		const past = Math.max(-r.x, -r.y, r.x + r.w - 1, r.y + r.h - 1);
		const need = Math.min(1, Math.max(0.25, past + 0.1));
		pad = shrink ? need : Math.max(pad, need);
	}
	$effect(() => {
		if (mode === 'crop') untrack(() => fitPad(true));
	});
	/** A place in the crop view, 0 to 1 of the image, in percent. */
	const at = (v: number) => ((pad + v) / (1 + 2 * pad)) * 100;
	const span = (v: number) => (v / (1 + 2 * pad)) * 100;

	function setCrop(c: { x: number; y: number; w: number; h: number }) {
		const near = (a: number, b: number) => Math.abs(a - b) < 1e-6;
		const full =
			near(c.x, 0) && near(c.y, 0) && near(c.w, 1) && near(c.h, 1);
		edits = { ...edits, crop: full ? null : c };
	}

	/** The biggest centered crop of the ratio. */
	function applyRatio() {
		if (want === null) return;
		// in fractions: w * W / (h * H) = want
		let w = 1;
		let h = turned.w / turned.h / want;
		if (h > 1) {
			w = 1 / h;
			h = 1;
		}
		setCrop({ x: (1 - w) / 2, y: (1 - h) / 2, w, h });
	}

	let drag: {
		kind: 'move' | 'nw' | 'ne' | 'sw' | 'se';
		start: { x: number; y: number };
		from: typeof rect;
		box: DOMRect;
	} | null = null;

	function down(e: PointerEvent, kind: NonNullable<typeof drag>['kind']) {
		if (!editable || !view) return;
		e.preventDefault();
		e.stopPropagation();
		(e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
		drag = {
			kind,
			start: { x: e.clientX, y: e.clientY },
			from: { ...rect },
			box: view.getBoundingClientRect(),
		};
	}

	function move(e: PointerEvent) {
		if (!drag) return;
		// in images, the view has the room around it
		const dx =
			((e.clientX - drag.start.x) / drag.box.width) * (1 + 2 * pad);
		const dy =
			((e.clientY - drag.start.y) / drag.box.height) * (1 + 2 * pad);
		const f = drag.from;
		const min = 0.03;
		// as far as the view goes, past the image too
		const lo = -pad;
		const hi = 1 + pad;
		if (drag.kind === 'move') {
			setCrop({
				...f,
				x: Math.min(Math.max(lo, f.x + dx), hi - f.w),
				y: Math.min(Math.max(lo, f.y + dy), hi - f.h),
			});
			return;
		}
		const left = drag.kind === 'nw' || drag.kind === 'sw';
		const top = drag.kind === 'nw' || drag.kind === 'ne';
		let x1 = f.x,
			y1 = f.y,
			x2 = f.x + f.w,
			y2 = f.y + f.h;
		if (left) x1 = Math.min(Math.max(lo, x1 + dx), x2 - min);
		else x2 = Math.max(Math.min(hi, x2 + dx), x1 + min);
		if (top) y1 = Math.min(Math.max(lo, y1 + dy), y2 - min);
		else y2 = Math.max(Math.min(hi, y2 + dy), y1 + min);
		// a ratio: the height follows the width, within the view
		if (want !== null) {
			let w = x2 - x1;
			let h = (w * turned.w) / (want * turned.h);
			const room = top ? y2 - lo : hi - y1;
			if (h > room) {
				h = room;
				w = (h * want * turned.h) / turned.w;
				if (left) x1 = x2 - w;
				else x2 = x1 + w;
			}
			if (top) y1 = y2 - h;
			else y2 = y1 + h;
		}
		setCrop({ x: x1, y: y1, w: x2 - x1, h: y2 - y1 });
	}

	function up() {
		drag = null;
		fitPad(false);
	}

	// --- rotate and flip ---

	function rotate(by: number) {
		// the flip comes after the rotation: with one flip a turn goes
		// the other way, as seen
		const mirrored = edits.flipH !== edits.flipV;
		const c = edits.crop;
		// a 16:9 crop turned is a 9:16 one
		if (ratio) portrait = !portrait;
		edits = {
			...edits,
			rotate: (edits.rotate + (mirrored ? -by : by) + 360) % 360,
			// the crop turns with the image, as seen
			crop: !c
				? null
				: by > 0
					? { x: 1 - c.y - c.h, y: c.x, w: c.h, h: c.w }
					: { x: c.y, y: 1 - c.x - c.w, w: c.h, h: c.w },
		};
	}

	function flip(axis: 'flipH' | 'flipV') {
		const c = edits.crop;
		edits = {
			...edits,
			[axis]: !edits[axis],
			crop: !c
				? null
				: axis === 'flipH'
					? { ...c, x: 1 - c.x - c.w }
					: { ...c, y: 1 - c.y - c.h },
		};
	}

	// --- size ---

	function setResize(width: number | null, height: number | null) {
		edits = {
			...edits,
			resize: width
				? {
						width: Math.round(width),
						height: height ? Math.round(height) : null,
					}
				: null,
		};
	}

	function setCanvas(on: boolean) {
		const side = Math.max(resized.w, resized.h);
		edits = {
			...edits,
			canvas: on ? { width: side, height: side } : null,
		};
	}

	// --- focal point ---

	function setFocal(e: MouseEvent) {
		if (!editable || mode !== 'focal' || !view) return;
		const r = view.getBoundingClientRect();
		focal = {
			x: Math.min(Math.max(0, (e.clientX - r.left) / r.width), 1),
			y: Math.min(Math.max(0, (e.clientY - r.top) / r.height), 1),
		};
	}

	const empty = $derived(noEdits(edits));

	function reset() {
		edits = { ...NO_EDITS };
	}
</script>

{#snippet backgroundPicker()}
	<label class="check">
		<input
			type="checkbox"
			checked={background === 'transparent'}
			onchange={e =>
				(edits.background = e.currentTarget.checked
					? 'transparent'
					: null)}
		/>
		Transparent
	</label>
	{#if background !== 'transparent'}
		<label class="color">
			Color
			<input
				type="color"
				value={background}
				oninput={e => (edits.background = e.currentTarget.value)}
			/>
		</label>
	{/if}
{/snippet}

<div class="editor">
	{#if editable}
		<div class="tools" role="toolbar" aria-label="Edit the image">
			<button
				type="button"
				class:on={mode === 'focal'}
				onclick={() => (mode = 'focal')}
			>
				<Icon name="focal" size={15} /> Focal point
			</button>
			<button
				type="button"
				class:on={mode === 'crop'}
				onclick={() => (mode = 'crop')}
			>
				<Icon name="crop" size={15} /> Crop
			</button>
			<button
				type="button"
				class:on={mode === 'size'}
				onclick={() => (mode = 'size')}
			>
				<Icon name="size" size={15} /> Size
			</button>
			<span class="sep"></span>
			<button
				type="button"
				class="icon"
				title="Rotate left"
				onclick={() => rotate(-90)}
			>
				<Icon name="rotate-left" size={16} />
			</button>
			<button
				type="button"
				class="icon"
				title="Rotate right"
				onclick={() => rotate(90)}
			>
				<Icon name="rotate-right" size={16} />
			</button>
			<button
				type="button"
				class="icon"
				class:active={edits.flipH}
				title="Flip horizontally"
				onclick={() => flip('flipH')}
			>
				<Icon name="flip-h" size={16} />
			</button>
			<button
				type="button"
				class="icon"
				class:active={edits.flipV}
				title="Flip vertically"
				onclick={() => flip('flipV')}
			>
				<Icon name="flip-v" size={16} />
			</button>
			<span class="size">{final.w} × {final.h}</span>
			<button
				type="button"
				class="link"
				disabled={empty}
				onclick={reset}
				title="Back to the uploaded image"
			>
				<Icon name="reset" size={14} /> Reset
			</button>
		</div>
	{/if}

	{#if mode === 'crop'}
		<div class="options">
			{#each RATIOS as [name, r] (name)}
				<button
					type="button"
					class:on={ratio === r}
					onclick={() => {
						ratio = r;
						applyRatio();
					}}
				>
					{name}
				</button>
			{/each}
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
				{@render backgroundPicker()}
			{/if}
		</div>
		<p class="note">Drag the crop past the image for room around it.</p>
	{:else if mode === 'size'}
		<div class="options sizes">
			<label>
				Width
				<input
					type="number"
					min="1"
					max="10000"
					value={resized.w}
					onchange={e =>
						setResize(
							+e.currentTarget.value,
							edits.resize?.height ?? null,
						)}
				/>
			</label>
			<label>
				Height
				<input
					type="number"
					min="1"
					max="10000"
					value={resized.h}
					disabled={!edits.resize}
					onchange={e =>
						setResize(
							edits.resize?.width ?? resized.w,
							+e.currentTarget.value,
						)}
				/>
			</label>
			{#if edits.resize}
				<button
					type="button"
					class="link"
					onclick={() => setResize(null, null)}
				>
					Original size
				</button>
			{/if}
			<label class="check">
				<input
					type="checkbox"
					checked={!!edits.canvas}
					onchange={e => setCanvas(e.currentTarget.checked)}
				/>
				Canvas around it
			</label>
			{#if edits.canvas}
				<label>
					Canvas width
					<input
						type="number"
						min="1"
						max="10000"
						bind:value={edits.canvas.width}
					/>
				</label>
				<label>
					Canvas height
					<input
						type="number"
						min="1"
						max="10000"
						bind:value={edits.canvas.height}
					/>
				</label>
				{@render backgroundPicker()}
			{/if}
		</div>
	{/if}
	{#if mode !== 'focal' && transparentEdges(edits)}
		<p class="note">
			Transparent needs a PNG: a JPEG is delivered as PNG then.
		</p>
	{/if}

	<div class="stage">
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
			{:else if mode === 'crop'}
				<div
					class="crop"
					style:left="{at(rect.x)}%"
					style:top="{at(rect.y)}%"
					style:width="{span(rect.w)}%"
					style:height="{span(rect.h)}%"
					onpointerdown={e => down(e, 'move')}
					onpointermove={move}
					onpointerup={up}
					role="presentation"
				>
					{#each ['nw', 'ne', 'sw', 'se'] as const as h (h)}
						<span
							class="handle {h}"
							onpointerdown={e => down(e, h)}
							onpointermove={move}
							onpointerup={up}
							role="presentation"
						></span>
					{/each}
				</div>
			{/if}
		</div>
		{#if !img}<p class="note">Loading…</p>{/if}
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
		gap: 10px;
	}

	.tools,
	.options {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 4px;
	}

	.tools {
		padding: 4px;
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: 8px;

		> button:not(.icon) {
			border-color: transparent;
		}
	}

	button.on {
		color: var(--accent);
		background: var(--accent-soft);
		border-color: var(--accent-soft);
	}

	button.active {
		color: var(--accent);
	}

	.sep {
		width: 1px;
		height: 18px;
		margin: 0 4px;
		background: var(--border);
	}

	.size {
		margin-left: auto;
		color: var(--muted);
		font-family: var(--mono);
		font-size: 11px;
	}

	.sizes {
		gap: 10px;
		align-items: flex-end;

		label {
			display: flex;
			flex-direction: column;
			gap: 3px;
			font-size: 12px;
			font-weight: 600;
		}

		input[type='number'] {
			width: 7rem;
		}
	}

	.check {
		display: flex;
		flex-direction: row !important;
		align-items: center;
		gap: 6px !important;
		font-size: 13px;
		font-weight: 500;
	}

	.color input {
		width: 3rem;
		height: 32px;
		padding: 2px;
	}

	.note {
		margin: 0;
		color: var(--muted);
		font-size: 12px;
	}

	.stage {
		display: grid;
		place-items: center;
		padding: 16px;
		border-radius: 8px;
		// a checkerboard: transparent parts show
		background-color: #fff;
		background-image:
			linear-gradient(45deg, #e9ecf1 25%, transparent 25%),
			linear-gradient(-45deg, #e9ecf1 25%, transparent 25%),
			linear-gradient(45deg, transparent 75%, #e9ecf1 75%),
			linear-gradient(-45deg, transparent 75%, #e9ecf1 75%);
		background-size: 16px 16px;
		background-position:
			0 0,
			0 8px,
			8px -8px,
			-8px 0;
		border: 1px solid var(--border);
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
		box-shadow: 0 0 0 1px rgba(16, 24, 40, 0.12);
	}

	.point {
		position: absolute;
		width: 1.4rem;
		height: 1.4rem;
		margin: -0.7rem 0 0 -0.7rem;
		border: 3px solid white;
		border-radius: 50%;
		box-shadow: 0 0 0 2px rgba(0, 0, 0, 0.5);
		pointer-events: none;
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
		width: 14px;
		height: 14px;
		background: white;
		border: 2px solid var(--accent);
		border-radius: 3px;
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

	.frame:has(.crop) {
		overflow: hidden;
	}
</style>
