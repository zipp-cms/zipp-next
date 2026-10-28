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
};

export function assetUrl(a: Asset, thumb = false): string {
	return thumb
		? `/assets/${a.id}/_thumb/${a.filename}`
		: `/assets/${a.id}/${a.filename}`;
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
	data: { title: string; alt: string; focalX: number; focalY: number },
): Promise<Asset> {
	return request('PUT', '/assets/' + id, data);
}

export function deleteAsset(id: number): Promise<void> {
	return request('DELETE', '/assets/' + id);
}
