import type { Entry } from './api';

/**
 * The tree of a structure, from its entries in tree order: how deep an
 * entry is, what is below it. Moving is zipp-ui's sortable.
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
