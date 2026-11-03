import { sectionName, type Component, type Schema } from './schema';
import { visibleNav } from './access';

/** The paths of the studio's pages. */

/** The list of a section. */
export function sectionPath(section: string): string {
	return '/admin/entries/' + section;
}

/** An entry in the studio, `new` for the form of a new one. */
export function entryPath(section: string, id: number | 'new'): string {
	return `${sectionPath(section)}/${id}`;
}

/** An entry type, a block or a section in Setup. */
export function setupPath(
	tab: 'types' | 'blocks' | 'sections',
	handle: string,
): string {
	return `/admin/setup/${tab}/${handle}`;
}

/** An entry type or block in Setup. */
export function componentPath(c: Pick<Component, 'handle' | 'entry'>): string {
	return setupPath(c.entry ? 'types' : 'blocks', c.handle);
}

/** A field of an entry type or block in Setup, opened. */
export function fieldPath(
	c: Pick<Component, 'handle' | 'entry'>,
	handle: string,
): string {
	return `${componentPath(c)}#field-${handle}`;
}

/** The sites page of Setup, at its roles or its mail. */
export function sitesPath(part?: 'roles' | 'mail'): string {
	return '/admin/setup/sites' + (part ? '#' + part : '');
}

/** A user's account, `new` for the form of a new one. */
export function userPath(id: number | 'new'): string {
	return '/admin/users/' + id;
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
	const name = sectionName(schema, section);
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
