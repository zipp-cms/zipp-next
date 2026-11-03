// the search page has no entry in zipp, crelte gets it from App.svelte's
// loadEntry: `/suche?q=konzert` in German, `/en/search?q=concert` in
// English, see templates/search.svelte

/** @param {{ language: string }} site */
export const searchPath = site =>
	site.language.startsWith('de') ? 'suche' : 'search';

/** @param {{ language: string }} site */
export const searchTitle = site =>
	site.language.startsWith('de') ? 'Suche' : 'Search';

/** @param {import('crelte/routing').Site} site */
export const searchUrl = site =>
	`${site.url.origin}${site.uri}/${searchPath(site)}`;

/**
 * The entry of the search page when the request is for it, else none
 * and crelte loads the entry of the uri.
 *
 * @param {import('crelte').CrelteRequest} cr
 */
export function searchEntry(cr) {
	if (!cr.req.siteMatches()) return null;
	const uri = cr.req.uri.replace(/^\/+|\/+$/g, '');
	if (uri !== searchPath(cr.site)) return null;
	return {
		sectionHandle: 'search',
		typeHandle: 'results',
		// a new search shows the page anew
		url: cr.req.url.href,
	};
}
