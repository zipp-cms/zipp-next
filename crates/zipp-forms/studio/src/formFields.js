// The fields of a form on its pages, what FormFields.svelte changes. The
// value is a list of items: `{ block }` of the type `formPage` starts a
// page, `{ width, block }` is a field; the fields before the first page
// item are on a first page without one. Here the items are pages with
// their fields (`groups`), each change makes new ones and flattens them.

export const PAGE = 'formPage';
export const WIDTHS = [25, 33, 50, 67, 75, 100];

/**
 * @typedef {{ id: string, type: string, fields: Record<string, any>, enabled?: false }} Block
 * @typedef {{ width?: number, block: Block }} Item
 * @typedef {{ page: Item | null, fields: Item[] }} Group
 */

/**
 * The pages with their fields; at least one.
 * @param {Item[]} items
 * @returns {Group[]}
 */
export function groupsOf(items) {
	/** @type {Group[]} */
	const groups = [];
	for (const item of items) {
		if (item.block.type === PAGE) groups.push({ page: item, fields: [] });
		else if (groups.length) groups.at(-1).fields.push(item);
		else groups.push({ page: null, fields: [item] });
	}
	if (!groups.length) groups.push({ page: null, fields: [] });
	return groups;
}

/**
 * The items of the pages: a page without its item (the first) gets one
 * when there are others, so it keeps its place.
 * @param {Group[]} groups
 * @param {() => Item} newPage
 * @returns {Item[]}
 */
export function flatten(groups, newPage) {
	return groups.flatMap((g, i) => {
		const page = g.page ?? (i > 0 || groups.length > 1 ? newPage() : null);
		return page ? [page, ...g.fields] : g.fields;
	});
}

/**
 * A text of a field in a site (`{ de: "Name" }`), else in another one.
 * @param {Record<string, string> | null | undefined} texts
 * @param {string} site
 */
export function textIn(texts, site) {
	if (!texts) return '';
	return texts[site] || Object.values(texts).find(Boolean) || '';
}

/**
 * The name of a page in a site.
 * @param {Group} g
 * @param {string} site
 */
export const labelOf = (g, site) => textIn(g.page?.block.fields.label, site);

/**
 * Names a page in a site.
 * @param {Group[]} groups
 * @param {number} i
 * @param {string} label
 * @param {string} site
 * @param {() => Item} newPage
 */
export function renamed(groups, i, label, site, newPage) {
	return groups.map((g, k) => {
		if (k !== i) return g;
		const page = g.page ?? newPage();
		const texts = { ...page.block.fields.label, [site]: label };
		const block = {
			...page.block,
			fields: { ...page.block.fields, label: texts },
		};
		return { ...g, page: { ...page, block } };
	});
}

/**
 * @template T
 * @param {T[]} list
 * @param {number} from
 * @param {number} to
 */
export function moved(list, from, to) {
	const next = [...list];
	const [x] = next.splice(from, 1);
	next.splice(to, 0, x);
	return next;
}

/**
 * Changes the fields of one page.
 * @param {Group[]} groups
 * @param {number} i
 * @param {(fields: Item[]) => Item[]} f
 */
export function withFields(groups, i, f) {
	return groups.map((g, k) => (k === i ? { ...g, fields: f(g.fields) } : g));
}

/**
 * Moves a field to the end of another page.
 * @param {Group[]} groups
 * @param {number} from the page it is on
 * @param {number} field
 * @param {number} to
 */
export function toPage(groups, from, field, to) {
	const item = groups[from].fields[field];
	const out = withFields(groups, from, fs =>
		fs.filter((_, k) => k !== field),
	);
	return withFields(out, to, fs => [...fs, item]);
}

/** Grid columns of a width, of 12. */
export const span = width => Math.round(((width ?? 100) / 100) * 12);
