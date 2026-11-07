/**
 * Whether a block was open or collapsed and the tab it showed, by its id,
 * in this browser like Craft: it stays when the page loads again and
 * isn't content. A new block gets its id in the studio, so its state
 * also stays when saving loads the content again.
 */

const OPEN = 'zipp-blocks-open';
const TAB = 'zipp-blocks-tab';
/** the newest ones are kept, an old block's state can go */
const MAX = 500;

function read<T>(key: string): Record<string, T> {
	try {
		return JSON.parse(localStorage.getItem(key) ?? '{}');
	} catch {
		return {};
	}
}

function remember<T>(key: string, id: string | null | undefined, value: T) {
	if (!id) return;
	const all = read<T>(key);
	delete all[id];
	all[id] = value;
	const keys = Object.keys(all);
	for (const k of keys.slice(0, Math.max(0, keys.length - MAX)))
		delete all[k];
	localStorage.setItem(key, JSON.stringify(all));
}

/** Its state, undefined when there is none. */
export function wasOpen(id: string | null | undefined): boolean | undefined {
	return id ? read<boolean>(OPEN)[id] : undefined;
}

export function rememberOpen(id: string | null | undefined, open: boolean) {
	remember(OPEN, id, open);
}

/** The name of the tab it showed, undefined when there is none. */
export function lastTab(id: string | null | undefined): string | undefined {
	return id ? read<string>(TAB)[id] : undefined;
}

export function rememberTab(id: string | null | undefined, tab: string) {
	remember(TAB, id, tab);
}
