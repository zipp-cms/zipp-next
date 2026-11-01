import type { Component } from '../../core/schema';
import { capitalized } from '../../core/format';
import type { Revision } from '../drafts';
import { t, type Key } from '../../i18n/index.svelte';

/** What an entry has besides its fields, as a change names it. */
const ENTRY_LABELS: Record<string, Key> = {
	title: 'common.title',
	slug: 'entries.slug',
	type: 'entries.type',
	published: 'common.published',
	postDate: 'entries.postDate',
	expiryDate: 'entries.expiryDate',
};

/** What a revision changed: `Title (de), Intro (de, en)`. */
export function changeText(
	rev: Revision,
	comp: Component | undefined,
	multiSite: boolean,
): string {
	if (!rev.changes.length) return t('entries.created');
	const label = (f: string) =>
		comp?.fields.find(x => x.handle === f)?.label ??
		(ENTRY_LABELS[f] ? t(ENTRY_LABELS[f]) : undefined) ??
		capitalized(f);
	return rev.changes
		.map(
			c => label(c.field) + (multiSite ? ` (${c.sites.join(', ')})` : ''),
		)
		.join(', ');
}
