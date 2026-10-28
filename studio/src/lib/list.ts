/** The list with the item at `i` moved by `by` places, a new array. */
export function moved<T>(list: T[], i: number, by: number): T[] {
	const next = [...list];
	const [item] = next.splice(i, 1);
	next.splice(i + by, 0, item);
	return next;
}
