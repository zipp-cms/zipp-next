import type { Edits } from '../api';

/**
 * The geometry of the image editor, without the DOM: the sizes after
 * each step, the crop and what rotating and flipping do to it. The server
 * applies the edits in the same order: rotate, flip, crop, resize, canvas.
 */

export type Size = { w: number; h: number };
/** fractions of the turned image, past 0 and 1 outside of it */
export type Rect = { x: number; y: number; w: number; h: number };
export type Corner = 'nw' | 'ne' | 'sw' | 'se';
/** A corner, or the middle of a side: only that side moves. */
export type Handle = Corner | 'n' | 'e' | 's' | 'w';

export const WHOLE: Rect = { x: 0, y: 0, w: 1, h: 1 };

export const NO_EDITS: Edits = {
	rotate: 0,
	flipH: false,
	flipV: false,
	crop: null,
	resize: null,
	canvas: null,
	background: null,
};

/** Whether the crop reaches past the image. */
export function cropsOutside(e: Edits): boolean {
	const c = e.crop;
	return !!c && (c.x < 0 || c.y < 0 || c.x + c.w > 1 || c.y + c.h > 1);
}

/** Whether the edits leave the image as uploaded. */
export function noEdits(e: Edits): boolean {
	return (
		!e.rotate && !e.flipH && !e.flipV && !e.crop && !e.resize && !e.canvas
	);
}

/** Whether the edited image has transparent parts around it. */
export function transparentEdges(e: Edits): boolean {
	const transparent = (e.background ?? 'transparent') === 'transparent';
	return transparent && (!!e.canvas || cropsOutside(e));
}

/** The sizes of an image after each step of its edits. */
export function sizes(natural: Size, edits: Edits) {
	const turned: Size =
		edits.rotate % 180 ? { w: natural.h, h: natural.w } : natural;
	const c = edits.crop;
	const cropped: Size = c
		? {
				w: Math.max(1, Math.round(turned.w * c.w)),
				h: Math.max(1, Math.round(turned.h * c.h)),
			}
		: turned;
	const r = edits.resize;
	const resized: Size = r
		? {
				w: r.width,
				h:
					r.height ??
					Math.max(1, Math.round((cropped.h * r.width) / cropped.w)),
			}
		: cropped;
	const final: Size = edits.canvas
		? { w: edits.canvas.width, h: edits.canvas.height }
		: resized;
	return { turned, cropped, resized, final };
}

/** The crop of the edits, none when it is the whole image. */
export function withCrop(edits: Edits, c: Rect): Edits {
	const near = (a: number, b: number) => Math.abs(a - b) < 1e-6;
	const whole = near(c.x, 0) && near(c.y, 0) && near(c.w, 1) && near(c.h, 1);
	return { ...edits, crop: whole ? null : c };
}

/** width / height in pixels of a crop ratio: 0 is the image's own. */
export function wantedRatio(
	ratio: number | null,
	portrait: boolean,
	turned: Size,
): number | null {
	if (ratio === null) return null;
	if (ratio === 0) return turned.w / turned.h;
	return portrait ? 1 / ratio : ratio;
}

/** The biggest centered crop of a ratio. */
export function ratioCrop(want: number, turned: Size): Rect {
	// in fractions: w * W / (h * H) = want
	let w = 1;
	let h = turned.w / turned.h / want;
	if (h > 1) {
		w = 1 / h;
		h = 1;
	}
	return { x: (1 - w) / 2, y: (1 - h) / 2, w, h };
}

/**
 * Pixels on screen per image pixel, so the image and the crop fit the
 * area, centered on the image.
 */
export function fitScale(area: Size, turned: Size, r: Rect): number {
	const needX = 2 * Math.max(0.5, 0.5 - r.x, r.x + r.w - 0.5);
	const needY = 2 * Math.max(0.5, 0.5 - r.y, r.y + r.h - 0.5);
	return Math.min(
		(0.8 * area.w) / turned.w,
		(0.8 * area.h) / turned.h,
		(0.96 * area.w) / (turned.w * needX),
		(0.96 * area.h) / (turned.h * needY),
	);
}

/** Where the image is in the area, and how far a crop may go. */
type Place = {
	left: number;
	top: number;
	w: number;
	h: number;
	x: [number, number];
	y: [number, number];
};

export function placeOf(area: Size, turned: Size, scale: number): Place {
	const w = turned.w * scale;
	const h = turned.h * scale;
	const left = (area.w - w) / 2;
	const top = (area.h - h) / 2;
	return {
		left,
		top,
		w,
		h,
		// at most one image beyond each side, within the area
		x: [Math.max(-1, -left / w), Math.min(2, 1 + left / w)],
		y: [Math.max(-1, -top / h), Math.min(2, 1 + top / h)],
	};
}

/** How a drag changes the crop besides where the pointer is. */
export type DragOptions = {
	/** Alt: the opposite side moves the other way, around the middle */
	center?: boolean;
	/** how close to the image's border an edge snaps to it, in pixels */
	snap?: number;
};

/** `v` on 0 or 1 (the image's border) when it is within `d` of it. */
function snapped(v: number, d: number): number {
	if (Math.abs(v) < d) return 0;
	if (Math.abs(v - 1) < d) return 1;
	return v;
}

/**
 * The crop after dragging it, a corner or a side by dx, dy (in images),
 * kept in the area and in the ratio if there is one. The moved edges
 * snap to the image's border, so a crop can keep the whole width and
 * change only its height.
 */
export function dragged(
	kind: 'move' | Handle,
	from: Rect,
	dx: number,
	dy: number,
	place: Place,
	want: number | null,
	turned: Size,
	opts: DragOptions = {},
): Rect {
	const [lx, hx] = place.x;
	const [ly, hy] = place.y;
	const sx = (opts.snap ?? 0) / place.w;
	const sy = (opts.snap ?? 0) / place.h;
	if (kind === 'move') {
		let x = Math.min(Math.max(lx, from.x + dx), hx - from.w);
		let y = Math.min(Math.max(ly, from.y + dy), hy - from.h);
		// its left or right edge onto the border
		if (snapped(x, sx) !== x) x = snapped(x, sx);
		else x = snapped(x + from.w, sx) - from.w;
		if (snapped(y, sy) !== y) y = snapped(y, sy);
		else y = snapped(y + from.h, sy) - from.h;
		return { ...from, x, y };
	}

	const min = 0.03;
	const left = kind.includes('w');
	const right = kind.includes('e');
	const top = kind.includes('n');
	const bottom = kind.includes('s');
	const center = !!opts.center;
	const cx = from.x + from.w / 2;
	const cy = from.y + from.h / 2;
	let x1 = from.x,
		y1 = from.y,
		x2 = from.x + from.w,
		y2 = from.y + from.h;

	// the dragged sides follow the pointer and snap; with Alt the other
	// side mirrors them around the middle
	if (left) {
		x1 = snapped(x1 + dx, sx);
		if (center) x2 = 2 * cx - x1;
	}
	if (right) {
		x2 = snapped(x2 + dx, sx);
		if (center) x1 = 2 * cx - x2;
	}
	if (top) {
		y1 = snapped(y1 + dy, sy);
		if (center) y2 = 2 * cy - y1;
	}
	if (bottom) {
		y2 = snapped(y2 + dy, sy);
		if (center) y1 = 2 * cy - y2;
	}

	// in the area, never smaller than `min`
	x1 = Math.max(lx, x1);
	x2 = Math.min(hx, x2);
	y1 = Math.max(ly, y1);
	y2 = Math.min(hy, y2);
	if (x2 - x1 < min) {
		if (center) [x1, x2] = [cx - min / 2, cx + min / 2];
		else if (left) x1 = x2 - min;
		else x2 = x1 + min;
	}
	if (y2 - y1 < min) {
		if (center) [y1, y2] = [cy - min / 2, cy + min / 2];
		else if (top) y1 = y2 - min;
		else y2 = y1 + min;
	}

	// a ratio: the height follows the width, within the area (a ratio
	// has no side handles, a corner moves both)
	if (want !== null) {
		let w = x2 - x1;
		let h = (w * turned.w) / (want * turned.h);
		const room = center
			? 2 * Math.min(cy - ly, hy - cy)
			: top
				? y2 - ly
				: hy - y1;
		if (h > room) {
			h = room;
			w = (h * want * turned.h) / turned.w;
			if (center) [x1, x2] = [cx - w / 2, cx + w / 2];
			else if (left) x1 = x2 - w;
			else x2 = x1 + w;
		}
		if (center) [y1, y2] = [cy - h / 2, cy + h / 2];
		else if (top) y1 = y2 - h;
		else y2 = y1 + h;
	}
	return { x: x1, y: y1, w: x2 - x1, h: y2 - y1 };
}

/** Turned by 90° either way; the crop turns with the image, as seen. */
export function rotated(edits: Edits, by: number): Edits {
	// the flip comes after the rotation: with one flip a turn goes the
	// other way, as seen
	const mirrored = edits.flipH !== edits.flipV;
	const c = edits.crop;
	return {
		...edits,
		rotate: (edits.rotate + (mirrored ? -by : by) + 360) % 360,
		crop: !c
			? null
			: by > 0
				? { x: 1 - c.y - c.h, y: c.x, w: c.h, h: c.w }
				: { x: c.y, y: 1 - c.x - c.w, w: c.h, h: c.w },
	};
}

export function flipped(edits: Edits, axis: 'flipH' | 'flipV'): Edits {
	const c = edits.crop;
	return {
		...edits,
		[axis]: !edits[axis],
		crop: !c
			? null
			: axis === 'flipH'
				? { ...c, x: 1 - c.x - c.w }
				: { ...c, y: 1 - c.y - c.h },
	};
}

export function withResize(
	edits: Edits,
	width: number | null,
	height: number | null,
): Edits {
	return {
		...edits,
		resize: width
			? {
					width: Math.round(width),
					height: height ? Math.round(height) : null,
				}
			: null,
	};
}

/** A square canvas as big as the resized image, or none. */
export function withCanvas(edits: Edits, on: boolean, resized: Size): Edits {
	const side = Math.max(resized.w, resized.h);
	return { ...edits, canvas: on ? { width: side, height: side } : null };
}

/**
 * How far an arrow key moves the focal point or the crop, in images: a
 * hundredth, with Shift a tenth. `null` for other keys.
 */
export function arrowStep(e: KeyboardEvent): { dx: number; dy: number } | null {
	const step = e.shiftKey ? 0.1 : 0.01;
	const dirs: Record<string, [number, number]> = {
		ArrowLeft: [-1, 0],
		ArrowRight: [1, 0],
		ArrowUp: [0, -1],
		ArrowDown: [0, 1],
	};
	const d = dirs[e.key];
	return d ? { dx: d[0] * step, dy: d[1] * step } : null;
}

/** The focal point moved by dx, dy, kept on the image. */
export function movedFocal(
	f: { x: number; y: number },
	dx: number,
	dy: number,
): { x: number; y: number } {
	const clamp = (v: number) => Math.min(Math.max(0, v), 1);
	return { x: clamp(f.x + dx), y: clamp(f.y + dy) };
}

/** The focal point of a click on an element showing the image. */
export function focalAt(e: MouseEvent, el: Element): { x: number; y: number } {
	const r = el.getBoundingClientRect();
	const clamp = (v: number) => Math.min(Math.max(0, v), 1);
	return {
		x: clamp((e.clientX - r.left) / r.width),
		y: clamp((e.clientY - r.top) / r.height),
	};
}
