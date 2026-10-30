import type { Component } from '../../core/schema';
import { capitalized } from '../../core/format';
import type { Revision } from '../drafts';

/** What a revision changed: `Title (de), Intro (de, en)`. */
export function changeText(
	rev: Revision,
	comp: Component | undefined,
	multiSite: boolean,
): string {
	if (!rev.changes.length) return 'Created';
	const label = (f: string) =>
		comp?.fields.find(x => x.handle === f)?.label ?? capitalized(f);
	return rev.changes
		.map(
			c => label(c.field) + (multiSite ? ` (${c.sites.join(', ')})` : ''),
		)
		.join(', ');
}
