/**
 * Hash routes: `#/login`, `#/assets`, `#/list/<section>`, `#/new/<section>`,
 * `#/entry/<id>`.
 */
export type Route =
	| { page: 'login' }
	| { page: 'assets' }
	| { page: 'translations' }
	| { page: 'list'; section: string | null }
	| { page: 'new'; section: string }
	| { page: 'entry'; id: number };

function parse(hash: string): Route {
	const [page, arg] = hash.replace(/^#\/?/, '').split('/');
	switch (page) {
		case 'login':
			return { page: 'login' };
		case 'assets':
			return { page: 'assets' };
		case 'translations':
			return { page: 'translations' };
		case 'new':
			return { page: 'new', section: arg };
		case 'entry':
			return { page: 'entry', id: Number(arg) };
		case 'list':
			return { page: 'list', section: arg ?? null };
		default:
			return { page: 'list', section: null };
	}
}

export const route = $state({ current: parse(location.hash) });

window.addEventListener('hashchange', () => {
	route.current = parse(location.hash);
});

export function go(hash: string) {
	location.hash = hash;
}

/** The site the studio edits, the same in all pages. */
export const site = $state({
	current: localStorage.getItem('zipp-site') ?? '',
});

export function setSite(handle: string) {
	site.current = handle;
	localStorage.setItem('zipp-site', handle);
}

/** The section the page shows, so the navigation knows its group. */
export const shown = $state({ section: '' });
