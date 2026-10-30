import type { Component, Revision } from '../api';

/** A date and time like the browser writes it. */
export function time(ms: number): string {
	return new Date(ms).toLocaleString();
}

/** A date like the browser writes it. */
export function date(ms: number): string {
	return new Date(ms).toLocaleDateString();
}

/** `publish` becomes `Publish`. */
export function capitalized(s: string): string {
	return s.charAt(0).toUpperCase() + s.slice(1);
}

/** A size of a file: `27 KB`, `1.4 MB`. */
export function fileSize(bytes: number): string {
	if (bytes < 1024 * 1024)
		return `${Math.max(1, Math.round(bytes / 1024))} KB`;
	return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
}

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
