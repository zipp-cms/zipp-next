import type { Component, Role, Schema, Section } from '../core/schema';
import { can, isAdmin } from '../core/access';
import { session } from '../core/session.svelte';
import { capitalized } from '../core/format';

/** What Setup shows of the schema: only what the role may see. */

/** The sections the role may see. */
export function visibleSections(schema: Schema): Section[] {
	return schema.sections.filter(s => can(schema, s.handle, 'view'));
}

/** An admin sees every role, a role only itself. */
export function visibleRoles(schema: Schema): Role[] {
	return isAdmin(schema)
		? schema.roles
		: schema.roles.filter(r => r.handle === session.user?.role);
}

/** The entry types of the sections the role sees. */
export function visibleEntryTypes(schema: Schema): Component[] {
	const sections = visibleSections(schema);
	return schema.components.filter(
		c => c.entry && sections.some(s => s.types.includes(c.handle)),
	);
}

/** The GraphQL type of an entry type or block. */
export function gqlType(c: Pick<Component, 'handle' | 'entry'>): string {
	return `${c.handle}_${c.entry ? 'Entry' : 'Block'}`;
}

/** An access level as a word, `–` for none. */
export function accessText(level: string | null | undefined): string {
	return level ? capitalized(level) : '–';
}

/** Where a component is used: sections and fields, as links of Setup. */
export function usedIn(
	schema: Schema,
	c: Component,
): { text: string; href: string }[] {
	const out = visibleSections(schema)
		.filter(s => s.types.includes(c.handle))
		.map(s => ({
			text: `section ${s.name}`,
			href: `/admin/setup/sections/${s.handle}`,
		}));
	for (const other of schema.components) {
		for (const f of other.fields) {
			const uses =
				(f.kind === 'blocks' && f.types.includes(c.handle)) ||
				(f.kind === 'richText' && f.blocks.includes(c.handle)) ||
				(f.kind === 'group' && f.component === c.handle) ||
				(f.kind === 'relation' && f.types.includes(c.handle));
			if (uses)
				out.push({
					text: `${other.handle}.${f.handle}`,
					href: `/admin/setup/${other.entry ? 'types' : 'blocks'}/${other.handle}#field-${f.handle}`,
				});
		}
	}
	return out;
}

/** The uri of an entry type in a section: its own, the site's or the section's. */
export function uriOf(
	section: Section,
	type: string,
	site?: string,
): string | null {
	const own = section.type_uris?.[type];
	if (own !== undefined) return own;
	const inSite = site ? section.site_uris?.[site] : undefined;
	return inSite ?? section.uri ?? null;
}
