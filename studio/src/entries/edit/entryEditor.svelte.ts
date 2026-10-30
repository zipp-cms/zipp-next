import { ApiError } from '../../core/client';
import { Busy } from '../../core/busy.svelte';
import { type Schema, type Values } from '../../core/schema';
import {
	deleteEntry,
	getEntry,
	listEntries,
	saveEntry,
	type Entry,
	type Refs,
} from '../api';
import {
	discardDraft,
	getDraft,
	getDraftWithRefs,
	getRevision,
	getRevisions,
	previewToken,
	publishDraft,
	putDraft,
	restoreRevision,
	type Draft,
	type Revision,
	type SiteDoc,
} from '../drafts';
import { site } from '../../core/router.svelte';
import { changedPaths, clean } from './diff';

/** What the role may do with the entry; the server checks it too. */
export type May = { edit: boolean; publish: boolean; delete: boolean };

/** How long after the last key a draft is saved. */
const SAVE_AFTER_MS = 700;

/**
 * The saves of all editors that didn't end yet: an editor that opens
 * waits for them, so it loads what an editor just left behind (leaving,
 * switching the site, the flyout) and doesn't save over it.
 */
const unsaved = new Set<Promise<unknown>>();

function track(p: Promise<unknown>) {
	unsaved.add(p);
	p.finally(() => unsaved.delete(p));
}

/** Whether a draft still waits for its save, the tab asks before closing. */
let waiting = 0;
if (typeof window !== 'undefined') {
	window.addEventListener('beforeunload', e => {
		if (waiting || unsaved.size) e.preventDefault();
	});
}

/**
 * The state of the entry editor and what it does: loading, saving drafts
 * while the editor types, publishing, conflicts with another editor,
 * revisions. The page renders it, Edit.svelte.
 */
export class EntryEditor {
	sec = $state('');
	ty = $state('');
	title = $state('');
	slug = $state('');
	uri = $state<string | null>(null);
	published = $state(false);
	parentId = $state<number | null>(null);
	fields = $state<Values>({});
	refs = $state<Refs>({});
	loaded = $state(false);
	/** loading, saving, publishing…: one at a time */
	readonly action = new Busy();
	/** whether the entry is live in each site, as saved */
	perSite = $state<Record<string, boolean>>({});
	/** possible parents of a new entry in a structure */
	parents = $state<Entry[]>([]);
	/** unpublished changes, saved while typing */
	draft = $state<Draft | null>(null);
	/** `failed`: the last save didn't work, the error says why */
	draftState = $state<'' | 'saving' | 'saved' | 'failed'>('');
	revisions = $state<Revision[]>([]);
	/** the live content of the site, to mark what the draft changes */
	live = $state<SiteDoc | null>(null);
	/** an old revision, shown read only */
	viewing = $state<{
		rev: Revision;
		doc: SiteDoc | null;
		changed: Set<string>;
	} | null>(null);
	/** someone else changed the draft, nothing is saved until resolved */
	conflict = $state('');

	private lastSaved = '';
	private timer: ReturnType<typeof setTimeout> | undefined;
	/** the save running now, the next one waits for it */
	private saving: Promise<boolean> | null = null;

	/** the site it edits, the one of the start: the site may switch */
	readonly site: string;

	get busy() {
		return this.action.busy;
	}

	get error() {
		return this.action.error;
	}

	set error(message: string) {
		this.action.error = message;
	}

	/** of the fields by their path */
	get errors() {
		return this.action.fields;
	}

	constructor(
		readonly schema: Schema,
		readonly id: number | null,
		section: string | null,
	) {
		this.site = site.current;
		this.sec = section ?? '';
		this.loaded = id === null;
	}

	/**
	 * The editor of an entry, loaded; a new one (`id` null) of the
	 * section with its first type and, in a structure, its parents.
	 */
	static async open(
		schema: Schema,
		id: number | null,
		section: string | null,
	): Promise<EntryEditor> {
		const ed = new EntryEditor(schema, id, section);
		await Promise.allSettled([...unsaved]);
		if (id !== null) {
			await ed.load();
			return ed;
		}
		const sec = schema.sections.find(s => s.handle === section);
		ed.ty = sec?.types[0] ?? '';
		if (sec?.kind === 'structure') await ed.loadParents();
		return ed;
	}

	/** What the draft changes against the live entry, also in blocks. */
	changed = $derived.by(() => {
		const live = this.live;
		if (!this.draft || !live) return new Set<string>();
		const set = changedPaths(live.fields, $state.snapshot(this.fields));
		if (this.title !== live.title) set.add('title');
		if (this.slug !== live.slug) set.add('slug');
		if (this.published !== live.published) set.add('published');
		return set;
	});

	async load() {
		const id = this.id;
		if (id === null) return;
		const [{ entry, relations }, { draft: d, relations: drafted }] =
			await Promise.all([
				getEntry(id, this.site),
				getDraftWithRefs(id, this.site),
			]);
		this.sec = entry.section;
		this.ty = entry.type;
		this.uri = entry.uri;
		this.parentId = entry.parentId;
		this.refs = { ...relations, ...drafted };
		this.draft = d;
		this.live = {
			title: entry.title,
			slug: entry.slug,
			published: entry.published,
			fields: entry.fields,
		};
		const content = d?.doc[this.site] ?? entry;
		this.title = content.title;
		this.slug = content.slug;
		this.published = content.published;
		this.fields = content.fields;
		this.lastSaved = this.snapshot();
		this.loaded = true;
		const [revs, list] = await Promise.all([
			getRevisions(id),
			listEntries({
				section: this.sec,
				ids: String(id),
				site: this.site,
			}),
		]);
		this.revisions = revs;
		this.perSite = list.published[id] ?? {};
	}

	/** A new entry of a structure: where it can go. */
	async loadParents() {
		const r = await listEntries({
			section: this.sec,
			site: this.site,
			order: 'structure',
		});
		this.parents = r.entries;
	}

	/**
	 * The content as publishing keeps it: an empty group that a field
	 * fills in on load is no change.
	 */
	snapshot(): string {
		return JSON.stringify([
			this.title,
			this.slug,
			this.published,
			clean($state.snapshot(this.fields)),
		]);
	}

	/** Called on every change: the draft is saved a moment later. */
	changedNow(canEdit: boolean) {
		const snap = this.snapshot();
		if (!this.loaded || this.id === null || this.conflict || !canEdit)
			return;
		if (snap === this.lastSaved) return this.cancelSave();
		this.stopTimer();
		this.draftState = 'saving';
		waiting++;
		this.timer = setTimeout(() => {
			this.timer = undefined;
			waiting--;
			this.flush();
		}, SAVE_AFTER_MS);
	}

	/**
	 * Saves what isn't saved yet, now, after the save that runs; whether
	 * all is saved. Leaving the editor calls it, the next editor waits.
	 */
	flush(): Promise<boolean> {
		this.stopTimer();
		const before = this.saving;
		const run = async () => {
			if (before) await before;
			if (this.id === null || this.conflict) return false;
			const snap = this.snapshot();
			return snap === this.lastSaved || this.saveDraft(snap);
		};
		const p = run();
		this.saving = p;
		track(p);
		p.finally(() => {
			if (this.saving === p) this.saving = null;
		});
		return p;
	}

	private stopTimer() {
		if (this.timer === undefined) return;
		clearTimeout(this.timer);
		this.timer = undefined;
		waiting--;
	}

	/** Saves the draft; whether it worked, a conflict or error shows. */
	private async saveDraft(snap: string): Promise<boolean> {
		if (this.id === null) return false;
		const content = {
			title: this.title,
			slug: this.slug,
			published: this.published,
			fields: $state.snapshot(this.fields),
		};
		try {
			this.draft = await putDraft(
				this.id,
				this.site,
				content,
				this.draft?.updatedAt ?? null,
			);
			this.lastSaved = snap;
			// a newer change may wait for its save
			if (snap === this.snapshot()) this.draftState = 'saved';
			return true;
		} catch (e: any) {
			if (e instanceof ApiError && e.type === 'CONFLICT') {
				this.conflict = e.message;
				this.draftState = '';
			} else {
				this.error = e.message;
				this.draftState = 'failed';
			}
			return false;
		}
	}

	/** No save of what was typed before: discarding, restoring, theirs. */
	private cancelSave() {
		this.stopTimer();
		this.draftState = '';
	}

	/**
	 * A new entry is created directly, its id comes back; a draft is
	 * published, the last keys first.
	 */
	async save(): Promise<number | null> {
		// Cmd+S while saving: once is enough, a new entry only once
		if (this.busy) return null;
		const created = await this.action.run(async () => {
			if (this.id === null) {
				const entry = await saveEntry(null, {
					section: this.sec,
					type: this.ty,
					site: this.site,
					parentId: this.parentId,
					title: this.title,
					slug: this.slug,
					published: this.published,
					fields: $state.snapshot(this.fields),
				});
				return entry.id;
			}
			// what isn't saved can't be published: the conflict or error shows
			if (!(await this.flush())) return null;
			if (!this.draft) return null;
			await publishDraft(this.id);
			this.draftState = '';
			await this.load();
			return null;
		});
		return created ?? null;
	}

	discard() {
		this.cancelSave();
		return this.action.run(async () => {
			await discardDraft(this.id!);
			this.draftState = '';
			await this.load();
		});
	}

	/**
	 * Takes title and fields of another site into the draft of this one,
	 * e.g. to start a translation from the German text. The slug stays.
	 */
	copyFrom(other: string) {
		const id = this.id!;
		return this.action.run(async () => {
			// the other site as the editor sees it: its draft, or live
			// the chips in this site's language
			const { draft: d, relations } = await getDraftWithRefs(
				id,
				this.site,
			);
			let theirs = d?.doc[other];
			if (!theirs) {
				const live = await getEntry(id, other);
				theirs = live.entry;
				Object.assign(this.refs, live.relations);
			}
			Object.assign(this.refs, relations);
			this.title = theirs.title;
			this.fields = theirs.fields;
		});
	}

	/** Throws away the own changes and shows the ones of the other. */
	async loadTheirs() {
		this.cancelSave();
		await this.action.run(() => this.load());
		// only now, else the own state would be saved again
		this.conflict = '';
	}

	/** Saves the own changes over the ones of the other. */
	keepMine() {
		this.conflict = '';
		return this.action.run(async () => {
			this.draft = await getDraft(this.id!);
			this.lastSaved = '';
			await this.flush();
		});
	}

	/** Shows a revision with what it changed against the one before. */
	view(rev: Revision) {
		return this.action.run(() => this.loadRevision(rev));
	}

	private async loadRevision(rev: Revision) {
		const id = this.id!;
		const before = this.revisions[this.revisions.indexOf(rev) + 1];
		const [doc, prev] = await Promise.all([
			getRevision(id, rev.id),
			before ? getRevision(id, before.id) : null,
		]);
		const now = doc[this.site] ?? null;
		const old = prev?.[this.site] ?? null;
		const set = new Set<string>();
		if (now && old) {
			changedPaths(old.fields, now.fields, '', set);
			if (now.title !== old.title) set.add('title');
			if (now.slug !== old.slug) set.add('slug');
		}
		this.viewing = { rev, doc: now, changed: set };
	}

	/** Replaces the draft, after asking: what isn't published is lost. */
	restore(rev: Revision) {
		const what = this.draft ? 'The draft is replaced. ' : '';
		if (!confirm(`Restore this revision? ${what}`.trim())) return;
		this.cancelSave();
		return this.action.run(async () => {
			await restoreRevision(this.id!, rev.id);
			this.viewing = null;
			await this.load();
		});
	}

	/** The website with the draft, in a new tab. */
	async preview(url: string) {
		// the tab opens with the click: one opened later is a blocked popup
		const tab = window.open('', '_blank');
		try {
			const token = await previewToken(this.id!);
			const page = `${url}?token=${token}`;
			if (tab) tab.location.href = page;
			else window.open(page, '_blank');
		} catch (e: any) {
			tab?.close();
			this.error = e.message;
		}
	}

	/** Deletes the entry in every site; whether it is gone. */
	async remove(): Promise<boolean> {
		const gone = await this.action.run(async () => {
			await deleteEntry(this.id!);
			return true;
		});
		return gone ?? false;
	}
}
