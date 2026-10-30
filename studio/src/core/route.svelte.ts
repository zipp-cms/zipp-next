import { BASE, current, go, router, toLogin } from './router.svelte';
import { site, storeSite } from './site.svelte';

/**
 * Where the studio is: the path of the page, the site it edits, the
 * section the navigation shows. The pages are paths under `/admin`, like
 * Craft; the routes are in the `routes.ts` of each area.
 */

export { BASE, current, go, site, toLogin };

/** The path of an entry in the studio. */
export function entryPath(section: string, id: number) {
	return `/entries/${section}/${id}`;
}

/** The entry the current page edits, if it does. */
export function currentEntryId(): number | null {
	const m = current.path.match(/^\/entries\/[^/]+\/(\d+)$/);
	return m ? Number(m[1]) : null;
}

/** Another site: the page loads again with its content. */
export function setSite(handle: string) {
	storeSite(handle);
	if (router.currentRequest.get()) router.reload();
}

/** The section the page shows, so the navigation knows its group. */
export const shown = $state({ section: '' });
