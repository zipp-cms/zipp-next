import type { AnyEntry, EntryRef } from './content';

export type Refs = Record<number, EntryRef>;

/** German lives on `/`, English on `/en`. */
export const sites = [
	{ handle: 'de', prefix: '', label: 'DE' },
	{ handle: 'en', prefix: '/en', label: 'EN' },
];

const path = location.pathname;
const current = path === '/en' || path.startsWith('/en/') ? sites[1] : sites[0];

export const site = current.handle;
/** The uri of the page in the cms, without the site prefix. */
export const uri = path.slice(current.prefix.length).replace(/^\/|\/$/g, '');

/** Entries and the entries they link to, only published ones. */
export async function entries<T extends AnyEntry>(
	params: Record<string, string>,
): Promise<{ entries: T[]; relations: Refs }> {
	const all = { site, ...params };
	const resp = await fetch('/api/entries?' + new URLSearchParams(all));
	if (!resp.ok) throw new Error(await resp.text());
	return resp.json();
}

/** The url of an entry in a site, by default this one. */
export function url(e: { uri: string | null }, siteHandle = site): string {
	const prefix = sites.find(s => s.handle === siteHandle)!.prefix;
	return `${prefix}/${e.uri ?? ''}`;
}
