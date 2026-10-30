import type { MenuItem } from 'zipp-ui/Menu.svelte';
import { plural } from '../core/format';
import { assetUrl, type Asset } from './api';

/** Asks before deleting, saying where it is used (`n` entries). */
export function confirmDelete(a: Asset, n: number): boolean {
	const used = n ? ` It is used in ${plural(n, 'entry', 'entries')}.` : '';
	return confirm(`Delete ${a.filename}?${used}`);
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
