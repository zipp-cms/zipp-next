import type { MenuItem } from 'zipp-ui/Menu.svelte';
import { plural } from '../core/format';
import { assetUrl, zipAssets, type Asset } from './api';

/**
 * Asks before moving it to the trash, saying where it is used (`n`
 * entries): there it is gone until it is restored.
 */
export function confirmDelete(a: Asset, n: number): boolean {
	const used = n
		? ` It is used in ${plural(n, 'entry', 'entries')}, they lose it until it is restored.`
		: '';
	return confirm(`Move ${a.filename} to the trash?${used}`);
}

/** The full url of an asset, to paste somewhere. */
export function copyUrl(a: Asset): Promise<void> {
	return navigator.clipboard.writeText(location.origin + assetUrl(a));
}

/** Download and copy its url, items of every ⋯ menu of an asset. */
export function linkItems(a: Asset): MenuItem[] {
	return [
		{
			label: 'Download',
			icon: 'download',
			href: assetUrl(a, false, true),
			download: a.filename,
		},
		{ label: 'Copy URL', icon: 'copy-url', onclick: () => copyUrl(a) },
	];
}

/**
 * Downloads the zip as `name`: it comes as a blob, the session is a
 * header no link sends, and is saved through a link to it.
 */
export async function downloadZip(
	of: { ids: number[] } | { folder: number },
	name: string,
): Promise<void> {
	const url = URL.createObjectURL(await zipAssets(of));
	const a = document.createElement('a');
	a.href = url;
	a.download = name.endsWith('.zip') ? name : name + '.zip';
	a.click();
	// a moment later, the browser may still read it after the click
	setTimeout(() => URL.revokeObjectURL(url), 60_000);
}

/** Whether a file is a zip, which can be unpacked into folders. */
export function isZip(file: File): boolean {
	return file.name.toLowerCase().endsWith('.zip');
}

/** Asks whether the zips are unpacked into folders or kept as files. */
export function confirmUnzip(zips: File[]): boolean {
	const what =
		zips.length === 1 ? zips[0].name : `the ${zips.length} zip files`;
	return confirm(
		`Unpack ${what} into folders?\n\nOK unpacks, Cancel uploads it as one zip file.`,
	);
}
