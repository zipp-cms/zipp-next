import { download, request, upload } from '../core/client';
import type { Values } from '../core/schema';
import { t } from '../i18n/index.svelte';

export type Asset = {
	id: number;
	filename: string;
	kind: 'image' | 'video' | 'file';
	mime: string;
	size: number;
	width: number | null;
	height: number | null;
	/** empty: the frontend gets one from the filename */
	title: string;
	/** per site handle, a site without has the primary site's */
	alt: Record<string, string>;
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
	/** the values of the schema's asset fields (the component `asset`) */
	fields: Values;
};

/** The component of the asset fields, in the schema when it has some. */
export const ASSET_FIELDS = 'asset';

/** The extensions of `Field::videos`. */
const VIDEOS = ['mp4', 'webm', 'mov'];

/** Whether a field of these extensions takes only videos. */
export function onlyVideos(extensions: string[] | undefined): boolean {
	return !!extensions?.length && extensions.every(e => VIDEOS.includes(e));
}

/** Whether a file is a video the browser plays. */
export function isVideo(a: Asset): boolean {
	return a.mime.startsWith('video/');
}

/** What the library can show: all, or only one type. */
export type AssetKind = 'image' | 'video' | 'document';

export function kinds(): { value: AssetKind | ''; label: string }[] {
	return [
		{ value: '', label: t('assets.all') },
		{ value: 'image', label: t('assets.images') },
		{ value: 'video', label: t('assets.videos') },
		{ value: 'document', label: t('assets.documents') },
	];
}

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

/**
 * The image with its edits, its thumbnail, or the file as uploaded; with
 * the version, a replaced file gets a new url.
 */
export function assetUrl(a: Asset, thumb = false, original = false): string {
	const path = thumb
		? `/assets/${a.id}/_thumb/${a.filename}`
		: `/assets/${a.id}/${a.filename}`;
	const params = [original ? 'original' : '', a.version && `v=${a.version}`];
	const query = params.filter(Boolean).join('&');
	return query ? `${path}?${query}` : path;
}

/** The alt text of a site, else of another one, to show in a list. */
export function altOf(a: Asset, site: string): string {
	return a.alt[site] ?? Object.values(a.alt)[0] ?? '';
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

function fileForm(file: File): FormData {
	const form = new FormData();
	form.append('file', file, file.name);
	return form;
}

/**
 * A new file in `folder` (null: the top) with its progress; a zip with
 * `unzip` is unpacked into folders there.
 */
export function uploadAsset(
	file: File,
	folder: number | null,
	unzip: boolean,
	onprogress: (sent: number) => void,
): Promise<Asset[] | Unzipped> {
	const path =
		(unzip ? '/assets/unzip' : '/assets') +
		(folder === null ? '' : `?folder=${folder}`);
	return upload(path, fileForm(file), onprogress);
}

/** Another file for the asset, where it is used stays. */
export function replaceAsset(id: number, file: File): Promise<Asset> {
	return request('POST', `/assets/${id}/file`, fileForm(file));
}

/** What to list: a search looks in all folders, else in `folder`. */
type AssetQuery = {
	search?: string;
	/** these, in this order */
	ids?: number[];
	/** null: the top, none: all folders */
	folder?: number | null;
	images?: boolean;
	/** only images, videos or documents (files but videos and audio) */
	kind?: AssetKind;
	extensions?: string[];
};

export function listAssets(q: AssetQuery): Promise<Asset[]> {
	const p = new URLSearchParams();
	if (q.search) p.set('search', q.search);
	if (q.ids) p.set('ids', q.ids.join(','));
	if (q.folder !== undefined)
		p.set('folder', q.folder === null ? 'top' : `${q.folder}`);
	if (q.images || q.kind) p.set('kind', q.images ? 'image' : q.kind!);
	if (q.extensions?.length) p.set('extensions', q.extensions.join(','));
	return request('GET', '/assets?' + p);
}

export function updateAsset(
	id: number,
	data: {
		title: string;
		/** of all sites */
		alt: Record<string, string>;
		focalX: number;
		focalY: number;
		filename?: string;
		edits?: Edits | null;
		/** unchanged when missing */
		fields?: Values;
	},
): Promise<Asset> {
	return request('PUT', '/assets/' + id, data);
}

/** Only the name of an asset, the rest as it is. */
export function renameAsset(a: Asset, filename: string): Promise<Asset> {
	return updateAsset(a.id, {
		title: a.title,
		alt: a.alt,
		focalX: a.focalX,
		focalY: a.focalY,
		filename,
		edits: a.edits,
	});
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

/** A zip of the assets, or of a folder with its subfolders. */
export function zipAssets(
	of: { ids: number[] } | { folder: number },
): Promise<Blob> {
	const q = 'ids' in of ? `ids=${of.ids.join(',')}` : `folder=${of.folder}`;
	return download('/assets/zip?' + q);
}

/** What an unpacked zip gave: its assets, the files left out and why. */
export type Unzipped = { assets: Asset[]; skipped: string[] };

/** Into a folder, or null for the top. */
export function moveAssets(ids: number[], folderId: number | null) {
	return request<void>('POST', '/assets/move', { ids, folderId });
}
