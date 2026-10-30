import type { Component, Link, Values } from '../core/schema';
import type { Refs } from '../entries/api';
import { plural } from '../core/format';
import { hasTarget } from './link/link';

/**
 * A block the outline of the editor asks to show: the path of its blocks
 * field and its index; `at` changes on every ask, also for the same one.
 */
export const focus = $state({ path: '', index: -1, at: 0 });

export function showBlock(path: string, index: number) {
	focus.path = path;
	focus.index = index;
	focus.at = Date.now();
}

/** The text of a rich text document, without its marks. */
function docText(doc: any): string {
	if (!doc || typeof doc !== 'object') return '';
	if (typeof doc.text === 'string') return doc.text;
	return (doc.content ?? []).map(docText).join(' ');
}

/** Where a link goes, short: the entry's title, a url's host. */
function linkTarget(link: Link, refs: Refs): string {
	if (link.type === 'entry' && typeof link.value === 'number')
		return refs[link.value]?.title ?? '';
	if (link.type === 'url') {
		try {
			return new URL(String(link.value)).host || String(link.value);
		} catch {
			return String(link.value);
		}
	}
	return String(link.value);
}

/**
 * What the head of a block shows: its first text or link (`Label →
 * target`), else how many images, entries or blocks it has.
 */
export function blockSummary(
	comp: Component | undefined,
	fields: Values,
	refs: Refs = {},
): string {
	if (!comp) return '';
	const counts: string[] = [];
	for (const f of comp.fields) {
		const v = fields[f.handle];
		if (v === undefined || v === null || v === '') continue;
		if (f.kind === 'text' && typeof v === 'string') return v.slice(0, 80);
		if (f.kind === 'link' && hasTarget(v as Link)) {
			const link = v as Link;
			const target = linkTarget(link, refs);
			return link.label ? `${link.label} → ${target}` : target;
		}
		if (f.kind === 'richText' || f.kind === 'markdown') {
			const t = typeof v === 'string' ? v : docText(v);
			if (t.trim()) return t.trim().slice(0, 80);
		}
		if (!Array.isArray(v) || !v.length) continue;
		if (f.kind === 'assets')
			counts.push(
				plural(
					v.length,
					f.images ? 'image' : 'file',
					f.images ? 'images' : 'files',
				),
			);
		else if (f.kind === 'relation')
			counts.push(plural(v.length, 'entry', 'entries'));
		else if (f.kind === 'blocks')
			counts.push(plural(v.length, 'block', 'blocks'));
	}
	return counts.join(' · ');
}

/** What a block misses that it needs, like `No link yet`; empty if nothing. */
export function blockMissing(
	comp: Component | undefined,
	fields: Values,
): string {
	for (const f of comp?.fields ?? []) {
		if (!f.required) continue;
		const v = fields[f.handle];
		if (f.kind === 'link') {
			if (!v || !hasTarget(v as Link)) return 'No link yet';
		} else if (
			v === undefined ||
			v === null ||
			v === '' ||
			(Array.isArray(v) && !v.length)
		)
			return `${f.label} is missing`;
	}
	return '';
}
