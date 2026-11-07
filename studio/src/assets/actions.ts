import type { MenuItem } from 'zipp-ui/Menu.svelte';
import { t } from '../i18n/index.svelte';
import { assetUrl, originalFile, zipAssets, type Asset } from './api';

/**
 * Asks before moving it to the trash, saying where it is used (`n`
 * entries): there it is gone until it is restored.
 */
export function confirmDelete(a: Asset, n: number): boolean {
	const { filename } = a;
	return confirm(
		n
			? t('assets.confirmTrashUsed', { filename, n })
			: t('assets.confirmTrash', { filename }),
	);
}

/** The full url of an asset, to paste somewhere. */
export function copyUrl(a: Asset): Promise<void> {
	return navigator.clipboard.writeText(location.origin + assetUrl(a));
}

/** Download and copy its url, items of every ⋯ menu of an asset. */
export function linkItems(a: Asset): MenuItem[] {
	return [
		{
			label: t('common.download'),
			icon: 'download',
			onclick: () => downloadOriginal(a),
		},
		{
			label: t('assets.copyUrl'),
			icon: 'copy-url',
			onclick: () => copyUrl(a),
		},
	];
}

/**
 * Saves a file of the api as `name`: it comes as a blob, the session is
 * a header no link sends, and is saved through a link to it.
 */
function saveBlob(blob: Blob, name: string) {
	const url = URL.createObjectURL(blob);
	const a = document.createElement('a');
	a.href = url;
	a.download = name;
	a.click();
	// a moment later, the browser may still read it after the click
	setTimeout(() => URL.revokeObjectURL(url), 60_000);
}

/** Downloads the zip as `name`. */
export async function downloadZip(
	of: { ids: number[] } | { folder: number },
	name: string,
): Promise<void> {
	const blob = await zipAssets(of);
	saveBlob(blob, name.endsWith('.zip') ? name : name + '.zip');
}

/** Downloads the file as it was uploaded. */
export async function downloadOriginal(a: Asset): Promise<void> {
	saveBlob(await originalFile(a.id), a.filename);
}

/** Whether a file is a zip, which can be unpacked into folders. */
export function isZip(file: File): boolean {
	return file.name.toLowerCase().endsWith('.zip');
}

/** Asks whether the zips are unpacked into folders or kept as files. */
export function confirmUnzip(zips: File[]): boolean {
	return confirm(
		zips.length === 1
			? t('assets.confirmUnzip', { name: zips[0].name })
			: t('assets.confirmUnzipMany', { n: zips.length }),
	);
}
