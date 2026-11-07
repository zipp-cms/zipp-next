import type { Component, Field, Role, Schema, Section } from '../core/schema';
import { componentOf } from '../core/schema';
import { can, isAdmin, visibleNav } from '../core/access';
import { listEntries } from '../entries/api';
import { describeField } from './fieldKind';
import { session } from '../core/session.svelte';
import { t } from '../i18n/index.svelte';
import { ASSET_FIELDS } from '../assets/api';
import { USER_FIELDS } from '../users/api';
import { componentPath, fieldPath, setupPath, sitesPath } from '../core/paths';

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

/** The blocks; inline groups are part of their parent, no blocks. */
export function blocksOf(schema: Schema): Component[] {
	return schema.components.filter(c => !c.entry && !c.part_of);
}

/** The entry types of the sections the role sees. */
export function visibleEntryTypes(schema: Schema): Component[] {
	const sections = visibleSections(schema);
	return schema.components.filter(
		c => c.entry && sections.some(s => s.types.includes(c.handle)),
	);
}

/**
 * The GraphQL type of an entry type or block, the asset fields' is
 * `Asset`, the user fields' `User`.
 */
export function gqlType(c: Pick<Component, 'handle' | 'entry'>): string {
	if (!c.entry && c.handle === ASSET_FIELDS) return 'Asset';
	if (!c.entry && c.handle === USER_FIELDS) return 'User';
	return `${c.handle}_${c.entry ? 'Entry' : 'Block'}`;
}

/** A link to an entry type or block, for `Links`. */
export function componentLink(c: Pick<Component, 'handle' | 'name' | 'entry'>) {
	return { text: c.name, href: componentPath(c) };
}

/** Where a component is used: sections and fields, as links of Setup. */
export function usedIn(
	schema: Schema,
	c: Component,
): { text: string; href: string }[] {
	if (!c.entry && c.handle === ASSET_FIELDS)
		return [{ text: t('setup.everyAsset'), href: '/admin/assets' }];
	if (!c.entry && c.handle === USER_FIELDS)
		return [{ text: t('setup.everyUser'), href: '/admin/users' }];
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
	if (f.kind === 'custom') return f.blocks ?? [];
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

/** The counts on top of the overview, each with its tab. */
export function overviewStats(
	schema: Schema,
	counts: Record<string, number>,
): { n: number; label: string; href: string }[] {
	return [
		{
			n: visibleSections(schema).length,
			label: t('shell.setupSections'),
			href: 'sections',
		},
		{
			n: visibleEntryTypes(schema).length,
			label: t('setup.entryTypes'),
			href: 'types',
		},
		{
			n: blocksOf(schema).length,
			label: t('shell.setupBlocks'),
			href: 'blocks',
		},
		{
			n: fieldUsage(schema).length,
			label: t('setup.fields'),
			href: 'fields',
		},
		{
			n: Object.values(counts).reduce((a, b) => a + b, 0),
			label: t('setup.entries'),
			href: 'sections',
		},
		{ n: schema.sites.length, label: t('setup.sites'), href: 'sites' },
	];
}

/** What a plugin added, its handles as links of Setup, by kind. */
export function pluginParts(
	schema: Schema,
	p: Schema['pluginsAdded'][number],
): { name: string; links: { text: string; href?: string }[] }[] {
	const comps = p.components
		.map(h => componentOf(schema, h))
		.filter(c => c !== undefined);
	return [
		{
			name: t('shell.setupSections'),
			links: visibleSections(schema)
				.filter(s => p.sections.includes(s.handle))
				.map(s => ({
					text: s.name,
					href: setupPath('sections', s.handle),
				})),
		},
		{
			name: t('setup.typesAndBlocks'),
			links: comps.map(c => ({
				text: c.name,
				href: componentPath(c),
			})),
		},
		{
			name: t('setup.fields'),
			links: p.fields.map(f => {
				const [c, field] = f.split('.');
				const comp = componentOf(schema, c);
				const href = comp ? fieldPath(comp, field) : undefined;
				return { text: f, href };
			}),
		},
		{
			name: t('setup.fieldTypes'),
			links: p.fieldTypes.map(f => ({ text: f })),
		},
		{
			name: t('setup.transformsShort'),
			links: p.transforms.map(tr => ({
				text: tr,
				href: sitesPath(),
			})),
		},
	].filter(x => x.links.length);
}
