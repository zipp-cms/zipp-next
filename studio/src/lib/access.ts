import { navGroups, type Access, type NavGroup, type Schema } from '../api';

/**
 * What the logged in user may do, from the roles of the schema. The
 * server checks it too, the studio only hides what isn't allowed.
 */
const LEVELS: Access[] = ['view', 'edit', 'publish', 'delete'];

function role(schema: Schema) {
	return schema.roles.find(r => r.handle === schema.me.role);
}

function enough(has: Access | null | undefined, need: Access) {
	return !!has && LEVELS.indexOf(has) >= LEVELS.indexOf(need);
}

/** The name of the user's role, `Admin` without one. */
export function roleName(schema: Schema): string {
	return isAdmin(schema) ? 'Admin' : (role(schema)?.name ?? schema.me.role!);
}

/** What the user may do in a section, for showing it. */
export function accessOf(schema: Schema, section: string): Access | null {
	if (isAdmin(schema)) return 'delete';
	return role(schema)?.sections[section] ?? null;
}

export function isAdmin(schema: Schema): boolean {
	return schema.me.role === null;
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
