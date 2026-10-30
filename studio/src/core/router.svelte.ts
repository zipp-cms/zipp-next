import { Router, type Request, type Route } from 'chuchi';
import type { Component } from 'svelte';
import { onSessionLost } from './client';
import { flyout } from './flyout.svelte';
import { session, type Session } from './session.svelte';
import { getSchema, type Schema } from './schema';

/**
 * The pages of the studio with chuchi's router, like hostdinghy: each
 * area registers its pages in its `routes.ts`, loaded when opened. A page
 * module may export `loadProps` (its data before it shows), `layout`
 * (the frame around it) and `isPublic` (no login needed).
 *
 * Pages are paths under `/admin`, like Craft, and every path here is the
 * whole one: `go('/admin/assets')`, `href="/admin/assets"`.
 */

export type Props = Record<string, any>;

export type PageModule = {
	default: Component<any>;
	layout?: Component<any>;
	// any: each page names the props it takes
	loadProps?: (
		props: any,
		lp: LoadProps,
	) => Promise<Props | void> | Props | void;
	isPublic?: boolean;
};

export type LoadModule = (req: Request) => Promise<PageModule>;

/** What `loadProps` gets besides the props of the route. */
export class LoadProps {
	redirect: string | null = null;

	constructor(readonly req: Request) {}

	/** Who is logged in. */
	get session(): Session {
		return session;
	}

	/** The schema, loaded once after the login. */
	schema(): Promise<Schema> {
		return loadSchema();
	}

	/** Opens another page instead of this one. */
	setRedirect(path: string) {
		this.redirect = path;
	}
}

const BASE = '/admin';

export const router = new Router<PageModule>();

/** Where the studio is. */
class Current {
	/** the path of the page, with `/admin` */
	path = $state('');
	/** the section of an entry or doc page, the nav shows its group */
	section = $derived(
		this.path.match(/^\/admin\/(?:entries|docs)\/([^/]+)/)?.[1] ?? '',
	);
}

export const current = new Current();

/** The path of an entry in the studio. */
export function entryPath(section: string, id: number) {
	return `${BASE}/entries/${section}/${id}`;
}

/** Opens a page of the studio. */
export function go(path: string, replace = false) {
	router.open(path, {}, { history: replace ? 'replace' : 'push' });
}

/** The site the studio edits, the same in all pages. */
export const site = $state({
	current: localStorage.getItem('zipp-site') ?? '',
});

function storeSite(handle: string) {
	site.current = handle;
	localStorage.setItem('zipp-site', handle);
}

/** Another site: the page loads again with its content. */
export function setSite(handle: string) {
	storeSite(handle);
	if (router.currentRequest.get()) router.reload();
}

/** Opens the login, which comes back to this page. */
export function toLogin() {
	if (location.pathname === BASE + '/login') return;
	const back = new URLSearchParams({
		url: location.pathname + location.search,
	});
	router.open(`${BASE}/login?${back}`, {}, { history: 'replace' });
}

let schema: Promise<Schema> | null = null;

function loadSchema(): Promise<Schema> {
	if (schema) return schema;
	const loading = getSchema().then(s => {
		if (!s.sites.some(x => x.handle === site.current))
			storeSite(s.sites[0].handle);
		return s;
	});
	// a failed load is tried again, unless a newer one runs already
	loading.catch(() => {
		if (schema === loading) schema = null;
	});
	return (schema = loading);
}

/** After a logout the next user gets their own schema. */
export function forgetSchema() {
	schema = null;
}

/** A page with its frame and props, what App renders. */
export type Page = {
	layout: Component<any> | null;
	component: Component<any>;
	props: Props;
	/** a new key renders the page new */
	key: string;
};

export const view = $state<{ page: Page | null; loading: number }>({
	page: null,
	loading: 0,
});

/** A page in a frame: the layout's props go to the page too. */
export function withLayout(layout: LoadModule, page: LoadModule): LoadModule {
	return async req => {
		const [l, p] = await Promise.all([layout(req), page(req)]);
		return {
			isPublic: p.isPublic,
			async loadProps(props, lp) {
				const fromLayout = (await l.loadProps?.(props, lp)) ?? {};
				props = { ...props, ...fromLayout };
				// the page's error shows in the frame
				try {
					const own = (await p.loadProps?.(props, lp)) ?? {};
					return { ...props, ...own };
				} catch (e) {
					return { ...props, loadError: e };
				}
			},
			layout: l.default,
			default: p.default,
		};
	};
}

/** Pages in one frame. */
export function inLayout(
	layout: LoadModule,
	pages: [string | RegExp, LoadModule][],
) {
	for (const [path, page] of pages)
		router.register(path, withLayout(layout, page));
}

/** The pages of the app for what has no page: a wrong path, an error. */
export type Fallbacks = {
	notFound: () => Promise<{ default: Component<any> }>;
	/** gets `loadError` */
	error: () => Promise<{ default: Component<any> }>;
};

let fallbacks: Fallbacks;

/** The page of a request, its data loaded, or where to go instead. */
async function resolve(
	req: Request,
	route: Route<PageModule> | null,
): Promise<{ page: Page } | { redirect: string }> {
	// an anchor on the same page is no new page
	const key = req.pathname + req.url.search + '|' + site.current;
	if (!route) {
		const NotFound = (await fallbacks.notFound()).default;
		return { page: { layout: null, component: NotFound, props: {}, key } };
	}

	try {
		const mod = await route.load(req);
		if (!mod.isPublic && !session.loggedIn) {
			const back = new URLSearchParams({ url: req.uri });
			return { redirect: `${BASE}/login?${back}` };
		}
		const lp = new LoadProps(req);
		let props: Props = route.toProps(req);
		props = { ...props, ...((await mod.loadProps?.(props, lp)) ?? {}) };
		if (lp.redirect) return { redirect: lp.redirect };
		const component = props.loadError
			? (await fallbacks.error()).default
			: mod.default;
		return {
			page: { layout: mod.layout ?? null, component, props, key },
		};
	} catch (e) {
		const ErrorPage = (await fallbacks.error()).default;
		return {
			page: {
				layout: null,
				component: ErrorPage,
				props: { loadError: e },
				key,
			},
		};
	}
}

/** The session is gone: everything of it, and the login. */
export function endSession(login = true) {
	session.invalidate();
	forgetSchema();
	flyout.open = null;
	if (login) toLogin();
}

/**
 * Starts the router: `onfirst` is called when the first page is ready,
 * to mount the app.
 */
export function startRouter(pages: Fallbacks, onfirst: () => void) {
	fallbacks = pages;
	onSessionLost(endSession);
	router.currentRequest.subscribe(req => {
		if (req) current.path = req.pathname;
	});
	let mounted = false;
	// the newest request, an older one doesn't redirect anymore
	let newest = 0;
	router.onRoute(async (req, route, routing) => {
		const mine = ++newest;
		view.loading++;
		try {
			const r = await resolve(req, route);
			if ('redirect' in r) {
				if (mine !== newest) return;
				// the redirect takes the place of the request: a link adds
				// to the history, the first page and back/forward don't
				const first = !router.currentRequest.get();
				const history =
					first || req.origin === 'pop' ? 'replace' : 'push';
				router.open(r.redirect, {}, { history });
				// the request ends, the new one replaced it
				await routing.dataReady();
				await routing.domReady();
				return;
			}
			if (await routing.dataReady()) return;
			// another page: the flyout was of the one before
			if (view.page && view.page.key !== r.page.key) flyout.open = null;
			view.page = r.page;
			if (!mounted) {
				mounted = true;
				onfirst();
			}
		} finally {
			view.loading--;
		}
		await Promise.resolve();
		await routing.domReady();
		markAnchor(req, () => mine === newest);
	});
	router.initClient();
}

/** An anchor of the url: scrolled to and marked for a moment. */
function markAnchor(req: Request, still: () => boolean) {
	const id = decodeURIComponent(req.url.hash.slice(1));
	if (!id) return;
	// the page may still render what the anchor names
	let tries = 0;
	const find = () => {
		if (!still()) return;
		const el = document.getElementById(id);
		if (!el) {
			if (++tries < 30) setTimeout(find, 100);
			return;
		}
		el.scrollIntoView({ block: 'center' });
		el.classList.add('target');
		setTimeout(() => el.classList.remove('target'), 2000);
	};
	setTimeout(find, 100);
}

// only pages of the studio open in it: files and downloads not
document.addEventListener('click', e => {
	const a = (e.target as Element).closest?.('a');
	if (!a) return;
	const studio = a.pathname === BASE || a.pathname.startsWith(BASE + '/');
	if (a.origin !== location.origin || !studio || a.hasAttribute('download'))
		e.stopPropagation();
});
