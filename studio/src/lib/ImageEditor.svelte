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
	const background = $derived(edits.background ?? 'transparent');
	// the color the color button sets, kept while transparent
	// svelte-ignore state_referenced_locally
	let color = $state(
		edits.background && edits.background !== 'transparent'
			? edits.background
			: '#ffffff',
	);

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
			// only the image, the crop area around it is the stage
			view.width = base.width;
			view.height = base.height;
			view.getContext('2d')!.drawImage(base, 0, 0);
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
	 * While cropping the whole stage is the area: the image in the middle,
	 * the crop may reach past it into the checkerboard, at most one image
	 * beyond each side. `scale` is pixels on screen per image pixel; it
	 * only changes between drags, the image doesn't move under the
	 * pointer.
	 */
	let area = $state({ w: 0, h: 0 });
	let scale = $state(0);
	let refit = $state(0);
	$effect(() => {
		void [mode, area.w, area.h, turned.w, turned.h, refit];
		untrack(() => {
			const { w: W, h: H } = turned;
			const r = rect;
			// the image and the crop fit, centered on the image
			const needX = 2 * Math.max(0.5, 0.5 - r.x, r.x + r.w - 0.5);
			const needY = 2 * Math.max(0.5, 0.5 - r.y, r.y + r.h - 0.5);
			scale = Math.min(
				(0.8 * area.w) / W,
				(0.8 * area.h) / H,
				(0.96 * area.w) / (W * needX),
				(0.96 * area.h) / (H * needY),
			);
		});
	});
	/** where the image is in the area, and how far the crop may go */
	const place = $derived.by(() => {
		const w = turned.w * scale;
		const h = turned.h * scale;
		const left = (area.w - w) / 2;
		const top = (area.h - h) / 2;
		return {
			left,
			top,
			w,
			h,
			x: [Math.max(-1, -left / w), Math.min(2, 1 + left / w)],
			y: [Math.max(-1, -top / h), Math.min(2, 1 + top / h)],
		};
	});

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
		};
	}

	function move(e: PointerEvent) {
		if (!drag) return;
		// in images
		const dx = (e.clientX - drag.start.x) / place.w;
		const dy = (e.clientY - drag.start.y) / place.h;
		const f = drag.from;
		const min = 0.03;
		// as far as the area goes, past the image too
		const [lx, hx] = place.x;
		const [ly, hy] = place.y;
		if (drag.kind === 'move') {
			setCrop({
				...f,
				x: Math.min(Math.max(lx, f.x + dx), hx - f.w),
				y: Math.min(Math.max(ly, f.y + dy), hy - f.h),
			});
			return;
		}
		const left = drag.kind === 'nw' || drag.kind === 'sw';
		const top = drag.kind === 'nw' || drag.kind === 'ne';
		let x1 = f.x,
			y1 = f.y,
			x2 = f.x + f.w,
			y2 = f.y + f.h;
		if (left) x1 = Math.min(Math.max(lx, x1 + dx), x2 - min);
		else x2 = Math.max(Math.min(hx, x2 + dx), x1 + min);
		if (top) y1 = Math.min(Math.max(ly, y1 + dy), y2 - min);
		else y2 = Math.max(Math.min(hy, y2 + dy), y1 + min);
		// a ratio: the height follows the width, within the view
		if (want !== null) {
			let w = x2 - x1;
			let h = (w * turned.w) / (want * turned.h);
			const room = top ? y2 - ly : hy - y1;
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
		refit++;
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
	<div class="background" role="group" aria-label="Background">
		<span class="label">Background</span>
		<button
			type="button"
			class:on={background === 'transparent'}
			onclick={() => (edits.background = null)}
		>
			<span class="swatch checker"></span>
			Transparent
		</button>
		<!-- the button opens the color picker -->
		<label class="button" class:on={background !== 'transparent'}>
			<span class="swatch" style:background={color}></span>
			{color}
			<input
				type="color"
				value={color}
				onclick={() => (edits.background = color)}
				oninput={e => {
					color = e.currentTarget.value;
					edits.background = color;
				}}
			/>
		</label>
	</div>
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

	<div class="stage" class:cropping={mode === 'crop'}>
		{#if mode === 'crop'}
			<div
				class="area"
				bind:clientWidth={area.w}
				bind:clientHeight={area.h}
			>
				{#if img && scale > 0}
					{#if background !== 'transparent'}
						<div
							class="fill"
							style:background
							style:left="{place.left + rect.x * place.w}px"
							style:top="{place.top + rect.y * place.h}px"
							style:width="{rect.w * place.w}px"
							style:height="{rect.h * place.h}px"
						></div>
					{/if}
					<canvas
						bind:this={view}
						class="placed"
						style:left="{place.left}px"
						style:top="{place.top}px"
						style:width="{place.w}px"
						style:height="{place.h}px"
					></canvas>
					<div
						class="crop"
						style:left="{place.left + rect.x * place.w}px"
						style:top="{place.top + rect.y * place.h}px"
						style:width="{rect.w * place.w}px"
						style:height="{rect.h * place.h}px"
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
		border-radius: var(--radius-lg);

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
		font-size: var(--fs-xs);
	}

	.sizes {
		gap: 10px;
		align-items: flex-end;

		label {
			display: flex;
			flex-direction: column;
			gap: 3px;
			font-size: var(--fs-sm);
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
		font-size: var(--fs-md);
		font-weight: 500;
	}

	.background {
		display: flex;
		align-items: center;
		gap: 4px;

		.label {
			margin-right: 2px;
			color: var(--muted);
			font-size: var(--fs-sm);
		}

		// a label that looks like the buttons, it opens the color picker
		.button {
			position: relative;
			display: inline-flex;
			align-items: center;
			gap: 6px;
			padding: 6px 12px;
			font-size: var(--fs-md);
			font-weight: 500;
			font-family: var(--mono);
			border: 1px solid var(--border-strong);
			border-radius: var(--radius);
			background: var(--surface);
			cursor: pointer;

			&.on {
				color: var(--accent);
				background: var(--accent-soft);
				border-color: var(--accent-soft);
			}

			input {
				position: absolute;
				inset: 0;
				width: 100%;
				height: 100%;
				opacity: 0;
				cursor: pointer;
			}
		}
	}

	.swatch {
		width: 14px;
		height: 14px;
		border-radius: var(--radius-sm);
		box-shadow: inset 0 0 0 1px rgba(16, 24, 40, 0.2);

		&.checker {
			background: repeating-conic-gradient(#d0d5dd 0 25%, #fff 0 50%) 0
				0 / 7px 7px;
		}
	}

	.note {
		margin: 0;
		color: var(--muted);
		font-size: var(--fs-sm);
	}

	.stage {
		display: grid;
		place-items: center;
		padding: 16px;
		border-radius: var(--radius-lg);
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

	// the whole stage is the crop area
	.stage.cropping {
		display: block;
		padding: 0;
		overflow: hidden;
	}

	.area {
		position: relative;
		width: 100%;
		height: 60vh;
		overflow: hidden;
	}

	.placed {
		position: absolute;
		max-width: none;
		max-height: none;
		box-shadow: none;
		outline: 1px dashed rgba(16, 24, 40, 0.45);
	}

	.fill {
		position: absolute;
	}
</style>
