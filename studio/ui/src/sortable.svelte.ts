import { tick } from 'svelte';
import { moved, stepDrop, subtree, type Drop, type Step } from './sortTree';

export { placeIn, type Drop, type Step } from './sortTree';

/**
 * Moving the items of a list or a tree: dragging a handle with the pointer
 * (also on touch), its menu and the arrow keys (see DragHandle.svelte).
 *
 * The rows are flat, in order, each with its depth (0 in a list); an item
 * moves with the rows under it. A move is a `Drop` (sortTree.ts), the
 * list turns it into its own data, `placeIn` helps.
 *
 * Dragging up and down picks the place, left and right the depth: a step
 * of `indent` pixels is a level, as deep as the row above allows.
 */

type Line = { top: number; left: number; width: number; height: number };

export type SortableOptions = {
	/** the element around the rows, they have `data-sort` in order */
	container: () => HTMLElement | undefined;
	/** the rows when they can't have `data-sort`, like a rich text's */
	rows?: () => HTMLElement[];
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
	/** rows at the end that stay last, nothing goes below them */
	keepLast?: () => number;
	/** a list that saves first returns the promise, see `step` */
	onmove: (from: number, drop: Drop) => unknown;
};

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
		if (this.o.rows) return this.o.rows();
		const c = this.o.container();
		if (!c) return [];
		// its own rows, not those of a list in a row (a block's fields)
		return [...c.querySelectorAll<HTMLElement>('[data-sort]')].filter(
			el => {
				const outer = el.parentElement?.closest('[data-sort]');
				return !outer || !c.contains(outer);
			},
		);
	}

	/** Whether there is anything to move: more than one row. */
	movable(): boolean {
		return this.o.depths().length > 1;
	}

	/** Whether row `i` is dragged: the row itself or one under it. */
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
			scrollNear(this.o.container(), ev.clientY);
		};
		const end = (ev: PointerEvent) => {
			window.removeEventListener('pointermove', move);
			window.removeEventListener('pointerup', end);
			window.removeEventListener('pointercancel', end);
			window.removeEventListener('keydown', key, true);
			const drop = this.drop;
			this.dragging = null;
			this.line = null;
			this.drop = null;
			if (!moving) return;
			this.dragged = true;
			setTimeout(() => (this.dragged = false));
			if (drop && ev.type === 'pointerup') this.o.onmove(from, drop);
		};
		// Escape drops nothing, and only that: the flyout or popup the
		// list is in stays (they close on Escape too, see layers)
		const key = (ev: KeyboardEvent) => {
			if (ev.key !== 'Escape' || !moving) return;
			ev.preventDefault();
			ev.stopImmediatePropagation();
			this.drop = null;
			end(new PointerEvent('pointercancel'));
		};
		window.addEventListener('pointermove', move);
		window.addEventListener('pointerup', end);
		window.addEventListener('pointercancel', end);
		window.addEventListener('keydown', key, true);
	}

	/** The drop of a pointer over the rows, and its line. */
	private over(e: PointerEvent, from: number, dx: number) {
		const box = this.o.container()?.getBoundingClientRect();
		const all = this.rows();
		const depths = this.o.depths();
		if (!box || all.length !== depths.length) return;
		const n = subtree(depths, from);
		const others = all.filter((_, i) => i < from || i >= from + n);
		if (this.o.grid?.()) this.overGrid(e, box, others);
		else this.overList(e, box, others, depths, from, dx);
	}

	/** Rows side by side: before or after the nearest one. */
	private overGrid(e: PointerEvent, box: DOMRect, others: HTMLElement[]) {
		let best = -1;
		let dist = Infinity;
		others.forEach((el, i) => {
			const r = el.getBoundingClientRect();
			const d = Math.hypot(
				e.clientX - (r.left + r.width / 2),
				e.clientY - (r.top + r.height / 2),
			);
			if (d < dist) [best, dist] = [i, d];
		});
		if (best < 0) return;
		const r = others[best].getBoundingClientRect();
		const after = e.clientX > r.left + r.width / 2;
		const last = others.length - (this.o.keepLast?.() ?? 0);
		this.drop = {
			index: Math.min(best + (after ? 1 : 0), last),
			depth: 0,
		};
		this.line = {
			top: r.top - box.top,
			left: (after ? r.right + 3 : r.left - 5) - box.left,
			width: 2,
			height: r.height,
		};
	}

	/** Rows under each other: the place by y, the depth by `dx`. */
	private overList(
		e: PointerEvent,
		box: DOMRect,
		others: HTMLElement[],
		depths: number[],
		from: number,
		dx: number,
	) {
		const n = subtree(depths, from);
		const odepths = depths.filter((_, i) => i < from || i >= from + n);
		const rect = (el: HTMLElement) => el.getBoundingClientRect();

		// before the first row whose middle is below the pointer
		let k = others.findIndex(el => {
			const r = rect(el);
			return e.clientY < r.top + r.height / 2;
		});
		if (k < 0) k = others.length;
		k = Math.min(k, others.length - (this.o.keepLast?.() ?? 0));

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
		return stepDrop(
			this.o.depths(),
			from,
			step,
			this.o.maxDepth?.() ?? 0,
			this.o.keepLast?.() ?? 0,
		);
	}

	/**
	 * Moves a row one step and gives its handle the focus again: the rows
	 * moved in the page, or are new ones, the focus is gone.
	 */
	async step(from: number, step: Step) {
		const drop = this.stepDrop(from, step);
		if (!drop) return;
		await this.o.onmove(from, drop);
		await tick();
		// a moved row is at the drop's index
		const row = this.rows()[drop.index];
		row?.querySelector<HTMLElement>('[data-drag-handle]')?.focus();
	}
}

/** How near the edge a drag scrolls, and how far each move. */
const SCROLL_EDGE = 48;
const SCROLL_STEP = 12;

/** Scrolls what the list is in (a flyout's body, or the page) near its edges. */
function scrollNear(list: HTMLElement | undefined, y: number) {
	const box = scroller(list);
	const top = box ? box.getBoundingClientRect().top : 0;
	const bottom = box ? box.getBoundingClientRect().bottom : innerHeight;
	const by =
		y < top + SCROLL_EDGE
			? -SCROLL_STEP
			: y > bottom - SCROLL_EDGE
				? SCROLL_STEP
				: 0;
	if (by) (box ?? window).scrollBy(0, by);
}

/** The element that scrolls around `el`, none: the page. */
function scroller(el: HTMLElement | undefined): HTMLElement | null {
	for (let at = el?.parentElement; at; at = at.parentElement) {
		const y = getComputedStyle(at).overflowY;
		if (
			(y === 'auto' || y === 'scroll') &&
			at.scrollHeight > at.clientHeight
		)
			return at;
	}
	return null;
}

/**
 * Moving the items of a flat list: rows with `data-sort` in `container`,
 * a move sets the list with the item at its new place.
 */
export function listSortable<T>(
	container: () => HTMLElement | undefined,
	items: () => T[],
	set: (next: T[]) => void,
	more: Pick<SortableOptions, 'grid'> = {},
): Sortable {
	return new Sortable({
		container,
		depths: () => items().map(() => 0),
		onmove: (from, drop) => set(moved(items(), from, drop)),
		...more,
	});
}
