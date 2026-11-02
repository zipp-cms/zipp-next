import { t } from '../i18n/index.svelte';
import type { Folder } from './api';

/** The folders of the asset library as a tree, found by name or path. */

/** The name of the top, where all folders are and the assets in none. */
export function root(): string {
	return t('assets.library');
}

/** The folder of a path like `Photos/Events`, if it exists. */
export function folderByPath(
	folders: Folder[],
	path: string | undefined,
): Folder | undefined {
	let parent: number | null = null;
	let found: Folder | undefined;
	for (const name of (path ?? '').split('/').filter(n => n.trim())) {
		found = childFolder(folders, parent, name);
		if (!found) return undefined;
		parent = found.id;
	}
	return found;
}

/**
 * The folder of a name in `parent` (null: the top); the server trims
 * names and compares them without case.
 */
export function childFolder(
	folders: Folder[],
	parent: number | null,
	name: string,
): Folder | undefined {
	const want = name.trim().toLowerCase();
	return folders.find(
		f => f.parentId === parent && f.name.toLowerCase() === want,
	);
}

export type FolderRow = {
	folder: Folder;
	depth: number;
	/** the names from the top, `Photos / Events` */
	path: string;
	/** the ids of the folders above it, the top first */
	parents: number[];
};

/** The folders in the order of the tree, with where each is. */
export function folderRows(folders: Folder[]): FolderRow[] {
	const rows: FolderRow[] = [];
	const add = (parent: number | null, parents: number[], path: string) => {
		for (const f of folders.filter(f => f.parentId === parent)) {
			const here = path ? `${path} / ${f.name}` : f.name;
			rows.push({
				folder: f,
				depth: parents.length,
				path: here,
				parents,
			});
			add(f.id, [...parents, f.id], here);
		}
	};
	add(null, [], '');
	return rows;
}
