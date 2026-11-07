import type { Link, LinkKind, Schema } from '../../core/schema';
import { hasPages, sectionOf } from '../../core/schema';
import type { Refs } from '../../entries/api';
import { t, type Key } from '../../i18n/index.svelte';

/** Links: of the link field, a navigation and the buttons of a rich text. */

const LINK_NAMES: Record<LinkKind, Key> = {
	entry: 'fields.linkEntry',
	url: 'fields.linkUrl',
	email: 'common.email',
	phone: 'fields.linkPhone',
	asset: 'fields.linkFile',
};

/** The name of a kind of link, in the user's language. */
export function linkName(kind: LinkKind): string {
	return t(LINK_NAMES[kind]);
}

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

/** The id of a link to an entry or a file, null for any other link. */
export function linkedId(link: Link, type: 'entry' | 'asset'): number | null {
	return link.type === type && typeof link.value === 'number'
		? link.value
		: null;
}

/** The href of a typed in link: a url with its anchor, mail, phone. */
export function linkHref(link: Link): string {
	const v = String(link.value ?? '');
	if (link.type === 'email') return `mailto:${v}`;
	if (link.type === 'phone') return `tel:${v}`;
	return v + (link.anchor ? `#${link.anchor}` : '');
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
		return {
			title: label || t('fields.noTargetYet'),
			sub: '',
			mono: false,
		};
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
		const name = file ?? t('fields.aFile');
		return { title: label || name, sub: label ? name : '', mono: false };
	}
	const value =
		String(link.value ?? '') + (link.type === 'url' ? anchor : '');
	return { title: label || value, sub: label ? value : '', mono: !!label };
}

/** What the links and buttons of a rich text go to. */
export const BUTTON_LINKS: LinkKind[] = ['entry', 'url', 'email', 'phone'];

/**
 * The entry types with a page, only they can be linked to; only those of
 * `sections` if any.
 */
export function linkableTypes(schema: Schema, sections: string[] = []) {
	return schema.sections
		.filter(hasPages)
		.filter(s => !sections.length || sections.includes(s.handle))
		.flatMap(s => s.types);
}

/** Like the website: external urls and files open in a new tab. */
function isExternal(schema: Schema, link: Link): boolean {
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

/** Where a link goes, short: the entry's title, a url's host. */
export function linkTarget(link: Link, refs: Refs): string {
	const entry = linkedId(link, 'entry');
	if (entry !== null) return refs[entry]?.title ?? `#${entry}`;
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
 * Whether a field's value counts as empty, like the server's required
 * check: nothing, an empty text or list, a link without a target, a
 * rich text without content.
 */
export function isEmpty(kind: string, v: unknown): boolean {
	if (v === undefined || v === null || v === '') return true;
	if (Array.isArray(v)) return !v.length;
	if (kind === 'link') return !hasTarget(v as Link);
	if (kind === 'richText')
		return !(v as { content?: unknown[] }).content?.length;
	return false;
}
