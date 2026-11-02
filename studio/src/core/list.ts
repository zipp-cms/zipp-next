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

/**
 * Keys for `{#each}` of items without an id: an item keeps its key while
 * it moves, `keep` gives a changed copy the key of the old one.
 */
export function rowKeys<T extends object>() {
	const keys = new WeakMap<T, number>();
	let next = 0;
	const key = (item: T) => {
		if (!keys.has(item)) keys.set(item, next++);
		return keys.get(item)!;
	};
	return { key, keep: (old: T, now: T) => keys.set(now, key(old)) };
}
