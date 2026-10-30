import { session } from './session.svelte';
import { navGroups, type Access, type NavGroup, type Schema } from './schema';

/**
 * What the logged in user may do, from the roles of the schema. The
 * server checks it too, the studio only hides what isn't allowed.
 */
const LEVELS: Access[] = ['view', 'edit', 'publish', 'delete'];

function role(schema: Schema) {
	return schema.roles.find(r => r.handle === session.user?.role);
}

function enough(has: Access | null | undefined, need: Access) {
	return !!has && LEVELS.indexOf(has) >= LEVELS.indexOf(need);
}

/** The name of the user's role, `Admin` without one. */
export function roleName(schema: Schema): string {
	return roleNameOf(schema, session.user?.role ?? null);
}

/** The name of a role, `null` is an admin. */
export function roleNameOf(schema: Schema, handle: string | null): string {
	if (handle === null) return 'Admin';
	return schema.roles.find(r => r.handle === handle)?.name ?? handle;
}

/** What the user may do in a section, for showing it. */
export function accessOf(schema: Schema, section: string): Access | null {
	if (isAdmin(schema)) return 'delete';
	return role(schema)?.sections[section] ?? null;
}

export function isAdmin(schema: Schema): boolean {
	return session.user !== null && session.user.role === null;
}

export function can(schema: Schema, section: string, need: Access): boolean {
	return isAdmin(schema) || enough(role(schema)?.sections[section], need);
}

export function canAssets(schema: Schema, need: Access): boolean {
	return isAdmin(schema) || enough(role(schema)?.assets, need);
}

export function canAi(schema: Schema): boolean {
	return isAdmin(schema) || !!role(schema)?.ai;
}

/** The nav without the sections the role may not see, docs stay. */
export function visibleNav(schema: Schema): NavGroup[] {
	return navGroups(schema)
		.map(g => ({
			...g,
			sections: g.sections.filter(
				h =>
					schema.docs.some(d => d.handle === h) ||
					can(schema, h, 'view'),
			),
		}))
		.filter(g => g.sections.length);
}

/**
 * The chat hands everything to the coding agent of the plugin `agent`:
 * development and admins only, the server checks it too.
 */
export function hasAgent(schema: Schema): boolean {
	return (
		schema.dev &&
		isAdmin(schema) &&
		schema.plugins.some(p => p.name === 'agent')
	);
}

/** The page of a section or a doc. */
export function navHref(schema: Schema, handle: string): string {
	if (schema.docs.some(d => d.handle === handle))
		return '/admin/docs/' + handle;
	return '/admin/entries/' + handle;
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
	out.push({ label: name, href: '/admin/entries/' + section });
	return out;
}
