import type { AnyEntry, EntryRef } from './content';

export type Refs = Record<number, EntryRef>;

/** Entries and the entries they link to, only published ones. */
export async function entries<T extends AnyEntry>(
	params: Record<string, string>,
): Promise<{ entries: T[]; relations: Refs }> {
	const resp = await fetch('/api/entries?' + new URLSearchParams(params));
	if (!resp.ok) throw new Error(await resp.text());
	return resp.json();
}

/** Where an entry lives on this site. */
export function url(e: EntryRef): string {
	return e.type === 'event' ? `/events/${e.slug}` : `/${e.slug}`;
}
