// The fields of a form on its pages, what FormFields.svelte changes: the
// value is a list of items `{ page, block }`, the items of a page one
// after the other. Each function returns a new list.

/**
 * @typedef {{ page: string, block: { id: string, type: string, fields: Record<string, unknown>, enabled?: false } }} Item
 * @typedef {{ label: string, rows: number[] }} Page
 */

/**
 * The pages in their order with the rows of their items, then the new
 * ones without fields yet (`extra`); at least one.
 * @param {Item[]} items
 * @param {string[]} extra
 * @returns {Page[]}
 */
export function pagesOf(items, extra) {
	/** @type {Page[]} */
	const pages = [];
	items.forEach((item, i) => {
		const last = pages.at(-1);
		if (last && last.label === item.page) last.rows.push(i);
		else pages.push({ label: item.page, rows: [i] });
	});
	for (const label of extra) {
		if (!pages.some(p => p.label === label))
			pages.push({ label, rows: [] });
	}
	if (!pages.length) pages.push({ label: '', rows: [] });
	return pages;
}

/**
 * Puts an item at the end of a page (the end of the list for a page
 * without fields).
 * @param {Item[]} items
 * @param {Item} item
 */
export function addTo(items, item) {
	const last = items.findLastIndex(i => i.page === item.page);
	const at = last < 0 ? items.length : last + 1;
	return [...items.slice(0, at), item, ...items.slice(at)];
}

/**
 * Moves the item of a row to the end of another page.
 * @param {Item[]} items
 * @param {number} row
 * @param {string} page
 */
export function moveTo(items, row, page) {
	const rest = items.filter((_, i) => i !== row);
	return addTo(rest, { ...items[row], page });
}

/**
 * Moves an item within its page: `from` and `to` count its page's rows.
 * @param {Item[]} items
 * @param {number[]} rows the rows of the page
 * @param {number} from
 * @param {number} to
 */
export function reorder(items, rows, from, to) {
	const page = rows.map(r => items[r]);
	const [item] = page.splice(from, 1);
	page.splice(to, 0, item);
	const next = [...items];
	rows.forEach((r, i) => (next[r] = page[i]));
	return next;
}

/**
 * @param {Item[]} items
 * @param {string} from
 * @param {string} to
 */
export function rename(items, from, to) {
	return items.map(i => (i.page === from ? { ...i, page: to } : i));
}

/**
 * @param {Item[]} items
 * @param {string} page
 */
export function removePage(items, page) {
	return items.filter(i => i.page !== page);
}

/**
 * A label none of the pages has: `wanted`, else with a number after it.
 * @param {string[]} labels
 * @param {string} wanted
 */
export function freeLabel(labels, wanted) {
	let label = wanted;
	for (let n = 2; labels.includes(label); n++) label = `${wanted} ${n}`;
	return label;
}
