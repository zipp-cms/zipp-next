/** The site the studio edits, the same in all pages. */
export const site = $state({
	current: localStorage.getItem('zipp-site') ?? '',
});

/** Keeps the site, see `setSite` of route.svelte.ts to switch to one. */
export function storeSite(handle: string) {
	site.current = handle;
	localStorage.setItem('zipp-site', handle);
}
