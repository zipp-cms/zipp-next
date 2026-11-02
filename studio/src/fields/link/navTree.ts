import { placeIn, type Drop } from 'zipp-ui/sortable';
import type { Link, NavItem } from '../../core/schema';

/**
 * The links of a navigation as a tree: its rows for the list, and the
 * changes, each on a copy of the items that it changes in place.
 */

export type Row = { item: NavItem; depth: number; path: number[] };

/** The tree as rows, a link before the ones under it. */
export function flatten(list: NavItem[], d = 0, path: number[] = []): Row[] {
	return list.flatMap((item, i) => [
		{ item, depth: d, path: [...path, i] },
		...flatten(item.children ?? [], d + 1, [...path, i]),
	]);
}

/** The link of an item, without its children. */
export function linkOf(item: NavItem): Link {
	const { children: _, ...link } = item;
	return link;
}

/** The list a row is in, of its parent or the top. */
function siblings(list: NavItem[], path: number[]): NavItem[] {
	let level = list;
	for (const i of path.slice(0, -1)) level = level[i].children!;
	return level;
}

/** Takes `item` out of its list, an empty list of children goes. */
function take(list: NavItem[], row: Row) {
	const sibs = siblings(list, row.path);
	sibs.splice(row.path.at(-1)!, 1);
	if (!sibs.length && row.path.length > 1) {
		delete siblings(list, row.path.slice(0, -1))[row.path.at(-2)!].children;
	}
}

/** Moves the row `from` with the ones under it to `drop`. */
export function moveRow(list: NavItem[], from: number, drop: Drop) {
	const rows = flatten(list);
	const to = placeIn(
		rows.map(r => r.depth),
		from,
		drop,
	);
	const item = rows[from].item;
	const parent = to.parent === null ? null : rows[to.parent].item;
	take(list, rows[from]);
	const into = parent ? (parent.children ??= []) : list;
	into.splice(to.index, 0, item);
}

/** Another link for row `i`, the ones under it stay. */
export function setRowLink(list: NavItem[], i: number, link: Link) {
	const row = flatten(list)[i];
	const children = row.item.children;
	siblings(list, row.path)[row.path.at(-1)!] = {
		...link,
		...(children ? { children } : {}),
	};
}

/** Removes row `i` with the ones under it. */
export function removeRow(list: NavItem[], i: number) {
	take(list, flatten(list)[i]);
}
