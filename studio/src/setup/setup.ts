import type {
	Access,
	Component,
	Field,
	Role,
	Schema,
	Section,
} from '../core/schema';
import { componentOf } from '../core/schema';
import { can, isAdmin, visibleNav } from '../core/access';
import { listEntries } from '../entries/api';
import { describeField } from '../fields/fieldKind';
import { session } from '../core/session.svelte';
import { t } from '../i18n/index.svelte';
import { ASSET_FIELDS } from '../assets/api';
import { fieldPath, setupPath } from '../core/paths';

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

/** The GraphQL type of an entry type or block, the asset fields' is `Asset`. */
export function gqlType(c: Pick<Component, 'handle' | 'entry'>): string {
	if (!c.entry && c.handle === ASSET_FIELDS) return 'Asset';
	return `${c.handle}_${c.entry ? 'Entry' : 'Block'}`;
}

/** A level of access in words, `none` without one. */
export function accessText(
	level: Access | null | undefined,
	none = '–',
): string {
	return level ? t(`common.${level}`) : none;
}

/** Where a component is used: sections and fields, as links of Setup. */
export function usedIn(
	schema: Schema,
	c: Component,
): { text: string; href: string }[] {
	if (!c.entry && c.handle === ASSET_FIELDS)
		return [{ text: t('setup.everyAsset'), href: '/admin/assets' }];
	const out = visibleSections(schema)
		.filter(s => s.types.includes(c.handle))
		.map(s => ({
			text: t('setup.sectionName', { name: s.name }),
			href: setupPath('sections', s.handle),
		}));
	for (const other of schema.components) {
		for (const f of other.fields) {
			const uses =
				heldBy(f).includes(c.handle) ||
				(f.kind === 'relation' && f.types.includes(c.handle));
			if (uses)
				out.push({
					text: `${other.handle}.${f.handle}`,
					href: fieldPath(other, f.handle),
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

/** The number of entries of each section the role sees, in the primary site. */
export async function entryCounts(
	schema: Schema,
): Promise<Record<string, number>> {
	const counts = await Promise.all(
		visibleSections(schema).map(s =>
			listEntries({ section: s.handle, limit: '1' })
				.then(r => [s.handle, r.total] as const)
				.catch(() => [s.handle, 0] as const),
		),
	);
	return Object.fromEntries(counts);
}

/** The blocks or groups a field holds, by handle. */
export function heldBy(f: Field): string[] {
	if (f.kind === 'blocks') return f.types;
	if (f.kind === 'richText') return f.blocks;
	if (f.kind === 'group') return [f.component];
	return [];
}

/** The fields of a component that hold blocks or groups, with them. */
export function holders(
	schema: Schema,
	c: Component,
): { field: Field; parts: Component[] }[] {
	return c.fields
		.map(field => ({
			field,
			parts: heldBy(field)
				.map(h => componentOf(schema, h))
				.filter(p => p !== undefined),
		}))
		.filter(h => h.parts.length);
}

/** The sections of each nav group, the ones of no group at the end. */
export function sectionsByGroup(
	schema: Schema,
): { name: string; sections: Section[] }[] {
	const sections = visibleSections(schema);
	const groups = visibleNav(schema)
		.map(g => ({
			name: g.name,
			sections: g.sections
				.map(h => sections.find(s => s.handle === h))
				.filter(s => s !== undefined),
		}))
		.filter(g => g.sections.length);
	const rest = sections.filter(
		s => !groups.some(g => g.sections.includes(s)),
	);
	if (rest.length) groups.push({ name: t('setup.notInNav'), sections: rest });
	return groups;
}

/** A field handle and every place it is in, like Craft's field usage. */
export type FieldUse = {
	handle: string;
	/** the labels it has, mostly one */
	labels: string[];
	/** its kinds as words, mostly one */
	kinds: string[];
	places: { component: Component; field: Field }[];
};

/** Every field handle of the entry types and blocks the role sees. */
export function fieldUsage(schema: Schema): FieldUse[] {
	const types = visibleEntryTypes(schema);
	const components = schema.components.filter(
		c => !c.entry || types.includes(c),
	);
	const byHandle = new Map<string, FieldUse>();
	for (const component of components) {
		for (const field of component.fields) {
			if (field.kind === 'note') continue;
			let use = byHandle.get(field.handle);
			if (!use) {
				use = {
					handle: field.handle,
					labels: [],
					kinds: [],
					places: [],
				};
				byHandle.set(field.handle, use);
			}
			const kind = describeField(field);
			if (!use.labels.includes(field.label)) use.labels.push(field.label);
			if (!use.kinds.includes(kind)) use.kinds.push(kind);
			use.places.push({ component, field });
		}
	}
	return [...byHandle.values()].sort((a, b) =>
		a.handle.localeCompare(b.handle),
	);
}
