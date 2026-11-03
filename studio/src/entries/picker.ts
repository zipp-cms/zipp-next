import type { Schema, Section } from '../core/schema';
import { componentName } from '../core/schema';
import { dateState, type Choice } from './api';
import { visibleNav } from '../core/access';
import { ancestors } from '../core/tree';
import { plain } from '../core/format';
import { t } from '../i18n/index.svelte';

/**
 * What the entry picker shows: the sections of the types grouped like
 * the nav, a structure as its tree, a channel newest first; a search
 * over all of them with the path of each match.
 */

/** "Choose Seite" with one type, "Choose entry" with several. */
export function chooseLabel(schema: Schema, types: string[]): string {
	return types.length === 1
		? t('entries.chooseType', { type: componentName(schema, types[0]) })
		: t('entries.chooseEntry');
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

	const pathOf = (e: Choice) =>
		ancestors(byId, e)
			.map(a => a.title)
			.reverse();

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
			const e = byId.get(id);
			if (e) for (const a of ancestors(byId, e)) open.add(a.id);
		}
	}
	for (const id of open) closed.delete(id);
	return closed;
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

/** The groups of the nav with the sections to pick from, in its order. */
export type NavPart = { name: string; groups: Group[] };

/**
 * The sections of `groups` as the sidebar groups them; one the nav
 * doesn't show comes last, without a name.
 */
export function pickerNav(schema: Schema, groups: Group[]): NavPart[] {
	const parts = visibleNav(schema)
		.map(n => ({
			name: n.name,
			groups: n.sections
				.map(h => groups.find(g => g.section.handle === h))
				.filter(g => g !== undefined),
		}))
		.filter(p => p.groups.length);
	const shown = new Set(parts.flatMap(p => p.groups));
	const rest = groups.filter(g => !shown.has(g));
	if (rest.length) parts.push({ name: '', groups: rest });
	return parts;
}

/**
 * The section it opens in: the one of a chosen entry, else the one of
 * last time, else the first.
 */
export function startSection(
	groups: Group[],
	chosen: number[],
	last: string | null,
): string | null {
	const has = (h: string | null) => groups.some(g => g.section.handle === h);
	const ofChosen = groups.find(g =>
		g.rows.some(r => chosen.includes(r.e.id)),
	);
	if (ofChosen) return ofChosen.section.handle;
	if (has(last)) return last;
	return groups[0]?.section.handle ?? null;
}

/** How many entries of a group can be picked, for the nav. */
export const pickable = (g: Group) => g.rows.filter(r => r.pickable).length;

/**
 * The status dot of an entry: live, published but out of its dates, or
 * not published.
 */
export function choiceState(e: Choice): {
	state: 'live' | 'waiting' | 'off';
	label: string;
} {
	if (!e.published) return { state: 'off', label: t('entries.notPublished') };
	const dates = dateState(e);
	if (dates === 'pending')
		return { state: 'waiting', label: t('entries.pending') };
	if (dates === 'expired')
		return { state: 'waiting', label: t('entries.expired') };
	return { state: 'live', label: t('entries.live') };
}
