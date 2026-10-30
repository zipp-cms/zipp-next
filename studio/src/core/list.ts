/** Lists of a field as new arrays: the values of fields are replaced. */

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
