import { placeIn, type Drop } from 'zipp-ui/sortable';
import { Busy } from '../core/busy.svelte';
import { Latest } from '../core/latest.svelte';
import { site } from '../core/router.svelte';
import type { Section } from '../core/schema';
import { ancestors, depths as depthsOf } from '../core/tree';
import { t } from '../i18n/index.svelte';
import {
	deleteEntry,
	listEntries,
	moveEntry,
	type Entry,
	type EntryList,
} from './api';

/** What the list of a section asks for: a structure in its order. */
export function listParams(
	section: Section,
	site: string,
	search: string,
): Record<string, string> {
	const params: Record<string, string> = {
		section: section.handle,
		site,
		limit: '500',
	};
	if (section.kind === 'structure') params.order = 'structure';
	if (search) params.search = search;
	return params;
}

/**
 * The entries of a section's page: searched a moment after the last
 * key, moved in a structure, deleted. It loads again after each, only
 * the last load counts, an older answer would show an old order.
 */
export class EntryListState {
	entries = $state<Entry[]>([]);
	published = $state<EntryList['published']>({});
	/** the entries with unpublished changes */
	drafts = $state<number[]>([]);
	total = $state(0);
	readonly loads = new Latest();
	/** moving and deleting */
	readonly action = new Busy();
	#search = $state('');

	constructor(
		private section: Section,
		first: EntryList,
	) {
		this.#show(first);
	}

	get search(): string {
		return this.#search;
	}

	set search(q: string) {
		if (q === this.#search) return;
		this.#search = q;
		this.loads.later(this.#ask, this.#show);
	}

	/** How deep each entry is, in a structure. */
	readonly depths = $derived(depthsOf(this.entries));

	#ask = () =>
		listEntries(listParams(this.section, site.current, this.#search));

	#show = (r: EntryList) => {
		this.entries = r.entries;
		this.total = r.total;
		this.published = r.published;
		this.drafts = r.drafts;
		this.action.error = '';
	};

	load() {
		return this.loads.run(this.#ask, this.#show);
	}

	/** Moves an entry of a structure; it changes the website at once. */
	move(from: number, drop: Drop) {
		const to = placeIn(this.depths, from, drop);
		const parentId = to.parent === null ? null : this.entries[to.parent].id;
		return this.action.run(async () => {
			await moveEntry(this.entries[from].id, parentId, to.index);
			// the handle keeps the focus in the new order
			await this.load();
		});
	}

	/** Moves it to the trash after asking, the entries below it too. */
	async remove(e: Entry) {
		const below = await this.#countBelow(e);
		if (below === undefined) return;
		const question = below
			? t('entries.confirmTrashBelow', { title: e.title, n: below })
			: t('entries.confirmTrash', { title: e.title });
		if (!confirm(question)) return;
		await this.action.run(async () => {
			await deleteEntry(e.id);
			this.load();
		});
	}

	/** While searching the list has only the matches: the whole tree. */
	async #countBelow(e: Entry): Promise<number | undefined> {
		let all = this.entries;
		if (this.section.kind === 'structure' && this.#search) {
			const r = await this.action.run(() =>
				listEntries(listParams(this.section, site.current, '')),
			);
			if (!r) return;
			all = r.entries;
		}
		const byId = new Map(all.map(c => [c.id, c]));
		return all.filter(c => ancestors(byId, c).some(a => a.id === e.id))
			.length;
	}

	stop() {
		this.loads.stop();
	}
}
