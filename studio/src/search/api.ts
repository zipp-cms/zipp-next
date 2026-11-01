import { request } from '../core/client';
import type { Asset } from '../assets/api';

/** An entry the search found. */
export type EntryHit = {
	id: number;
	section: string;
	type: string;
	title: string;
	/** in the site */
	published: boolean;
	/** the words around the match, null when the title or slug matched */
	snippet: string | null;
	/** its editor */
	path: string;
};

/** What a search found, the best first. */
export type Found = { entries: EntryHit[]; assets: Asset[] };

/** Entries of a site and assets with all the words, the role's only. */
export function search(q: string, site: string): Promise<Found> {
	const p = new URLSearchParams({ q });
	if (site) p.set('site', site);
	return request('GET', '/search?' + p);
}
