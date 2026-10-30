import { request } from '../core/client';

export type Asset = {
	id: number;
	filename: string;
	kind: 'image' | 'file';
	mime: string;
	size: number;
	width: number | null;
	height: number | null;
	/** empty: the frontend gets one from the filename */
	title: string;
	alt: string;
	focalX: number;
	focalY: number;
	createdAt: number;
	/** the size of the uploaded file, `width` is the one after the edits */
	originalWidth: number | null;
	originalHeight: number | null;
	edits: Edits | null;
	/** the email of who uploaded it */
	uploadedBy: string | null;
	/** changes with the edits and the focal point, for the browser's cache */
	version: string;
	/** null: on the top, not in a folder */
	folderId: number | null;
};

/**
 * What the image editor changes, the uploaded file stays: rotate, flip,
 * crop (0 to 1 of the rotated image, up to one image past each side),
 * resize, a canvas around it, and the background past the image.
 */
export type Edits = {
	rotate: number;
	flipH: boolean;
	flipV: boolean;
	crop: { x: number; y: number; w: number; h: number } | null;
	resize: { width: number; height: number | null } | null;
	canvas: { width: number; height: number } | null;
	/** `#rrggbb` or `transparent`, transparent without one */
	background: string | null;
};

export const NO_EDITS: Edits = {
	rotate: 0,
	flipH: false,
	flipV: false,
	crop: null,
	resize: null,
	canvas: null,
	background: null,
};

/** Whether the crop reaches past the image. */
export function cropsOutside(e: Edits): boolean {
	const c = e.crop;
	return !!c && (c.x < 0 || c.y < 0 || c.x + c.w > 1 || c.y + c.h > 1);
}

/** Whether the edits leave the image as uploaded. */
export function noEdits(e: Edits): boolean {
	return (
		!e.rotate && !e.flipH && !e.flipV && !e.crop && !e.resize && !e.canvas
	);
}

/** Whether the edited image has transparent parts around it. */
export function transparentEdges(e: Edits): boolean {
	const transparent = (e.background ?? 'transparent') === 'transparent';
	return transparent && (!!e.canvas || cropsOutside(e));
}

/** The image with its edits, its thumbnail, or the file as uploaded. */
export function assetUrl(a: Asset, thumb = false, original = false): string {
	const path = thumb
		? `/assets/${a.id}/_thumb/${a.filename}`
		: `/assets/${a.id}/${a.filename}`;
	if (original) return `${path}?original`;
	return a.version ? `${path}?v=${a.version}` : path;
}

/** An entry that uses an asset, with the sites it does in. */
export type AssetUsage = {
	id: number;
	section: string;
	type: string;
	title: string;
	sites: string[];
};

export function assetUsage(id: number): Promise<AssetUsage[]> {
	return request('GET', `/assets/${id}/usage`);
}

/** Another file for the asset, where it is used stays. */
export function replaceAsset(id: number, file: File): Promise<Asset> {
	const form = new FormData();
	form.append('file', file, file.name);
	return request('POST', `/assets/${id}/file`, form);
}

/** What to list: a search looks in all folders, else in `folder`. */
export type AssetQuery = {
	search?: string;
	/** these, in this order */
	ids?: number[];
	/** null: the top, none: all folders */
	folder?: number | null;
	images?: boolean;
	extensions?: string[];
};

export function listAssets(q: AssetQuery): Promise<Asset[]> {
	const p = new URLSearchParams();
	if (q.search) p.set('search', q.search);
	if (q.ids) p.set('ids', q.ids.join(','));
	if (q.folder !== undefined)
		p.set('folder', q.folder === null ? 'top' : `${q.folder}`);
	if (q.images) p.set('kind', 'image');
	if (q.extensions?.length) p.set('extensions', q.extensions.join(','));
	return request('GET', '/assets?' + p);
}

export function updateAsset(
	id: number,
	data: {
		title: string;
		alt: string;
		focalX: number;
		focalY: number;
		filename?: string;
		edits?: Edits | null;
	},
): Promise<Asset> {
	return request('PUT', '/assets/' + id, data);
}

export function deleteAsset(id: number): Promise<void> {
	return request('DELETE', '/assets/' + id);
}

/** A folder of the asset library, only in the studio, urls stay. */
export type Folder = {
	id: number;
	parentId: number | null;
	name: string;
	/** the assets right in it, not in its subfolders */
	assets: number;
};

/** The name of the top, where all folders are and the assets in none. */
export const ROOT = 'Library';

/** The folders, and how many assets are in none. */
export function listFolders(): Promise<{ folders: Folder[]; top: number }> {
	return request('GET', '/asset-folders');
}

export function createFolder(
	parentId: number | null,
	name: string,
): Promise<Folder> {
	return request('POST', '/asset-folders', { parentId, name });
}

export function updateFolder(
	id: number,
	parentId: number | null,
	name: string,
): Promise<Folder> {
	return request('PUT', '/asset-folders/' + id, { parentId, name });
}

/** Only an empty one. */
export function deleteFolder(id: number): Promise<void> {
	return request('DELETE', '/asset-folders/' + id);
}

/** Into a folder, or null for the top. */
export function moveAssets(ids: number[], folderId: number | null) {
	return request<void>('POST', '/assets/move', { ids, folderId });
}

/** The folder of a path like `Photos/Events`, if it exists. */
export function folderByPath(
	folders: Folder[],
	path: string | undefined,
): Folder | undefined {
	let parent: number | null = null;
	let found: Folder | undefined;
	for (const name of (path ?? '').split('/').filter(n => n.trim())) {
		found = folders.find(
			f =>
				f.parentId === parent &&
				f.name.toLowerCase() === name.trim().toLowerCase(),
		);
		if (!found) return undefined;
		parent = found.id;
	}
	return found;
}

/** The folders in the order of the tree, with their depth and path. */
export function folderRows(
	folders: Folder[],
): { folder: Folder; depth: number; path: string }[] {
	const rows: { folder: Folder; depth: number; path: string }[] = [];
	const add = (parent: number | null, depth: number, path: string) => {
		for (const f of folders.filter(f => f.parentId === parent)) {
			const here = path ? `${path} / ${f.name}` : f.name;
			rows.push({ folder: f, depth, path: here });
			add(f.id, depth + 1, here);
		}
	};
	add(null, 0, '');
	return rows;
}
