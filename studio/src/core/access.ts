import { session } from './session.svelte';
import { t } from '../i18n/index.svelte';
import {
	navGroups,
	type Access,
	type NavGroup,
	type Role,
	type Schema,
} from './schema';

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
	if (handle === null) return t('common.admin');
	return schema.roles.find(r => r.handle === handle)?.name ?? handle;
}

/** Whether users of a role log into the studio, `null` is an admin. */
export function studioRole(schema: Schema, handle: string | null): boolean {
	return schema.roles.find(r => r.handle === handle)?.studio ?? true;
}

/** What the user may do in a section (their own entries apart), for
 * showing it. */
export function accessOf(schema: Schema, section: string): Access | null {
	if (isAdmin(schema)) return 'delete';
	return role(schema)?.sections[section] ?? null;
}

/** A level of access in words, `none` without one. */
export function accessText(
	level: Access | null | undefined,
	none = '–',
): string {
	return level ? t(`common.${level}`) : none;
}

/** A role's access to a section for showing it, with the one to the
 * entries its users made. */
export function roleAccessText(role: Role, section: string): string {
	const own = role.own[section];
	if (!own) return accessText(role.sections[section]);
	const mine = t('common.ownAccess', { access: accessText(own) });
	const all = role.sections[section];
	return all ? `${accessText(all)}, ${mine}` : mine;
}

export function isAdmin(schema: Schema): boolean {
	return session.user !== null && session.user.role === null;
}

export function can(schema: Schema, section: string, need: Access): boolean {
	return isAdmin(schema) || enough(role(schema)?.sections[section], need);
}

/**
 * What the user may do with an entry of a section that `author` made:
 * the section's access, for their own entries also the role's own one.
 */
export function canEntry(
	schema: Schema,
	section: string,
	author: number | null,
	need: Access,
): boolean {
	if (can(schema, section, need)) return true;
	const mine = author !== null && author === session.user?.id;
	return mine && enough(role(schema)?.own[section], need);
}

/** What the user may do with a new entry of a section: it is theirs. */
export function canNew(schema: Schema, section: string, need: Access) {
	return canEntry(schema, section, session.user?.id ?? null, need);
}

export function canAssets(schema: Schema, need: Access): boolean {
	return isAdmin(schema) || enough(role(schema)?.assets, need);
}

/** Whether the user may delete anything, what the trash shows. */
export function canTrash(schema: Schema): boolean {
	const r = role(schema);
	return (
		isAdmin(schema) ||
		enough(r?.assets, 'delete') ||
		[
			...Object.values(r?.sections ?? {}),
			...Object.values(r?.own ?? {}),
		].some(a => enough(a, 'delete'))
	);
}

/** The AI page: a model or the coding agent, and a role that may. */
export function canAi(schema: Schema): boolean {
	return (
		(schema.aiModels.length > 0 || hasAgent(schema)) &&
		(isAdmin(schema) || !!role(schema)?.ai)
	);
}

export function canSetup(schema: Schema): boolean {
	return isAdmin(schema) || !!role(schema)?.setup;
}

/** The nav without the sections the role may not see, docs stay. */
export function visibleNav(schema: Schema): NavGroup[] {
	return navGroups(schema)
		.map(g => ({
			...g,
			sections: g.sections.filter(
				h =>
					schema.docs.some(d => d.handle === h) ||
					canNew(schema, h, 'view'),
			),
		}))
		.filter(g => g.sections.length);
}

/**
 * The chat hands everything to the coding agent of a plugin that says
 * `chat` in its data: development and admins only, the server checks it
 * too.
 */
export function hasAgent(schema: Schema): boolean {
	return (
		schema.dev &&
		isAdmin(schema) &&
		schema.plugins.some(p => (p.data as { chat?: boolean } | null)?.chat)
	);
}
