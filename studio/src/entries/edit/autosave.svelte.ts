import { ApiError } from '../../core/client';
import { guardUnload } from '../../core/leave';
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
 * key, one save after the other, not over a conflict.
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
	 * A new entry with content: it has no draft, leaving or closing the
	 * tab would lose it.
	 */
	get unsaved(): boolean {
		return this.ed.id === null && this.ed.snapshot() !== this.lastSaved;
	}

	/** Called on every change: the draft is saved a moment later. */
	changed(canEdit: boolean) {
		const ed = this.ed;
		const snap = ed.snapshot();
		if (!ed.loaded || ed.id === null || ed.conflict || !canEdit) return;
		if (snap === this.lastSaved) return this.cancel();
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
			if (this.ed.id === null || this.ed.conflict) return false;
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

	/** Saves the draft; whether it worked, a conflict or error shows. */
	private async save(snap: string): Promise<boolean> {
		const ed = this.ed;
		if (ed.id === null) return false;
		try {
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
