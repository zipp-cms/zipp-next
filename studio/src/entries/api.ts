import { request } from '../core/client';
import { type Values } from '../core/schema';

/**
 * When an entry is live, in all its sites (milliseconds): from its post
 * date (set when it is first published) until its expiry date.
 */
export type Dates = { postDate: number | null; expiryDate: number | null };

/** Whether the dates let an entry be live now, or why not. */
export function dateState(
	d: Dates,
	now = Date.now(),
): 'pending' | 'expired' | null {
	if (d.postDate !== null && d.postDate > now) return 'pending';
	if (d.expiryDate !== null && d.expiryDate <= now) return 'expired';
	return null;
}

export type Entry = Dates & {
	id: number;
	section: string;
	type: string;
	site: string;
	title: string;
	slug: string;
	uri: string | null;
	published: boolean;
	parentId: number | null;
	fields: Values;
	createdAt: number;
	updatedAt: number;
};

export type EntryRef = {
	id: number;
	section: string;
	type: string;
	title: string;
	slug: string;
	uri: string | null;
	/** in the site */
	published: boolean;
};

export type Refs = Record<number, EntryRef>;

/** A page of entries with what lists show of them. */
export type EntryList = {
	entries: Entry[];
	total: number;
	relations: Refs;
	published: Record<number, Record<string, boolean>>;
	/** the entries with unpublished changes */
	drafts: number[];
};

export function listEntries(
	params: Record<string, string>,
): Promise<EntryList> {
	return request('GET', '/entries?' + new URLSearchParams(params));
}

export function getEntry(
	id: number,
	site: string,
): Promise<{ entry: Entry; relations: Refs }> {
	return request('GET', `/entries/${id}?site=${site}`);
}

export function moveEntry(
	id: number,
	parentId: number | null,
	index: number,
): Promise<void> {
	return request('POST', `/entries/${id}/move`, { parentId, index });
}

export type EntryInput = {
	section: string;
	type: string;
	site: string;
	parentId: number | null;
	title: string;
	slug: string;
	published: boolean;
	fields: Values;
};

export function saveEntry(
	id: number | null,
	input: EntryInput,
): Promise<Entry> {
	if (id === null) return request('POST', '/entries', input);
	return request('PUT', '/entries/' + id, input);
}

export function deleteEntry(id: number): Promise<void> {
	return request('DELETE', '/entries/' + id);
}

/** What an entry's SEO is without its own values, the field's placeholders. */
