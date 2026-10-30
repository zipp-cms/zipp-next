import type { Edits } from '../../api';
import { WHOLE, type Size, sizes } from './edits';

/** Draws images with their edits on canvases, like the server makes them. */

/** The image rotated and flipped, what the crop is made of. */
export function turnedCanvas(
	img: HTMLImageElement,
	edits: Edits,
	turned: Size,
): HTMLCanvasElement {
	const c = document.createElement('canvas');
	c.width = turned.w;
	c.height = turned.h;
	const ctx = c.getContext('2d')!;
	// the last transform applies first: rotate, then flip the turned
	// image, like the server
	ctx.translate(turned.w / 2, turned.h / 2);
	ctx.scale(edits.flipH ? -1 : 1, edits.flipV ? -1 : 1);
	ctx.rotate((edits.rotate * Math.PI) / 180);
	ctx.drawImage(img, -img.naturalWidth / 2, -img.naturalHeight / 2);
	return c;
}

/** Fills a canvas with the background, unless it is transparent. */
function fill(
	ctx: CanvasRenderingContext2D,
	background: string,
	w: number,
	h: number,
) {
	if (background === 'transparent') return;
	ctx.fillStyle = background;
	ctx.fillRect(0, 0, w, h);
}

/** The image turned, for cropping: the crop area is around it. */
export function drawTurned(
	view: HTMLCanvasElement,
	img: HTMLImageElement,
	edits: Edits,
) {
	const natural = { w: img.naturalWidth, h: img.naturalHeight };
	const base = turnedCanvas(img, edits, sizes(natural, edits).turned);
	view.width = base.width;
	view.height = base.height;
	view.getContext('2d')!.drawImage(base, 0, 0);
}

/** The edited image: crop, resize, canvas, the background around it. */
export function drawEdited(
	view: HTMLCanvasElement,
	img: HTMLImageElement,
	edits: Edits,
	outside: boolean,
) {
	const natural = { w: img.naturalWidth, h: img.naturalHeight };
	const { turned, cropped, resized, final } = sizes(natural, edits);
	const background = edits.background ?? 'transparent';
	const base = turnedCanvas(img, edits, turned);

	// the crop, past the image the background
	const c = edits.crop ?? WHOLE;
	const part = document.createElement('canvas');
	part.width = cropped.w;
	part.height = cropped.h;
	const pctx = part.getContext('2d')!;
	if (outside) fill(pctx, background, part.width, part.height);
	pctx.drawImage(base, -c.x * base.width, -c.y * base.height);

	view.width = final.w;
	view.height = final.h;
	const ctx = view.getContext('2d')!;
	ctx.clearRect(0, 0, final.w, final.h);
	if (edits.canvas) fill(ctx, background, final.w, final.h);
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
}
