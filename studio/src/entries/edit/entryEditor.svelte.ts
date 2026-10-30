import { ApiError } from '../../core/client';
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
import { site } from '../../core/route.svelte';
import { changedPaths, clean } from './diff';

/** How long after the last key a draft is saved. */
const SAVE_AFTER_MS = 700;

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
	busy = $state(false);
	error = $state('');
	errors = $state<Record<string, string>>({});
	/** whether the entry is live in each site, as saved */
	perSite = $state<Record<string, boolean>>({});
	/** possible parents of a new entry in a structure */
	parents = $state<Entry[]>([]);
	/** unpublished changes, saved while typing */
	draft = $state<Draft | null>(null);
	draftState = $state<'' | 'saving' | 'saved'>('');
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

	constructor(
		readonly schema: Schema,
		readonly id: number | null,
		section: string | null,
	) {
		this.sec = section ?? '';
		this.loaded = id === null;
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
		const [{ entry, relations }, d] = await Promise.all([
			getEntry(id, site.current),
			getDraft(id),
		]);
		this.sec = entry.section;
		this.ty = entry.type;
		this.uri = entry.uri;
		this.parentId = entry.parentId;
		this.refs = relations;
		this.draft = d;
		this.live = {
			title: entry.title,
			slug: entry.slug,
			published: entry.published,
			fields: entry.fields,
		};
		const content = d?.doc[site.current] ?? entry;
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
				site: site.current,
			}),
		]);
		this.revisions = revs;
		this.perSite = list.published[id] ?? {};
	}

	/** A new entry of a structure: where it can go. */
	async loadParents() {
		const r = await listEntries({
			section: this.sec,
			site: site.current,
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
		if (snap === this.lastSaved) return;
		clearTimeout(this.timer);
		this.draftState = 'saving';
		this.timer = setTimeout(() => this.saveDraft(snap), SAVE_AFTER_MS);
	}

	private async saveDraft(snap: string) {
		if (this.id === null) return;
		const content = {
			title: this.title,
			slug: this.slug,
			published: this.published,
			fields: $state.snapshot(this.fields),
		};
		try {
			this.draft = await putDraft(
				this.id,
				site.current,
				content,
				this.draft?.updatedAt ?? null,
			);
			this.lastSaved = snap;
			this.draftState = 'saved';
		} catch (e: any) {
			if (e instanceof ApiError && e.status === 409) {
				this.conflict = e.message;
				this.draftState = '';
			} else {
				this.error = e.message;
			}
		}
	}

	/** Runs an action: busy meanwhile, its errors on the fields. */
	private async run(f: () => Promise<unknown>) {
		this.busy = true;
		this.error = '';
		this.errors = {};
		try {
			await f();
		} catch (e: any) {
			this.error = e.message;
			if (e instanceof ApiError) {
				for (const f of e.fields) this.errors[f.path] = f.message;
			}
		} finally {
			this.busy = false;
		}
	}

	/**
	 * A new entry is created directly, its id comes back; a draft is
	 * published, the last keys first.
	 */
	save(): Promise<number | null> {
		let created: number | null = null;
		return this.run(async () => {
			if (this.id === null) {
				const entry = await saveEntry(null, {
					section: this.sec,
					type: this.ty,
					site: site.current,
					parentId: this.parentId,
					title: this.title,
					slug: this.slug,
					published: this.published,
					fields: $state.snapshot(this.fields),
				});
				created = entry.id;
				return;
			}
			clearTimeout(this.timer);
			const snap = this.snapshot();
			if (snap !== this.lastSaved) await this.saveDraft(snap);
			if (!this.draft) return;
			await publishDraft(this.id);
			this.draftState = '';
			await this.load();
		}).then(() => created);
	}

	discard() {
		return this.run(async () => {
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
		return this.run(async () => {
			// the other site as the editor sees it: its draft, or live
			const d = await getDraft(id);
			const theirs = d?.doc[other] ?? (await getEntry(id, other)).entry;
			this.title = theirs.title;
			this.fields = theirs.fields;
		});
	}

	/** Throws away the own changes and shows the ones of the other. */
	async loadTheirs() {
		clearTimeout(this.timer);
		this.draftState = '';
		await this.run(() => this.load());
		// only now, else the own state would be saved again
		this.conflict = '';
	}

	/** Saves the own changes over the ones of the other. */
	keepMine() {
		this.conflict = '';
		return this.run(async () => {
			this.draft = await getDraft(this.id!);
			await this.saveDraft(this.snapshot());
		});
	}

	/** Shows a revision with what it changed against the one before. */
	async view(rev: Revision) {
		const id = this.id!;
		const before = this.revisions[this.revisions.indexOf(rev) + 1];
		const [doc, prev] = await Promise.all([
			getRevision(id, rev.id),
			before ? getRevision(id, before.id) : null,
		]);
		const now = doc[site.current] ?? null;
		const old = prev?.[site.current] ?? null;
		const set = new Set<string>();
		if (now && old) {
			changedPaths(old.fields, now.fields, '', set);
			if (now.title !== old.title) set.add('title');
			if (now.slug !== old.slug) set.add('slug');
		}
		this.viewing = { rev, doc: now, changed: set };
	}

	restore(rev: Revision) {
		return this.run(async () => {
			await restoreRevision(this.id!, rev.id);
			this.viewing = null;
			await this.load();
		});
	}

	/** The website with the draft, in a new tab. */
	async preview(url: string) {
		const token = await previewToken(this.id!);
		window.open(`${url}?token=${token}`, '_blank');
	}

	/** Deletes the entry in every site; whether it is gone. */
	async remove(): Promise<boolean> {
		let gone = false;
		await this.run(async () => {
			await deleteEntry(this.id!);
			gone = true;
		});
		return gone;
	}
}
