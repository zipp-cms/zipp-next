/**
 * Moving the items of a list or a tree: dragging a handle with the pointer
 * (also on touch), its menu and the arrow keys (see DragHandle.svelte).
 *
 * The rows are flat, in order, each with its depth (0 in a list); an item
 * moves with the rows under it. A move is a `Drop`: before the row
 * `index` of the rows without the moved ones (their count: at the end),
 * at `depth`. The list turns it into its own data, `moved` and `placeIn`
 * help.
 *
 * Dragging up and down picks the place, left and right the depth: a step
 * of `indent` pixels is a level, as deep as the row above allows.
 */

export type Drop = { index: number; depth: number };

export type Step = 'up' | 'down' | 'out' | 'in';

type Line = { top: number; left: number; width: number; height: number };

export type SortableOptions = {
	/** the element around the rows, they have `data-sort` in order */
	container: () => HTMLElement | undefined;
	/** the depth of each row */
	depths: () => number[];
	/** how deep a row may go, 0 for a list */
	maxDepth?: () => number;
	/** rows side by side that wrap, like images */
	grid?: () => boolean;
	/** pixels of one level */
	indent?: number;
	/** pixels from the container's left to the rows' start */
	inset?: number;
	onmove: (from: number, drop: Drop) => void;
};

/** The rows of `from` with the ones under it: their count. */
function subtree(depths: number[], from: number): number {
	let n = 1;
	while (from + n < depths.length && depths[from + n] > depths[from]) n++;
	return n;
}

export class Sortable {
	/** the row that is dragged */
	dragging = $state<number | null>(null);
	/** where it would go, relative to the container */
	line = $state<Line | null>(null);
	private drop: Drop | null = null;
	/** a drag just ended: the click that follows isn't one */
	dragged = false;

	constructor(private o: SortableOptions) {}

	private rows(): HTMLElement[] {
		const c = this.o.container();
		return c ? [...c.querySelectorAll<HTMLElement>('[data-sort]')] : [];
	}

	/** Whether row `i` is dragged, itself or with the row above it. */
	moving(i: number): boolean {
		const from = this.dragging;
		if (from === null) return false;
		return i >= from && i < from + subtree(this.o.depths(), from);
	}

	/** Starts dragging row `from` when the pointer moves a little. */
	start(e: PointerEvent, from: number) {
		if (e.button !== 0) return;
		const x0 = e.clientX;
		const y0 = e.clientY;
		let moving = false;

		const move = (ev: PointerEvent) => {
			if (!moving) {
				if (Math.hypot(ev.clientX - x0, ev.clientY - y0) < 5) return;
				moving = true;
				this.dragging = from;
			}
			ev.preventDefault();
			this.over(ev, from, ev.clientX - x0);
			scrollNear(ev.clientY);
		};
		const end = (ev: PointerEvent) => {
			window.removeEventListener('pointermove', move);
			window.removeEventListener('pointerup', end);
			window.removeEventListener('pointercancel', end);
			window.removeEventListener('keydown', key);
			const drop = this.drop;
			this.dragging = null;
			this.line = null;
			this.drop = null;
			if (!moving) return;
			this.dragged = true;
			setTimeout(() => (this.dragged = false));
			if (drop && ev.type === 'pointerup') this.o.onmove(from, drop);
		};
		// Escape drops nothing
		const key = (ev: KeyboardEvent) => {
			if (ev.key !== 'Escape') return;
			this.drop = null;
			end(new PointerEvent('pointercancel'));
		};
		window.addEventListener('pointermove', move);
		window.addEventListener('pointerup', end);
		window.addEventListener('pointercancel', end);
		window.addEventListener('keydown', key);
	}

	/** The drop of a pointer over the rows, and its line. */
	private over(e: PointerEvent, from: number, dx: number) {
		const box = this.o.container()?.getBoundingClientRect();
		const all = this.rows();
		const depths = this.o.depths();
		if (!box || all.length !== depths.length) return;
		const n = subtree(depths, from);
		const others = all.filter((_, i) => i < from || i >= from + n);
		const odepths = depths.filter((_, i) => i < from || i >= from + n);
		const rect = (el: HTMLElement) => el.getBoundingClientRect();

		if (this.o.grid?.()) {
			// the nearest one, before or after it
			let best = -1;
			let dist = Infinity;
			others.forEach((el, i) => {
				const r = rect(el);
				const d = Math.hypot(
					e.clientX - (r.left + r.width / 2),
					e.clientY - (r.top + r.height / 2),
				);
				if (d < dist) [best, dist] = [i, d];
			});
			if (best < 0) return;
			const r = rect(others[best]);
			const after = e.clientX > r.left + r.width / 2;
			this.drop = { index: best + (after ? 1 : 0), depth: 0 };
			this.line = {
				top: r.top - box.top,
				left: (after ? r.right + 3 : r.left - 5) - box.left,
				width: 2,
				height: r.height,
			};
			return;
		}

		// before the first row whose middle is below the pointer
		let k = others.findIndex(el => {
			const r = rect(el);
			return e.clientY < r.top + r.height / 2;
		});
		if (k < 0) k = others.length;

		const own = depths.slice(from, from + n);
		const below = Math.max(...own) - depths[from];
		const indent = this.o.indent ?? 24;
		const want = depths[from] + Math.round(dx / indent);
		const max = Math.min(
			k > 0 ? odepths[k - 1] + 1 : 0,
			(this.o.maxDepth?.() ?? 0) - below,
		);
		const min = k < others.length ? odepths[k] : 0;
		const depth = Math.max(min, Math.min(want, max));
		this.drop = { index: k, depth };

		const edge =
			k < others.length
				? rect(others[k]).top - 3
				: others.length
					? rect(others[others.length - 1]).bottom + 1
					: box.top;
		const left = (this.o.inset ?? 0) + depth * indent;
		this.line = {
			top: edge - box.top,
			left,
			width: box.width - left - (this.o.inset ?? 0),
			height: 2,
		};
	}

	/** The drop of a step with the keyboard or the menu, null if none. */
	stepDrop(from: number, step: Step): Drop | null {
		const depths = this.o.depths();
		const d = depths[from];
		const n = subtree(depths, from);
		// positions in the rows without the moved ones
		const at = (i: number) => (i < from ? i : i - n);
		const end = (i: number) => i + subtree(depths, i);
		let prev = from - 1;
		while (prev >= 0 && depths[prev] > d) prev--;
		const sibBefore = prev >= 0 && depths[prev] === d ? prev : -1;
		const next = from + n;
		const sibAfter = next < depths.length && depths[next] === d ? next : -1;
		let parent = from - 1;
		while (parent >= 0 && depths[parent] >= d) parent--;
		const below = Math.max(...depths.slice(from, from + n)) - d;

		switch (step) {
			case 'up':
				return sibBefore < 0
					? null
					: { index: at(sibBefore), depth: d };
			case 'down':
				return sibAfter < 0
					? null
					: { index: at(end(sibAfter)), depth: d };
			case 'out':
				return d === 0 || parent < 0
					? null
					: { index: at(end(parent)), depth: d - 1 };
			case 'in':
				return sibBefore < 0 || d + 1 + below > (this.o.maxDepth?.() ?? 0)
					? null
					: // where it is, the rows before it stay
						{ index: from, depth: d + 1 };
		}
	}

	step(from: number, step: Step) {
		const drop = this.stepDrop(from, step);
		if (drop) this.o.onmove(from, drop);
	}
}

/** Scrolls the page when the pointer is near its top or bottom. */
function scrollNear(y: number) {
	const edge = 48;
	if (y < edge) window.scrollBy(0, -12);
	else if (y > window.innerHeight - edge) window.scrollBy(0, 12);
}

/** A list with the item `from` dropped at `drop.index`. */
export function moved<T>(list: T[], from: number, drop: Drop): T[] {
	const next = [...list];
	const [item] = next.splice(from, 1);
	next.splice(drop.index, 0, item);
	return next;
}

/**
 * Where a drop puts an item in a tree: its new parent (its row in
 * `depths`, null on the top) and its place among the parent's children,
 * not counting itself.
 */
export function placeIn(
	depths: number[],
	from: number,
	drop: Drop,
): { parent: number | null; index: number } {
	const n = subtree(depths, from);
	const rows = depths
		.map((depth, i) => ({ depth, i }))
		.filter(r => r.i < from || r.i >= from + n);
	let parent: number | null = null;
	for (let k = drop.index - 1; k >= 0; k--) {
		if (rows[k].depth === drop.depth - 1) {
			parent = rows[k].i;
			break;
		}
		if (rows[k].depth < drop.depth) break;
	}
	let index = 0;
	for (let k = drop.index - 1; k >= 0; k--) {
		if (rows[k].depth < drop.depth) break;
		if (rows[k].depth === drop.depth) index++;
	}
	return { parent, index };
}
