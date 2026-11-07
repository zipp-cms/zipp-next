import { download } from '../core/client';

const icons = new Map<string, Promise<string | null>>();

/**
 * The favicon of a site's website as an object url, null when it has
 * none: the cms fetches it, the studio can't read another origin. Asked
 * once per site and load of the studio.
 */
export function favicon(site: string): Promise<string | null> {
	let icon = icons.get(site);
	if (!icon) {
		icon = download(`/sites/${encodeURIComponent(site)}/favicon`)
			.then(blob => URL.createObjectURL(blob))
			.catch(() => null);
		icons.set(site, icon);
	}
	return icon;
}
