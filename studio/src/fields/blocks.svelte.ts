import type { Component, Values } from '../core/schema';
import { plural } from '../core/list';

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

/**
 * What a collapsed block shows: its first text, else how many images,
 * entries or blocks it has.
 */
export function blockSummary(
	comp: Component | undefined,
	fields: Values,
): string {
	if (!comp) return '';
	const counts: string[] = [];
	for (const f of comp.fields) {
		const v = fields[f.handle];
		if (v === undefined || v === null || v === '') continue;
		if (f.kind === 'text' && typeof v === 'string') return v.slice(0, 80);
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
