/**
 * The tree of a structure, from its entries in tree order: how deep each
 * entry is, what is above it. Moving is zipp-ui's sortable.
 */

type Node = { id: number; parentId: number | null };

/** How deep each entry of a list in tree order is, the top is 0. */
export function depths(list: Node[]): number[] {
	// a parent comes before its children, an entry without its parent in
	// the list is at the top
	const known = new Map<number, number>();
	return list.map(e => {
		const up = e.parentId === null ? undefined : known.get(e.parentId);
		const d = up === undefined ? 0 : up + 1;
		known.set(e.id, d);
		return d;
	});
}

/** The parents of an entry in `byId`, the nearest first. */
export function ancestors<N extends Node>(byId: Map<number, N>, e: Node): N[] {
	const out: N[] = [];
	// a loop in broken data ends at 100
	for (let p = e.parentId; p !== null && out.length < 100;) {
		const up = byId.get(p);
		if (!up) break;
		out.push(up);
		p = up.parentId;
	}
	return out;
}

/**
 * The rows of a tree that show: those whose parents are all open. Each
 * row has the ids of all its parents.
 */
export function visibleRows<R extends { parents: number[] }>(
	rows: R[],
	open: (id: number) => boolean,
): R[] {
	return rows.filter(r => r.parents.every(open));
}
