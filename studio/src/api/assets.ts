import { request } from './client';

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
};

/**
 * What the image editor changes, the uploaded file stays: rotate, flip,
 * crop (0 to 1 of the rotated image), resize, a canvas around it.
 */
export type Edits = {
	rotate: number;
	flipH: boolean;
	flipV: boolean;
	crop: { x: number; y: number; w: number; h: number } | null;
	resize: { width: number; height: number | null } | null;
	canvas: { width: number; height: number; background: string } | null;
};

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

export function listAssets(params: Record<string, string>): Promise<Asset[]> {
	return request('GET', '/assets?' + new URLSearchParams(params));
}

export function uploadAssets(files: FileList | File[]): Promise<Asset[]> {
	const form = new FormData();
	for (const f of files) form.append('file', f, f.name);
	return request('POST', '/assets', form);
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
