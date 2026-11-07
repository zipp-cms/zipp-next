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
	/** who made it, null: the system or a deleted user */
	authorId: number | null;
};

/** Who made an entry, the public part of a user. */
export type Author = { id: number; name: string };

export type EntryRef = Dates & {
	id: number;
	section: string;
	type: string;
	title: string;
	slug: string;
	uri: string | null;
	/** in the site */
	published: boolean;
	authorId: number | null;
};

export type Refs = Record<number, EntryRef>;

/** An entry to choose, with its parent for the tree and its draft. */
export type Choice = EntryRef & { parentId: number | null; draft: boolean };

/**
 * How many entries of a section there are: `entries` in its order (a
 * structure with its other types), `pickable` of the types.
 */
export type ChoiceCount = { entries: number; pickable: number };

export type Choices = {
	entries: Choice[];
	/** per section, also of what wasn't sent */
	totals: Record<string, ChoiceCount>;
	/** in a search the parents of the found ones, for their paths */
	parents: Choice[];
};

/**
 * The entries of the sections that have one of the types, only of
 * `sections` if any: up to a limit per section, more from `offset`;
 * with `search` the ones that have its words.
 */
export function listChoices(
	types: string[],
	site: string,
	sections: string[] = [],
	more: { offset?: number; search?: string } = {},
): Promise<Choices> {
	const params = new URLSearchParams({ type: types.join(','), site });
	if (sections.length) params.set('section', sections.join(','));
	if (more.offset) params.set('offset', String(more.offset));
	if (more.search) params.set('search', more.search);
	return request('GET', '/entries/choices?' + params);
}

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
): Promise<{ entry: Entry; relations: Refs; author: Author | null }> {
	return request('GET', `/entries/${id}?site=${site}`);
}

/** Gives an entry another author, admins only. */
export function setAuthor(id: number, authorId: number | null): Promise<void> {
	return request('PUT', `/entries/${id}/author`, { authorId });
}

export function moveEntry(
	id: number,
	parentId: number | null,
	index: number,
): Promise<void> {
	return request('POST', `/entries/${id}/move`, { parentId, index });
}

type EntryInput = {
	section: string;
	type: string;
	site: string;
	parentId: number | null;
	title: string;
	slug: string;
	published: boolean;
	fields: Values;
};

export function createEntry(input: EntryInput): Promise<Entry> {
	return request('POST', '/entries', input);
}

export function deleteEntry(id: number): Promise<void> {
	return request('DELETE', '/entries/' + id);
}

/** What an entry's SEO is without its own values, the field's placeholders. */
