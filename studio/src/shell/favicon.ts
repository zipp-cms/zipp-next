import { download } from '../core/client';

const icons = new Map<string, Promise<string | null>>();

/**
 * The favicon of a site's website as an object url, null when it has
 * none: the cms fetches it, the studio can't read another origin. Asked
 * once per site and load of the studio, again after a failure.
 */
export function favicon(site: string): Promise<string | null> {
	let icon = icons.get(site);
	if (!icon) {
		icon = download(`/sites/${encodeURIComponent(site)}/favicon`)
			// empty: the website has none
			.then(blob => (blob.size ? URL.createObjectURL(blob) : null))
			.catch(() => {
				// asked again next time: it may have been the network
				icons.delete(site);
				return null;
			});
		icons.set(site, icon);
	}
	return icon;
}
