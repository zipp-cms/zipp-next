/** Lists of a field as new arrays: the values of fields are replaced. */

/** The list with the item at `i` moved by `by` places. */
export function moved<T>(list: T[], i: number, by: number): T[] {
	const next = [...list];
	const [item] = next.splice(i, 1);
	next.splice(i + by, 0, item);
	return next;
}

/** The list with `item` at `at`, the end without it. */
export function inserted<T>(list: T[], item: T, at = list.length): T[] {
	const next = [...list];
	next.splice(at, 0, item);
	return next;
}

export function removed<T>(list: T[], i: number): T[] {
	return list.filter((_, j) => j !== i);
}

export function replaced<T>(list: T[], i: number, item: T): T[] {
	return list.map((x, j) => (j === i ? item : x));
}

/** Whether a field with this many items takes no more. */
export function isFull(count: number, max: number | null): boolean {
	return max !== null && count >= max;
}

/** A copy of plain JSON, also of Svelte's state proxies. */
export function clone<T>(v: T): T {
	return JSON.parse(JSON.stringify(v));
}
