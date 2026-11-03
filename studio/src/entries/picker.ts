import type { Schema, Section } from '../core/schema';
import { componentName } from '../core/schema';
import {
	dateState,
	type Choice,
	type ChoiceCount,
	type Choices,
	type EntryRef,
} from './api';
import { visibleNav } from '../core/access';
import { ancestors } from '../core/tree';
import { plain } from '../core/format';
import { t } from '../i18n/index.svelte';

/**
 * What the entry picker shows: the sections of the types grouped like
 * the nav, a structure as its tree, a channel newest first; a search
 * over all of them with the path of each match. The state of the
 * picker is in `entryPicker.svelte.ts`.
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
	/** the ids above it, for what is open */
	parents: number[];
	/** whether entries are below it */
	parent: boolean;
	/** of one of the types; the others are there as parents in a tree */
	pickable: boolean;
};

/** The rows of a section, of as many as there are. */
export type Group = { section: Section; rows: Row[]; total: ChoiceCount };

/** The rows of entries, their parents in `byId`. */
function toRows(
	entries: Choice[],
	byId: Map<number, Choice>,
	types: string[],
): Row[] {
	const withChildren = new Set([...byId.values()].map(e => e.parentId));
	return entries.map(e => {
		const up = ancestors(byId, e);
		return {
			e,
			depth: up.length,
			path: up.map(a => a.title).reverse(),
			parents: up.map(a => a.id),
			parent: withChildren.has(e.id),
			pickable: types.includes(e.type),
		};
	});
}

/** The rows of each section in the schema's order, as the server sent them. */
export function groupChoices(
	schema: Schema,
	entries: Choice[],
	types: string[],
	totals: Record<string, ChoiceCount>,
): Group[] {
	const byId = new Map(entries.map(e => [e.id, e]));
	const rows = toRows(entries, byId, types);

	return schema.sections
		.map(section => ({
			section,
			rows: rows.filter(r => r.e.section === section.handle),
			total: totals[section.handle] ?? { entries: 0, pickable: 0 },
		}))
		.filter(g => g.rows.length);
}

/** The found ones of a search of the server, with their paths. */
export function foundRows(r: Choices, types: string[]): Row[] {
	const byId = new Map([...r.parents, ...r.entries].map(e => [e.id, e]));
	return toRows(r.entries, byId, types);
}

/**
 * What is closed at first: what has entries below it from the second
 * level on, so a big tree starts short; not the way to a chosen one.
 */
export function initiallyClosed(rows: Row[], chosen: number[]): Set<number> {
	const open = new Set(
		rows.filter(r => chosen.includes(r.e.id)).flatMap(r => r.parents),
	);
	const closed = rows.filter(r => r.parent && r.depth > 0);
	return new Set(closed.map(r => r.e.id).filter(id => !open.has(id)));
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

/**
 * The status dot of an entry: live, published but out of its dates, or
 * not published.
 */
export function choiceState(e: EntryRef): {
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
