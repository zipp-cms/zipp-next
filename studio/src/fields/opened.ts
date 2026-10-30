/**
 * Whether a block was open or collapsed, by its id, in this browser like
 * Craft: it stays when the page loads again and isn't content. Blocks
 * without an id yet (new ones) aren't kept.
 */

const KEY = 'zipp-blocks-open';
/** the newest ones are kept, an old block's state can go */
const MAX = 500;

function read(): Record<string, boolean> {
	try {
		return JSON.parse(localStorage.getItem(KEY) ?? '{}');
	} catch {
		return {};
	}
}

/** Its state, undefined when there is none. */
export function wasOpen(id: string | null | undefined): boolean | undefined {
	return id ? read()[id] : undefined;
}

export function rememberOpen(id: string | null | undefined, open: boolean) {
	if (!id) return;
	const all = read();
	delete all[id];
	all[id] = open;
	const keys = Object.keys(all);
	for (const k of keys.slice(0, Math.max(0, keys.length - MAX)))
		delete all[k];
	localStorage.setItem(KEY, JSON.stringify(all));
}
