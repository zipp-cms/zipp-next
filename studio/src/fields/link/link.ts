import type { Link, LinkKind, Schema } from '../../core/schema';
import { sectionOf } from '../../core/schema';
import type { Refs } from '../../entries/api';

/** Links: of the link field, a navigation and the buttons of a rich text. */

export const LINK_NAMES: Record<LinkKind, string> = {
	entry: 'Entry',
	url: 'URL',
	email: 'Email',
	phone: 'Phone',
	asset: 'File',
};

/** The icon of each kind of link. */
export const LINK_ICONS: Record<LinkKind, string> = {
	entry: 'page',
	url: 'link',
	email: 'mail',
	phone: 'phone',
	asset: 'file',
};

/** Whether a link has a target yet. */
export function hasTarget(link: Link): boolean {
	return link.value !== null && link.value !== '';
}

/** Whether the link opens in a new tab: set, else external ones do. */
export function opensInNewTab(
	schema: Schema,
	link: Link,
	externalInNewTab: boolean,
): boolean {
	return link.newTab ?? (externalInNewTab && isExternal(schema, link));
}

/** Entries and urls may point to a section of the page. */
export function hasAnchor(type: LinkKind): boolean {
	return type === 'entry' || type === 'url';
}

/**
 * A link in a row: its label or target as the title, under it where it
 * goes (`→ Bern · Pages · #anfahrt`); `mono` when that's a url. `file`
 * is the filename of a linked asset.
 */
export function linkSummary(
	schema: Schema,
	link: Link,
	refs: Refs,
	file?: string,
): { title: string; sub: string; mono: boolean } {
	const anchor = link.anchor ? `#${link.anchor}` : '';
	const label = link.label?.trim();
	if (!hasTarget(link))
		return { title: label || 'No target yet', sub: '', mono: false };
	if (link.type === 'entry') {
		const ref = typeof link.value === 'number' ? refs[link.value] : null;
		const target = ref?.title ?? `#${link.value}`;
		const section = ref ? (sectionOf(schema, ref.section)?.name ?? '') : '';
		const sub = [label ? `→ ${target}` : '', section, anchor];
		return {
			title: label || target,
			sub: sub.filter(Boolean).join(' · '),
			mono: false,
		};
	}
	if (link.type === 'asset') {
		const name = file ?? 'A file';
		return { title: label || name, sub: label ? name : '', mono: false };
	}
	const value =
		String(link.value ?? '') + (link.type === 'url' ? anchor : '');
	return { title: label || value, sub: label ? value : '', mono: !!label };
}

/** What the links and buttons of a rich text go to. */
export const BUTTON_LINKS: LinkKind[] = ['entry', 'url', 'email', 'phone'];

/** The entry types with a page, only they can be linked to. */
export function linkableTypes(schema: Schema): string[] {
	return schema.sections
		.filter(s => s.uri !== null || Object.keys(s.type_uris ?? {}).length)
		.flatMap(s => s.types);
}

/** Like the website: external urls and files open in a new tab. */
export function isExternal(schema: Schema, link: Link): boolean {
	if (link.type === 'asset') return true;
	if (link.type !== 'url') return false;
	const url = String(link.value ?? '');
	return (
		url.startsWith('http') &&
		!Object.values(schema.siteUrls).some(b => b && url.startsWith(b))
	);
}

/** The input of a link that is typed in. */
export function linkInput(type: LinkKind): {
	type: string;
	placeholder: string;
} {
	switch (type) {
		case 'email':
			return { type: 'email', placeholder: 'name@example.ch' };
		case 'phone':
			return { type: 'tel', placeholder: '+41 31 …' };
		default:
			return { type: 'url', placeholder: 'https://…' };
	}
}

/** Where a link goes, in words: the title of its entry or its value. */
export function linkTarget(link: Link, refs: Refs): string {
	if (link.value === null || link.value === '') return 'no target';
	if (link.type === 'entry' && typeof link.value === 'number')
		return refs[link.value]?.title ?? `#${link.value}`;
	return String(link.value);
}

/** What a link says: its label, else where it goes. */
export function linkLabel(link: Link, refs: Refs, fallback = 'Link'): string {
	if (link.label) return link.label;
	if (link.value === null || link.value === '') return fallback;
	return linkTarget(link, refs);
}
