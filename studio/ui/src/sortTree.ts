/**
 * The moves of a sortable on its rows, without the page: the rows are
 * their depths, in order (see sortable.svelte.ts).
 */

/**
 * A move: before the row `index` of the rows without the moved ones
 * (their count: at the end), at `depth`.
 */
export type Drop = { index: number; depth: number };

export type Step = 'up' | 'down' | 'out' | 'in';

/** The rows of `from` with the ones under it: their count. */
export function subtree(depths: number[], from: number): number {
	let n = 1;
	while (from + n < depths.length && depths[from + n] > depths[from]) n++;
	return n;
}

/**
 * The drop of a step with the keyboard or the menu, null if none: not
 * deeper than `maxDepth`, not below the `keepLast` rows at the end.
 */
export function stepDrop(
	depths: number[],
	from: number,
	step: Step,
	maxDepth: number,
	keepLast: number,
): Drop | null {
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
			return sibBefore < 0 ? null : { index: at(sibBefore), depth: d };
		case 'down': {
			// not below the rows that stay last
			const max = depths.length - n - keepLast;
			const index = sibAfter < 0 ? -1 : at(end(sibAfter));
			return index < 0 || index > max ? null : { index, depth: d };
		}
		case 'out':
			return d === 0 || parent < 0
				? null
				: { index: at(end(parent)), depth: d - 1 };
		case 'in':
			return sibBefore < 0 || d + 1 + below > maxDepth
				? null
				: // where it is, the rows before it stay
					{ index: from, depth: d + 1 };
	}
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
