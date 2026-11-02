import { SEARCH_DELAY_MS } from 'zipp-ui/SearchInput.svelte';
import type { Asset } from '../assets/api';
import { sectionName, type Schema } from '../core/schema';
import { search, type EntryHit, type Found } from './api';
import { t } from '../i18n/index.svelte';

/** Whether the search over the page is open, from anywhere. */
export const searchBox = $state({ open: false });

/** A line of the results, what Enter opens. */
export type Hit =
	{ kind: 'entry'; entry: EntryHit } | { kind: 'asset'; asset: Asset };

/** The results under a heading: a section's entries, or the files. */
type HitGroup = { name: string; hits: Hit[] };

/**
 * The search of the studio as you type: it asks the server a moment
 * after the last key, shows only the answer of the newest words, and
 * keeps which line the arrow keys chose.
 */
export class Search {
	q = $state('');
	loading = $state(false);
	error = $state('');
	/** the chosen line, an index of `hits` */
	selected = $state(0);
	/** entries by section, in the order of their best one, then files */
	groups = $state.raw<HitGroup[]>([]);

	#schema: Schema;
	#timer: ReturnType<typeof setTimeout> | undefined;

	constructor(schema: Schema) {
		this.#schema = schema;
	}

	/** The lines of all groups, in their order. */
	get hits(): Hit[] {
		return this.groups.flatMap(g => g.hits);
	}

	/** Searches the words in a site after a moment. */
	type(q: string, site: string) {
		this.q = q;
		clearTimeout(this.#timer);
		if (!q.trim()) {
			this.groups = [];
			this.loading = false;
			return;
		}
		this.loading = true;
		this.#timer = setTimeout(() => this.load(q, site), SEARCH_DELAY_MS);
	}

	private async load(q: string, site: string) {
		try {
			const found = await search(q, site);
			if (q !== this.q) return;
			this.groups = group(this.#schema, found);
			this.error = '';
			this.selected = 0;
		} catch (e) {
			if (q === this.q) this.error = (e as Error).message;
		} finally {
			if (q === this.q) this.loading = false;
		}
	}

	/** Moves the chosen line up or down, around at the ends. */
	move(by: number) {
		const n = this.hits.length;
		if (n) this.selected = (this.selected + by + n) % n;
	}

	stop() {
		clearTimeout(this.#timer);
	}
}

function group(schema: Schema, found: Found): HitGroup[] {
	const out: HitGroup[] = [];
	const bySection = new Map<string, HitGroup>();
	for (const entry of found.entries) {
		let g = bySection.get(entry.section);
		if (!g) {
			g = { name: sectionName(schema, entry.section), hits: [] };
			bySection.set(entry.section, g);
			out.push(g);
		}
		g.hits.push({ kind: 'entry', entry });
	}
	if (found.assets.length) {
		const hits = found.assets.map(asset => ({ kind: 'asset', asset }));
		out.push({ name: t('search.assets'), hits: hits as Hit[] });
	}
	return out;
}

/**
 * A text in parts, `hit` where it has a word of the search, to show
 * what matched.
 */
export function highlight(
	text: string,
	q: string,
): { text: string; hit: boolean }[] {
	const words = q
		.split(/\s+/)
		.filter(w => w)
		.map(w => w.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'));
	if (!words.length) return [{ text, hit: false }];
	// at the start of a word, like the server finds them
	const re = new RegExp(`(?<![\\p{L}\\p{N}])(${words.join('|')})`, 'giu');
	// the words are the odd parts of the split
	return text
		.split(re)
		.map((t, i) => ({ text: t, hit: i % 2 === 1 }))
		.filter(p => p.text);
}
