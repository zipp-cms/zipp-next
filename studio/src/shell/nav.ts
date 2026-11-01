import type { Schema } from '../core/schema';
import { isAdmin, visibleNav } from '../core/access';
import { sectionOf } from '../core/schema';
import type { Key } from '../i18n/index.svelte';

/** The navigation of the studio: its groups, their items and links. */

export type NavGroup = ReturnType<typeof visibleNav>[number];

/**
 * The pages of Setup, also its crumbs: the handle and its name's key;
 * requests only for admins.
 */
export const SETUP_TABS: [string, Key][] = [
	['overview', 'shell.setupOverview'],
	['sections', 'shell.setupSections'],
	['types', 'shell.setupTypes'],
	['blocks', 'shell.setupBlocks'],
	['fields', 'shell.setupFields'],
	['sites', 'shell.setupSites'],
	['requests', 'shell.setupRequests'],
];

/** The pages of Setup the user sees. */
export function setupTabs(schema: Schema): [string, Key][] {
	return SETUP_TABS.filter(([h]) => h !== 'requests' || isAdmin(schema));
}

/** A group of only globals is settings, others are content. */
export function groupIcon(schema: Schema, group: NavGroup): string {
	const all = group.sections.map(h => sectionOf(schema, h));
	return all.every(s => s?.kind === 'single') ? 'globe' : 'entries';
}

/** The sections and docs of a group, in the order of the nav. */
export function groupItems(
	schema: Schema,
	group: NavGroup | undefined,
): { handle: string; name: string }[] {
	return (group?.sections ?? []).map(h => {
		const s = sectionOf(schema, h);
		if (s) return { handle: h, name: s.name };
		const d = schema.docs.find(d => d.handle === h);
		return { handle: h, name: d?.name ?? h };
	});
}
