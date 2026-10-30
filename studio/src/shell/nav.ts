import type { Schema } from '../core/schema';
import { visibleNav } from '../core/access';
import { sectionOf } from '../core/schema';

/** The navigation of the studio: its groups, their items and links. */

export type NavGroup = ReturnType<typeof visibleNav>[number];

/** The pages of Setup, also its crumbs. */
export const SETUP_TABS: [string, string][] = [
	['sections', 'Sections'],
	['types', 'Entry types'],
	['blocks', 'Blocks'],
	['sites', 'Sites, transforms and roles'],
];

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
