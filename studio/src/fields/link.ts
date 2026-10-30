import type { Field, Link, LinkKind, Schema } from '../core/schema';
import type { Refs } from '../entries/api';

/** Links: of the link field, a navigation and the buttons of a rich text. */

export const LINK_NAMES: Record<LinkKind, string> = {
	entry: 'Entry',
	url: 'URL',
	email: 'Email',
	phone: 'Phone',
	asset: 'File',
};

/** The field of a link that isn't a field of the schema itself. */
export function linkField(
	to: LinkKind[],
	{ label = 'Link', labels = true } = {},
): Field {
	return {
		handle: 'link',
		label,
		required: false,
		indexed: false,
		translatable: false,
		kind: 'link',
		to,
		labels,
		newTab: true,
	};
}

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
