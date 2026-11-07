import { Busy } from '../../core/busy.svelte';
import {
	componentOf,
	defaults,
	hasPages,
	sameKind,
	sectionOf,
	type Schema,
	type Values,
} from '../../core/schema';
import {
	deleteEntry,
	getEntry,
	listEntries,
	createEntry,
	setAuthor,
	type Author,
	type Dates,
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
	restoreRevision,
	type Draft,
	type Revision,
	type SiteDoc,
} from '../drafts';
import { site } from '../../core/router.svelte';
import { DraftSaver, savesDone } from './autosave.svelte';
import { docChanges } from './diff';
import { canEntry } from '../../core/access';
import { session } from '../../core/session.svelte';
import { listUsers, type Account } from '../../users/api';
import { clean } from '../../core/values';
import { slugify } from '../../core/format';
import { depths } from '../../core/tree';
import { t } from '../../i18n/index.svelte';

/** A value that is no content: empty, or the type's default. */
function isEmpty(v: unknown, def: unknown): boolean {
	const c = JSON.stringify(clean(v));
	return c === 'null' || c === JSON.stringify(clean(def));
}

/** What the role may do with the entry; the server checks it too. */
export type May = { edit: boolean; publish: boolean; delete: boolean };

/**
 * The state of the entry editor and what it does: loading, saving drafts
 * while the editor types, publishing, conflicts with another editor,
 * revisions. The page renders it, Edit.svelte.
 */
export class EntryEditor {
	sec = $state('');
	ty = $state('');
	#title = $state('');
	#slug = $state('');
	/**
	 * The slug comes from the title as it is typed, like Craft's: while
	 * the entry was never published and the slug is the title's (or none);
	 * one written by hand stays.
	 */
	slugFollows = $state(true);

	get title() {
		return this.#title;
	}

	set title(t: string) {
		this.#title = t;
		if (this.slugFollows) this.#slug = slugify(t);
	}

	get slug() {
		return this.#slug;
	}

	/** A slug written by hand; emptied, it follows the title again. */
	set slug(s: string) {
		this.#slug = s;
		this.slugFollows = !s || s === slugify(this.#title);
	}
	uri = $state<string | null>(null);
	published = $state(false);
	/** the same in every site */
	dates = $state<Dates>({ postDate: null, expiryDate: null });
	parentId = $state<number | null>(null);
	/** who made it, a new one is the user's */
	authorId = $state<number | null>(session.user?.id ?? null);
	author = $state<Author | null>(null);
	/** whom an admin may make the author */
	users = $state<Account[]>([]);
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
	readonly saver = new DraftSaver(this);
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

	/** the values of each type the editor had, see `changeType` */
	#byType: Record<string, Values> = {};
	/** a new entry as it opened, see `blank` */
	#opened: SiteDoc | null = null;

	/** the site it edits, the one of the start: the site may switch */
	readonly site: string;

	/** the other sites a saved entry can copy its content from */
	get copySites() {
		if (this.id === null || !this.may.edit) return [];
		return this.schema.sites.filter(s => s.handle !== this.site);
	}

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

	/** the page shows once both are loaded */
	secDef = $derived.by(() => sectionOf(this.schema, this.sec));
	comp = $derived.by(() => componentOf(this.schema, this.ty));
	single = $derived(this.secDef?.kind === 'single');
	/** a single without a page, its title is the section's name */
	global = $derived(
		this.single && this.secDef !== undefined && !hasPages(this.secDef),
	);
	/** what the role may do here, the server checks it too */
	may: May = $derived.by(() => ({
		edit: canEntry(this.schema, this.sec, this.authorId, 'edit'),
		publish: canEntry(this.schema, this.sec, this.authorId, 'publish'),
		delete:
			this.id !== null &&
			!this.single &&
			canEntry(this.schema, this.sec, this.authorId, 'delete'),
	}));

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
		await savesDone();
		if (id !== null) {
			await ed.load();
			return ed;
		}
		const sec = sectionOf(schema, ed.sec);
		ed.ty = sec?.types[0] ?? '';
		ed.fields = defaults(schema, ed.ty);
		ed.#opened = ed.doc;
		ed.saver.saved();
		if (sec?.kind === 'structure') await ed.loadParents();
		return ed;
	}

	/** What the draft changes against the live entry, also in blocks. */
	changed = $derived.by(() => {
		if (!this.draft || !this.live) return new Set<string>();
		return docChanges(this.live, this.doc);
	});

	/** The content of the site as a draft keeps it. */
	get doc(): SiteDoc {
		return {
			type: this.ty,
			title: this.title,
			slug: this.slug,
			published: this.published,
			fields: $state.snapshot(this.fields),
			...$state.snapshot(this.dates),
		};
	}

	/** Shows a content of the site: the live one or a draft's. */
	private setDoc(d: SiteDoc) {
		// a draft may change the type
		this.ty = d.type;
		this.#title = d.title;
		this.#slug = d.slug;
		this.published = d.published;
		this.dates = {
			postDate: d.postDate ?? null,
			expiryDate: d.expiryDate ?? null,
		};
		this.fields = d.fields;
		// another content: the values of other types are not its own
		this.#byType = {};
	}

	async load() {
		const id = this.id;
		if (id === null) return;
		const [{ entry, relations, author }, { draft: d, relations: drafted }] =
			await Promise.all([
				getEntry(id, this.site),
				getDraftWithRefs(id, this.site),
			]);
		this.sec = entry.section;
		this.uri = entry.uri;
		this.parentId = entry.parentId;
		this.authorId = entry.authorId;
		this.author = author;
		this.refs = { ...relations, ...drafted };
		this.draft = d;
		this.live = {
			type: entry.type,
			title: entry.title,
			slug: entry.slug,
			published: entry.published,
			fields: entry.fields,
			postDate: entry.postDate,
			expiryDate: entry.expiryDate,
		};
		const content = d?.doc[this.site] ?? entry;
		this.setDoc(content);
		// a live entry's url stays when its title changes
		this.slugFollows =
			entry.postDate === null &&
			(!content.slug || content.slug === slugify(content.title));
		this.saver.saved();
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

	/** A new entry of a structure: where it can go, within its max level. */
	async loadParents() {
		const r = await listEntries({
			section: this.sec,
			site: this.site,
			order: 'structure',
		});
		const max = sectionOf(this.schema, this.sec)?.max_level;
		const d = depths(r.entries);
		// a child of the top (depth 0) is on level 2
		this.parents = r.entries.filter((_, i) => !max || d[i] + 2 <= max);
	}

	/**
	 * The content as publishing keeps it: an empty group that a field
	 * fills in on load is no change.
	 */
	snapshot(): string {
		const doc = this.doc;
		return JSON.stringify({ ...doc, fields: clean(doc.fields) });
	}

	/**
	 * Another type of the section, like Craft's: a field with the same
	 * handle and kind keeps its content, the others go, without asking.
	 * The draft converts the other sites the same way; the live entry and
	 * the revisions keep the old type, discarding or restoring brings it
	 * back. While the editor is open each type keeps its values: back to
	 * a type, they are there again.
	 */
	changeType(to: string) {
		const from = componentOf(this.schema, this.ty);
		const into = componentOf(this.schema, to);
		if (!into || to === this.ty) return;
		const kept = (handle: string) => {
			const a = from?.fields.find(f => f.handle === handle);
			const b = into.fields.find(f => f.handle === handle);
			return !!a && !!b && sameKind(a, b);
		};
		const fields = $state.snapshot(this.fields) as Values;
		this.#byType[this.ty] = fields;

		// a default of the new type replaces an empty value or the old
		// type's default
		const before = defaults(this.schema, this.ty);
		const after = defaults(this.schema, to);
		const taken = Object.entries(fields).filter(
			([h, v]) => kept(h) && !(h in after && isEmpty(v, before[h])),
		);
		const theirs = Object.entries(this.#byType[to] ?? {}).filter(
			([h]) => !kept(h),
		);
		this.fields = {
			...after,
			...Object.fromEntries(theirs),
			...Object.fromEntries(taken),
		};
		this.ty = to;
	}

	/**
	 * A new entry with nothing but its type's defaults: leaving it loses
	 * nothing.
	 */
	get blank(): boolean {
		if (!this.#opened) return false;
		const empty = {
			...this.#opened,
			type: this.ty,
			fields: defaults(this.schema, this.ty),
		};
		return docChanges(empty, this.doc).size === 0;
	}

	/**
	 * A new entry is created directly, its id comes back (`unpublished`:
	 * not live yet whatever its switch says, for the live preview); a
	 * draft is published, the last keys first.
	 */
	async save(unpublished = false): Promise<number | null> {
		// Cmd+S while saving: once is enough, a new entry only once
		if (this.busy) return null;
		const created = await this.action.run(async () => {
			if (this.id === null) {
				const entry = await createEntry({
					section: this.sec,
					site: this.site,
					parentId: this.parentId,
					...this.doc,
					...(unpublished ? { published: false } : {}),
				});
				// saved, the page of the entry may open
				this.saver.saved();
				return entry.id;
			}
			// what isn't saved can't be published: the conflict or error shows
			if (!(await this.saver.flush())) return null;
			if (!this.draft) return null;
			await publishDraft(this.id);
			this.saver.state = '';
			await this.load();
			return null;
		});
		return created ?? null;
	}

	discard() {
		this.saver.cancel();
		return this.action.run(async () => {
			await discardDraft(this.id!);
			this.saver.state = '';
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
			// the slug stays, see above
			this.#title = theirs.title;
			this.fields = theirs.fields;
		});
	}

	/** The users an admin may make the author, loaded once. */
	loadUsers() {
		if (this.users.length) return;
		listUsers().then(
			users => (this.users = users),
			e => (this.action.error = e.message),
		);
	}

	/** Another author, at once: it is no change of the content. */
	changeAuthor(id: number | null) {
		return this.action.run(async () => {
			await setAuthor(this.id!, id);
			this.authorId = id;
			const user = this.users.find(u => u.id === id);
			this.author = user ? { id: user.id, name: user.name } : null;
		});
	}

	/** Throws away the own changes and shows the ones of the other. */
	async loadTheirs() {
		this.saver.cancel();
		await this.action.run(() => this.load());
		// only now, else the own state would be saved again
		this.conflict = '';
	}

	/** Saves the own changes over the ones of the other. */
	keepMine() {
		this.conflict = '';
		return this.action.run(async () => {
			this.draft = await getDraft(this.id!);
			this.saver.forget();
			await this.saver.flush();
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
		const changed = now && old ? docChanges(old, now) : new Set<string>();
		this.viewing = { rev, doc: now, changed };
	}

	/** Replaces the draft, after asking: what isn't published is lost. */
	restore(rev: Revision) {
		const question = this.draft
			? t('entries.confirmRestoreDraft')
			: t('entries.confirmRestore');
		if (!confirm(question)) return;
		this.saver.cancel();
		return this.action.run(async () => {
			await restoreRevision(
				this.id!,
				rev.id,
				this.draft?.updatedAt ?? null,
			);
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
			// the website can't navigate the studio's tab
			if (tab) {
				tab.opener = null;
				tab.location.href = page;
			} else window.open(page, '_blank', 'noopener');
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
