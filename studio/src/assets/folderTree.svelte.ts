import { untrack } from 'svelte';
import { createFolder, deleteFolder, updateFolder, type Folder } from './api';
import { folderRows } from './folders';
import { visibleRows } from '../core/tree';
import { t } from '../i18n/index.svelte';

/**
 * The state of FolderTree: which folders are open, the one added or
 * renamed, and deleting one.
 */
export class FolderTreeState {
	open = $state(new Set<number>());
	/** A new folder in `parent`, or a folder being renamed. */
	editing = $state<{ parent: number | null } | { id: number } | null>(null);
	error = $state('');
	readonly all = $derived.by(() => folderRows(this.o.folders()));
	/** The rows whose parents are all open. */
	readonly rows = $derived(visibleRows(this.all, id => this.open.has(id)));
	private openedFor: number | null = null;

	constructor(
		private o: {
			folders: () => Folder[];
			/** the folder shown, null: the top */
			current: () => number | null;
			onselect: (id: number | null) => void;
			/** after a folder was added, renamed or deleted */
			onchange?: () => void;
		},
	) {}

	/**
	 * Opens the folders around the current one, also when the folders
	 * come after the tree (the picker loads them) or another one opens;
	 * for an effect.
	 */
	reveal() {
		const id = this.o.current();
		if (id === null || id === this.openedFor) return;
		const row = this.all.find(r => r.folder.id === id);
		if (!row) return;
		this.openedFor = id;
		untrack(() => (this.open = new Set([...this.open, ...row.parents])));
	}

	hasChildren(id: number): boolean {
		return this.o.folders().some(f => f.parentId === id);
	}

	toggle(id: number) {
		const next = new Set(this.open);
		if (!next.delete(id)) next.add(id);
		this.open = next;
	}

	/** Opens a folder, it shows its subfolders too. */
	select(id: number) {
		this.open = new Set(this.open).add(id);
		this.o.onselect(id);
	}

	startNew(parent: number | null) {
		if (parent !== null) this.open = new Set(this.open).add(parent);
		this.editing = { parent };
		this.error = '';
	}

	startRename(f: Folder) {
		this.editing = { id: f.id };
		this.error = '';
	}

	/** The name of the folder being renamed, empty for a new one. */
	readonly editedName = $derived.by(() => {
		const e = this.editing;
		return e && 'id' in e
			? (this.o.folders().find(f => f.id === e.id)?.name ?? '')
			: '';
	});

	/** Adds or renames; whether it worked, else the error shows. */
	async save(name: string): Promise<boolean> {
		const e = this.editing;
		if (!e) return true;
		try {
			if ('id' in e) {
				const f = this.o.folders().find(f => f.id === e.id)!;
				await updateFolder(f.id, f.parentId, name);
			} else {
				await createFolder(e.parent, name);
			}
			this.editing = null;
			this.o.onchange?.();
			return true;
		} catch (err) {
			this.error = (err as Error).message;
			return false;
		}
	}

	/** Deletes an empty folder, after asking. */
	async remove(f: Folder) {
		if (!confirm(t('assets.confirmDeleteFolder', { name: f.name }))) return;
		this.error = '';
		try {
			await deleteFolder(f.id);
			if (this.o.current() === f.id) this.o.onselect(f.parentId);
			this.o.onchange?.();
		} catch (err) {
			this.error = (err as Error).message;
		}
	}
}
