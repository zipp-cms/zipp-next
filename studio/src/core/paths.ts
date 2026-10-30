import type { Schema } from './schema';
import { visibleNav } from './access';

/** The paths of the studio's pages, besides `entryPath` of the router. */

/** The list of a section. */
export function sectionPath(section: string): string {
	return '/admin/entries/' + section;
}

/** An entry type, a block or a section in Setup. */
export function setupPath(
	tab: 'types' | 'blocks' | 'sections',
	handle: string,
): string {
	return `/admin/setup/${tab}/${handle}`;
}

/** The page of a section or a doc. */
export function navHref(schema: Schema, handle: string): string {
	if (schema.docs.some(d => d.handle === handle))
		return '/admin/docs/' + handle;
	return sectionPath(handle);
}

/** The way to a section in the nav: its group, then the section. */
export function sectionCrumbs(
	schema: Schema,
	section: string,
): { label: string; href?: string }[] {
	const group = visibleNav(schema).find(g => g.sections.includes(section));
	const name =
		schema.sections.find(s => s.handle === section)?.name ?? section;
	const out: { label: string; href?: string }[] = [];
	if (group && group.sections[0] !== section) {
		// the first of a group may be a doc
		out.push({
			label: group.name,
			href: navHref(schema, group.sections[0]),
		});
	}
	out.push({ label: name, href: sectionPath(section) });
	return out;
}
