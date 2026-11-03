import type { Block, Component, Link, Schema, Values } from '../core/schema';
import { componentOf } from '../core/schema';
import { t } from '../i18n/index.svelte';
import type { Refs } from '../entries/api';
import { hasTarget, isEmpty, linkTarget } from './link/link';

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

/**
 * A copy of a block is new, also the blocks in it: its fields' blocks,
 * the blocks embedded in a rich text and those in groups lose their ids,
 * saving gives them new ones. Changes `block`, a snapshot.
 */
export function withoutIds(schema: Schema, block: Block) {
	delete block.id;
	fieldsWithoutIds(schema, block.type, block.fields);
}

function fieldsWithoutIds(schema: Schema, type: string, values: Values) {
	for (const f of componentOf(schema, type)?.fields ?? []) {
		const v = values?.[f.handle] as any;
		if (!v) continue;
		if (f.kind === 'blocks' && Array.isArray(v))
			for (const b of v) withoutIds(schema, b);
		else if (f.kind === 'group') fieldsWithoutIds(schema, f.component, v);
		else if (f.kind === 'richText')
			for (const n of v.content ?? []) {
				if (n.type !== 'embed' || !n.attrs) continue;
				n.attrs.id = null;
				fieldsWithoutIds(schema, n.attrs.type, n.attrs.fields);
			}
	}
}

/** The text of a rich text document, without its marks. */
function docText(doc: any): string {
	if (!doc || typeof doc !== 'object') return '';
	if (typeof doc.text === 'string') return doc.text;
	return (doc.content ?? []).map(docText).join(' ');
}

/**
 * The value of a block's title field as a short text: a dropdown's label,
 * a number with its unit, a link's label or target, an entry's title.
 */
function blockTitle(
	comp: Component | undefined,
	fields: Values,
	refs: Refs = {},
): string {
	const f = comp?.fields.find(f => f.handle === comp.title_field);
	const v = f && fields[f.handle];
	if (!f || v === undefined || v === null || v === '') return '';
	let t: string;
	if (f.kind === 'dropdown' || f.kind === 'buttonGroup')
		t = f.options.find(o => o.value === v)?.label ?? String(v);
	else if (f.kind === 'number') t = f.unit ? `${v} ${f.unit}` : String(v);
	else if (f.kind === 'date') t = String(v).replace('T', ' ');
	else if (f.kind === 'link')
		t = hasTarget(v as Link)
			? (v as Link).label || linkTarget(v as Link, refs)
			: '';
	else if (f.kind === 'relation')
		t = Array.isArray(v) && v.length ? (refs[v[0]]?.title ?? '') : '';
	else t = typeof v === 'string' ? v : docText(v);
	return t.trim().slice(0, 80);
}

/**
 * What the head of a block shows: its title field, else its first text
 * or link (`Label → target`), else how many images, entries or blocks it
 * has.
 */
export function blockSummary(
	comp: Component | undefined,
	fields: Values,
	refs: Refs = {},
): string {
	if (!comp) return '';
	const title = blockTitle(comp, fields, refs);
	if (title) return title;
	const counts: string[] = [];
	for (const f of comp.fields) {
		const v = fields[f.handle];
		if (v === undefined || v === null || v === '') continue;
		if ((f.kind === 'text' || f.kind === 'email') && typeof v === 'string')
			return v.slice(0, 80);
		if (f.kind === 'link' && hasTarget(v as Link)) {
			const link = v as Link;
			const target = linkTarget(link, refs);
			return link.label ? `${link.label} → ${target}` : target;
		}
		if (f.kind === 'richText' || f.kind === 'markdown') {
			const text = typeof v === 'string' ? v : docText(v);
			if (text.trim()) return text.trim().slice(0, 80);
		}
		if (!Array.isArray(v) || !v.length) continue;
		if (f.kind === 'assets')
			counts.push(
				t(f.images ? 'fields.images' : 'common.files', { n: v.length }),
			);
		else if (f.kind === 'relation')
			counts.push(t('common.entries', { n: v.length }));
		else if (f.kind === 'blocks')
			counts.push(t('fields.blocks', { n: v.length }));
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
		if (!isEmpty(f.kind, fields[f.handle])) continue;
		return f.kind === 'link'
			? t('fields.noLinkYet')
			: t('fields.isMissing', { label: f.label });
	}
	return '';
}
