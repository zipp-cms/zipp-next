import type { Component, Revision } from '../api';

/** A date and time like the browser writes it. */
export function time(ms: number): string {
	return new Date(ms).toLocaleString();
}

/** What a revision changed: `Title (de), Intro (de, en)`. */
export function changeText(
	rev: Revision,
	comp: Component | undefined,
	multiSite: boolean,
): string {
	if (!rev.changes.length) return 'Created';
	const label = (f: string) =>
		comp?.fields.find(x => x.handle === f)?.label ??
		f.charAt(0).toUpperCase() + f.slice(1);
	return rev.changes
		.map(
			c => label(c.field) + (multiSite ? ` (${c.sites.join(', ')})` : ''),
		)
		.join(', ');
}
