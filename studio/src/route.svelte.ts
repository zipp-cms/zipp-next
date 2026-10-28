/**
 * The pages of the studio are paths under `/admin`, like Craft:
 * `/admin/login`, `/admin/assets`, `/admin/ai`, `/admin/users`,
 * `/admin/model/<sections|types|blocks|sites>`,
 * `/admin/docs/<doc>`, `/admin/entries/<section>`,
 * `/admin/entries/<section>/new`, `/admin/entries/<section>/<id>`.
 */
export type Route =
	| { page: 'login' }
	| { page: 'assets' }
	| { page: 'ai' }
	| { page: 'users' }
	| { page: 'model'; tab: string }
	| { page: 'doc'; handle: string }
	| { page: 'list'; section: string | null }
	| { page: 'new'; section: string }
	| { page: 'entry'; section: string; id: number };

export const BASE = '/admin';

function parse(path: string): Route {
	const [page, a, b] = path
		.slice(BASE.length)
		.split('/')
		.filter(p => p)
		.map(decodeURIComponent);
	switch (page) {
		case 'login':
		case 'assets':
		case 'ai':
		case 'users':
			return { page };
		case 'model':
			return { page: 'model', tab: a ?? 'sections' };
		case 'docs':
			return { page: 'doc', handle: a };
		case 'entries':
			if (b === 'new') return { page: 'new', section: a };
			if (b) return { page: 'entry', section: a, id: Number(b) };
			return { page: 'list', section: a ?? null };
		default:
			return { page: 'list', section: null };
	}
}

export const route = $state({ current: parse(location.pathname) });

window.addEventListener('popstate', () => {
	route.current = parse(location.pathname);
});

/** Opens a page of the studio, `path` without `/admin`. */
export function go(path: string, replace = false) {
	const url = BASE + path;
	if (replace) history.replaceState(null, '', url);
	else if (url !== location.pathname) history.pushState(null, '', url);
	route.current = parse(url);
	if (!replace) window.scrollTo(0, 0);
}

/** Opens the login, which comes back to this page. */
export function toLogin() {
	if (route.current.page === 'login') return;
	sessionStorage.setItem('zipp-back', location.pathname.slice(BASE.length));
	go('/login', true);
}

/** After the login: the page that was wanted. */
export function back() {
	const path = sessionStorage.getItem('zipp-back');
	sessionStorage.removeItem('zipp-back');
	go(path && path !== '/login' ? path : '/', true);
}

/** The path of an entry in the studio. */
export function entryPath(section: string, id: number) {
	return `/entries/${section}/${id}`;
}

// links to pages of the studio open without a reload
document.addEventListener('click', e => {
	if (e.defaultPrevented || e.button !== 0) return;
	if (e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;
	const a = (e.target as Element).closest?.('a');
	if (!a || a.target || a.origin !== location.origin) return;
	if (!a.pathname.startsWith(BASE + '/') && a.pathname !== BASE) return;
	e.preventDefault();
	go(a.pathname.slice(BASE.length));
	// an anchor on the new page, once it is there
	const id = decodeURIComponent(a.hash.slice(1));
	if (id) setTimeout(() => document.getElementById(id)?.scrollIntoView(), 50);
});

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
