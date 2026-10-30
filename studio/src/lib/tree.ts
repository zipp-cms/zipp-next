import type { Entry } from '../api';

/**
 * The tree of a structure, from its entries in tree order: how deep an
 * entry is, what is below it and where a dragged entry goes.
 */

type Node = Pick<Entry, 'id' | 'parentId'>;

/** How deep an entry is, from the parents in the list. */
export function depth(list: Node[], e: Node): number {
	const byId = new Map(list.map(n => [n.id, n]));
	let d = 0;
	for (let p = e.parentId; p !== null && d < 100; d++) {
		p = byId.get(p)?.parentId ?? null;
	}
	return d;
}

/** Whether `e` is somewhere below `ancestor`. */
export function isBelow(list: Node[], e: Node, ancestor: Node): boolean {
	const byId = new Map(list.map(n => [n.id, n]));
	for (let p = e.parentId; p !== null; p = byId.get(p)?.parentId ?? null) {
		if (p === ancestor.id) return true;
	}
	return false;
}

export type Where = 'before' | 'after' | 'into';

/** Where a drop at `y` (0 top, 1 bottom) of a row puts the entry. */
export function dropWhere(y: number): Where {
	return y < 0.3 ? 'before' : y > 0.7 ? 'after' : 'into';
}

/**
 * The parent and position of an entry dropped on `target`: into it as
 * the last child, or before or after it among its siblings.
 */
export function dropTo<T extends Node>(
	list: T[],
	dragged: T,
	target: T,
	where: Where,
): { parentId: number | null; index: number } {
	if (where === 'into') return { parentId: target.id, index: 1e6 };
	// the position among the new siblings, without the dragged one
	const sibs = list.filter(
		s => s.parentId === target.parentId && s.id !== dragged.id,
	);
	const index = sibs.indexOf(target) + (where === 'after' ? 1 : 0);
	return { parentId: target.parentId, index };
}
