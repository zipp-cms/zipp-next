import { t } from '../i18n/index.svelte';
import type { Folder } from './api';
import type { Editing } from '../core/editing';
import { pathDirs } from './files';

/** The folders of the asset library as a tree, found by name or path. */

/** The name of the top, where all folders are and the assets in none. */
export function root(): string {
	return t('assets.library');
}

/**
 * A field's folder with the entry's `{slug}`, `{section}` and `{type}`
 * filled in. A missing one (a new entry without a slug, no editor)
 * ends the path before its folder. It is resolved at each upload, so a
 * renamed slug leaves the files where they are, like Craft.
 */
export function fieldFolder(
	path: string | undefined,
	entry: Pick<Editing, 'slug' | 'section' | 'type'> | undefined,
): string | undefined {
	const names: string[] = [];
	for (const name of (path ?? '').split('/')) {
		let missing = false;
		const filled = name.replace(/\{(\w+)\}/g, (_, k: string) => {
			// the schema check lets only these through
			const v = entry?.[k as 'slug' | 'section' | 'type'] ?? '';
			if (!v) missing = true;
			return v;
		});
		if (missing) break;
		names.push(filled);
	}
	return names.join('/') || undefined;
}

/** The folder of a path like `Photos/Events`, if it exists. */
export function folderByPath(
	folders: Folder[],
	path: string | undefined,
): Folder | undefined {
	const { found, rest } = deepest(folders, path);
	return rest.length ? undefined : found;
}

/**
 * The part of a path that has no folders yet: the deepest one that is
 * there (null: the top) and the names below it; none when all are.
 */
export function missingFolders(
	folders: Folder[],
	path: string | undefined,
): { parent: number | null; names: string[] } | null {
	const { found, rest } = deepest(folders, path);
	return rest.length ? { parent: found?.id ?? null, names: rest } : null;
}

/** The deepest folder of a path that exists, and the names below it. */
function deepest(
	folders: Folder[],
	path: string | undefined,
): { found: Folder | undefined; rest: string[] } {
	const names = pathDirs(path);
	let found: Folder | undefined;
	for (const [i, name] of names.entries()) {
		const child = childFolder(folders, found?.id ?? null, name);
		if (!child) return { found, rest: names.slice(i) };
		found = child;
	}
	return { found, rest: [] };
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
