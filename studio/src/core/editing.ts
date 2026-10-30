import { getContext, setContext } from 'svelte';

/**
 * What an editor edits, for the fields in it: a custom field tells its
 * plugin the entry and site, also in the flyout over another entry.
 */
export type Editing = { entryId: number | null; site: string };

const KEY = Symbol('editing');

export function setEditing(e: Editing) {
	setContext(KEY, e);
}

/** The editor around a field, none outside of one (Setup's preview). */
export function getEditing(): Editing | undefined {
	return getContext(KEY);
}
