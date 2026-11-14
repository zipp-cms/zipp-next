import { ApiError } from '../../core/client';
import { guardUnload } from '../../core/leave';
import { createEntry } from '../api';
import { putDraft } from '../drafts';
import type { EntryEditor } from './entryEditor.svelte';

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

/** Waits for the saves of the editors that were open before. */
export function savesDone(): Promise<unknown> {
	return Promise.allSettled([...unsaved]);
}

/** Whether a draft still waits for its save, the tab asks before closing. */
let waiting = 0;
guardUnload(() => waiting > 0 || unsaved.size > 0);

/**
 * Saves the draft of an editor while it types: a moment after the last
 * key, one save after the other, not over a conflict. A new entry is
 * created with its first change, like Craft's, not live whatever its
 * switch says; from then on its changes are a draft like any entry's.
 */
export class DraftSaver {
	/** `failed`: the last save didn't work, the error says why */
	state = $state<'' | 'saving' | 'saved' | 'failed'>('');

	private lastSaved = '';
	private timer: ReturnType<typeof setTimeout> | undefined;
	/** the save running now, the next one waits for it */
	private saving: Promise<boolean> | null = null;

	constructor(private readonly ed: EntryEditor) {}

	/** What the editor shows now is saved: loaded or created. */
	saved() {
		this.lastSaved = this.ed.snapshot();
	}

	/** The next flush saves, also what looks saved: keeping mine. */
	forget() {
		this.lastSaved = '';
	}

	/**
	 * A new entry with content that isn't created yet: leaving or closing
	 * the tab would lose it. Its type's defaults are no content.
	 */
	get unsaved(): boolean {
		return this.ed.id === null && !this.ed.blank;
	}

	/** Called on every change: the draft is saved a moment later. */
	changed(canEdit: boolean) {
		const ed = this.ed;
		const snap = ed.snapshot();
		if (!ed.loaded || ed.conflict || !canEdit) return;
		if (snap === this.lastSaved || (ed.id === null && ed.blank))
			return this.cancel();
		this.stopTimer();
		this.state = 'saving';
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
			if (this.ed.conflict) return false;
			if (this.ed.id === null && !(await this.create())) return false;
			const snap = this.ed.snapshot();
			return snap === this.lastSaved || this.save(snap);
		};
		const p = run();
		this.saving = p;
		track(p);
		p.finally(() => {
			if (this.saving === p) this.saving = null;
		});
		return p;
	}

	/** After a new login: the save that failed goes again. */
	retry() {
		if (this.state !== 'failed') return;
		this.ed.error = '';
		this.flush();
	}

	/** No save of what was typed before: discarding, restoring, theirs. */
	cancel() {
		this.stopTimer();
		this.state = '';
	}

	private stopTimer() {
		if (this.timer === undefined) return;
		clearTimeout(this.timer);
		this.timer = undefined;
		waiting--;
	}

	/**
	 * Creates the new entry, not live, so nothing typed is lost: required
	 * fields and the title wait for publishing. What the editor has
	 * besides (its switch, a slug that was taken, what was typed meanwhile)
	 * is the draft that the flush saves next; whether it worked.
	 */
	private async create(): Promise<boolean> {
		const ed = this.ed;
		try {
			const entry = await createEntry({
				section: ed.sec,
				site: ed.site,
				parentId: ed.parentId,
				...ed.doc,
				// one of the title the server makes free (`-2`, …)
				...(ed.slugFollows ? { slug: '' } : {}),
				published: false,
			});
			await ed.created(entry);
			this.state = 'saved';
			return true;
		} catch (e: any) {
			ed.error = e.message;
			if (e instanceof ApiError)
				for (const f of e.fields) ed.errors[f.path] = f.message;
			this.state = 'failed';
			return false;
		}
	}

	/** Saves the draft; whether it worked, a conflict or error shows. */
	private async save(snap: string): Promise<boolean> {
		const ed = this.ed;
		if (ed.id === null) return false;
		try {
			// null: the change was undone, no draft and nothing to publish
			ed.draft = await putDraft(
				ed.id,
				ed.site,
				ed.doc,
				ed.draft?.updatedAt ?? null,
			);
			this.lastSaved = snap;
			// a newer change may wait for its save
			if (snap === ed.snapshot()) this.state = 'saved';
			return true;
		} catch (e: any) {
			if (e instanceof ApiError && e.type === 'CONFLICT') {
				ed.conflict = e.message;
				this.state = '';
			} else {
				ed.error = e.message;
				this.state = 'failed';
			}
			return false;
		}
	}
}
