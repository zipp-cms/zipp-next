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

export const WHOLE: Rect = { x: 0, y: 0, w: 1, h: 1 };

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
export type Place = {
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

/**
 * The crop after dragging it or one of its corners by dx, dy (in images),
 * kept in the area and in the ratio if there is one.
 */
export function dragged(
	kind: 'move' | Corner,
	from: Rect,
	dx: number,
	dy: number,
	place: Place,
	want: number | null,
	turned: Size,
): Rect {
	const [lx, hx] = place.x;
	const [ly, hy] = place.y;
	if (kind === 'move') {
		return {
			...from,
			x: Math.min(Math.max(lx, from.x + dx), hx - from.w),
			y: Math.min(Math.max(ly, from.y + dy), hy - from.h),
		};
	}
	const min = 0.03;
	const left = kind === 'nw' || kind === 'sw';
	const top = kind === 'nw' || kind === 'ne';
	let x1 = from.x,
		y1 = from.y,
		x2 = from.x + from.w,
		y2 = from.y + from.h;
	if (left) x1 = Math.min(Math.max(lx, x1 + dx), x2 - min);
	else x2 = Math.max(Math.min(hx, x2 + dx), x1 + min);
	if (top) y1 = Math.min(Math.max(ly, y1 + dy), y2 - min);
	else y2 = Math.max(Math.min(hy, y2 + dy), y1 + min);
	// a ratio: the height follows the width, within the area
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

/** The focal point of a click on an element showing the image. */
export function focalAt(e: MouseEvent, el: Element): { x: number; y: number } {
	const r = el.getBoundingClientRect();
	const clamp = (v: number) => Math.min(Math.max(0, v), 1);
	return {
		x: clamp((e.clientX - r.left) / r.width),
		y: clamp((e.clientY - r.top) / r.height),
	};
}
