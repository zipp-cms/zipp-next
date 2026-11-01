import { request } from '../core/client';
import type { Schema, Section } from '../core/schema';
import type { EntryRef } from './api';

/**
 * What the entry picker shows: the sections of the types, a structure as
 * its tree, a channel newest first; a search over all of them with the
 * path of each match.
 */

/** An entry to choose, with its parent for the tree. */
export type Choice = EntryRef & { parentId: number | null };

/** The entries of the sections that have one of the types. */
export function listChoices(
	types: string[],
	site: string,
): Promise<{ entries: Choice[] }> {
	const params = new URLSearchParams({ type: types.join(','), site });
	return request('GET', '/entries/choices?' + params);
}

export type Row = {
	e: Choice;
	depth: number;
	/** the titles above it in its tree */
	path: string[];
	/** whether entries are below it */
	parent: boolean;
	/** of one of the types; the others are there as parents in a tree */
	pickable: boolean;
};

export type Group = { section: Section; rows: Row[] };

/** The rows of each section in the schema's order, as the server sent them. */
export function groupChoices(
	schema: Schema,
	entries: Choice[],
	types: string[],
): Group[] {
	const byId = new Map(entries.map(e => [e.id, e]));
	const parents = new Set(entries.map(e => e.parentId));

	const pathOf = (e: Choice) => {
		const path: string[] = [];
		let up = e.parentId === null ? undefined : byId.get(e.parentId);
		while (up && path.length < 100) {
			path.unshift(up.title);
			up = up.parentId === null ? undefined : byId.get(up.parentId);
		}
		return path;
	};

	return schema.sections
		.map(section => {
			const tree = section.kind === 'structure';
			const rows = entries
				.filter(e => e.section === section.handle)
				// only a tree needs the other types, as parents
				.filter(e => tree || types.includes(e.type))
				.map(e => {
					const path = pathOf(e);
					return {
						e,
						depth: path.length,
						path,
						parent: parents.has(e.id),
						pickable: types.includes(e.type),
					};
				});
			return { section, rows };
		})
		.filter(g => g.rows.length);
}

/** The rows of a group not inside a closed one. */
export function openRows(rows: Row[], closed: Set<number>): Row[] {
	const hidden = new Set<number>();
	return rows.filter(r => {
		const p = r.e.parentId;
		if (p !== null && (hidden.has(p) || closed.has(p))) {
			hidden.add(r.e.id);
			return false;
		}
		return true;
	});
}

/**
 * What is closed at first: what has entries below it from the second
 * level on, so a big tree starts short; not the way to a chosen one.
 */
export function initiallyClosed(groups: Group[], chosen: number[]) {
	const closed = new Set<number>();
	const open = new Set<number>();
	for (const g of groups) {
		for (const r of g.rows) {
			if (r.parent && r.depth > 0) closed.add(r.e.id);
		}
		// the parents of a chosen one stay open
		const byId = new Map(g.rows.map(r => [r.e.id, r.e]));
		for (const id of chosen) {
			let p = byId.get(id)?.parentId ?? null;
			while (p !== null && !open.has(p)) {
				open.add(p);
				p = byId.get(p)?.parentId ?? null;
			}
		}
	}
	for (const id of open) closed.delete(id);
	return closed;
}

/** Lower case without accents: "uber" finds "Über". */
export function plain(s: string): string {
	return s
		.normalize('NFD')
		.replace(/\p{Diacritic}/gu, '')
		.toLowerCase();
}

/** The rows of all groups whose title or slug has every word of `q`. */
export function searchRows(groups: Group[], q: string): Row[] {
	const words = plain(q).split(/\s+/).filter(Boolean);
	return groups.flatMap(g =>
		g.rows.filter(r => {
			if (!r.pickable) return false;
			const text = plain(r.e.title + ' ' + r.e.slug);
			return words.every(w => text.includes(w));
		}),
	);
}
