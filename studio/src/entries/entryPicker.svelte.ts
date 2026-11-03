import { site } from '../core/router.svelte';
import type { Schema } from '../core/schema';
import { isFull } from '../core/list';
import { Latest } from '../core/latest.svelte';
import { visibleRows } from '../core/tree';
import { componentName } from '../core/schema';
import {
	listChoices,
	type Choice,
	type ChoiceCount,
	type Choices,
} from './api';
import {
	foundRows,
	groupChoices,
	initiallyClosed,
	pickerNav,
	searchRows,
	startSection,
	type Row,
} from './picker';
import { t } from '../i18n/index.svelte';

/** The section the picker showed last, it opens there again. */
let lastSection: string | null = null;

/**
 * The state of EntryPicker: the entries of the sections, the one shown,
 * what is closed in a tree, the search. A section comes in parts of a
 * limit (1000), the rest as it scrolls; while one isn't whole the
 * search asks the server, else it looks at what is here.
 */
export class EntryPickerState {
	entries = $state<Choice[]>([]);
	totals = $state<Record<string, ChoiceCount>>({});
	loaded = $state(false);
	error = $state('');
	closed = $state(new Set<number>());
	current = $state<string | null>(null);
	/** the answer of the server's search */
	answer = $state<Choices | null>(null);
	/** the server's search, the newest counts */
	readonly finder = new Latest();
	/** more of a section */
	readonly more = new Latest();
	#search = $state('');

	readonly groups = $derived.by(() =>
		groupChoices(
			this.o.schema(),
			this.entries,
			this.o.types(),
			this.totals,
		),
	);
	readonly nav = $derived.by(() => pickerNav(this.o.schema(), this.groups));
	readonly group = $derived(
		this.groups.find(g => g.section.handle === this.current),
	);
	readonly searching = $derived(this.#search.trim() !== '');
	/** whether every section is here, so the search runs here */
	readonly whole = $derived(
		this.groups.every(g => g.rows.length >= g.total.entries),
	);
	readonly found = $derived.by(() => {
		if (!this.searching) return [];
		if (this.whole) return searchRows(this.groups, this.#search);
		return this.answer ? foundRows(this.answer, this.o.types()) : [];
	});
	/** found by the server, but more than it sent */
	readonly foundMore = $derived(
		!this.whole &&
			this.answer !== null &&
			Object.values(this.answer.totals).reduce(
				(n, c) => n + c.entries,
				0,
			) > this.answer.entries.length,
	);
	readonly tree = $derived(this.group?.section.kind === 'structure');
	readonly rows = $derived.by(() => {
		const g = this.group;
		if (!g) return [];
		if (!this.tree) return g.rows;
		return visibleRows(g.rows, id => !this.closed.has(id));
	});
	/** whether the section shown has more than is here */
	readonly hasMore = $derived(
		!!this.group && this.group.rows.length < this.group.total.entries,
	);
	readonly full = $derived.by(() =>
		isFull(this.o.chosen().length, this.o.max()),
	);

	constructor(
		private o: {
			schema: () => Schema;
			/** the entry types to choose from */
			types: () => string[];
			/** only entries of these sections, none: all */
			sections: () => string[];
			chosen: () => number[];
			multiple: () => boolean;
			max: () => number | null;
			onpick: (e: Choice) => void;
			onclose: () => void;
		},
	) {}

	get search() {
		return this.#search;
	}

	/** Typing: a search on the server a moment later, if it needs one. */
	set search(q: string) {
		this.#search = q;
		if (!q.trim() || this.whole) {
			this.finder.stop();
			this.answer = null;
			return;
		}
		this.finder.later(
			() =>
				listChoices(this.o.types(), site.current, this.o.sections(), {
					search: q.trim(),
				}),
			r => (this.answer = r),
		);
	}

	/** The first part of each section, it opens at a chosen one. */
	async load() {
		try {
			const r = await listChoices(
				this.o.types(),
				site.current,
				this.o.sections(),
			);
			this.entries = r.entries;
			this.totals = r.totals;
			const rows = this.groups.flatMap(g => g.rows);
			const chosen = this.o.chosen();
			this.closed = initiallyClosed(rows, chosen);
			this.current = startSection(this.groups, chosen, lastSection);
		} catch (e) {
			this.error = (e as Error).message;
		} finally {
			this.loaded = true;
		}
	}

	/** The next part of the section shown, after the last one here. */
	loadMore() {
		const g = this.group;
		if (!g || !this.hasMore || this.more.loading) return;
		const before = new Set(g.rows.filter(r => r.parent).map(r => r.e.id));
		this.more.run(
			() =>
				listChoices(this.o.types(), site.current, [g.section.handle], {
					offset: g.rows.length,
				}),
			r => {
				const known = new Set(this.entries.map(e => e.id));
				const added = r.entries.filter(e => !known.has(e.id));
				this.entries = [...this.entries, ...added];
				// what got entries below it now starts closed too
				const rows = this.group?.rows ?? [];
				const closed = initiallyClosed(rows, this.o.chosen());
				const next = new Set(this.closed);
				for (const id of closed) if (!before.has(id)) next.add(id);
				this.closed = next;
			},
		);
	}

	select(handle: string) {
		lastSection = handle;
		this.current = handle;
		this.search = '';
	}

	isChosen = (r: Row) => this.o.chosen().includes(r.e.id);

	canPick = (r: Row) =>
		r.pickable && (this.isChosen(r) ? this.o.multiple() : !this.full);

	pick(r: Row) {
		if (!this.canPick(r)) return;
		lastSection = r.e.section;
		this.o.onpick(r.e);
		if (!this.o.multiple()) this.o.onclose();
	}

	/** Why a row can't be picked, as its title. */
	why(r: Row): string | undefined {
		const type = componentName(this.o.schema(), r.e.type);
		if (!r.pickable) return t('entries.cantChoose', { type });
		if (this.isChosen(r) && !this.o.multiple())
			return t('entries.alreadyChosen');
		if (!this.isChosen(r) && this.full) return t('entries.fieldFull');
	}

	toggle(id: number) {
		const next = new Set(this.closed);
		if (!next.delete(id)) next.add(id);
		this.closed = next;
	}
}
