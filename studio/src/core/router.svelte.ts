import { Router, type Request, type Route } from 'chuchi';
import type { Component } from 'svelte';
import { onSessionLost } from './client';
import { session, type Session } from './session.svelte';
import { getSchema, type Schema } from './schema';
import { site, storeSite } from './site.svelte';

/**
 * The pages of the studio with chuchi's router, like hostdinghy: each
 * area registers its pages in its `routes.ts`, loaded when opened. A page
 * module may export `loadProps` (its data before it shows), `layout`
 * (the frame around it) and `isPublic` (no login needed).
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

	/** Opens another page instead of this one, the path under /admin. */
	setRedirect(path: string) {
		this.redirect = BASE + path;
	}
}

export const BASE = '/admin';

export const router = new Router<PageModule>();

/** The path of the current page, without `/admin`. */
export const current = $state({ path: '' });

/** Opens a page of the studio, `path` without `/admin`. */
export function go(path: string, replace = false) {
	router.open(BASE + path, {}, { history: replace ? 'replace' : 'push' });
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
	schema ??= getSchema().then(s => {
		if (!s.sites.some(x => x.handle === site.current))
			storeSite(s.sites[0].handle);
		return s;
	});
	schema.catch(() => (schema = null));
	return schema;
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

/** Pages under one path in one frame. */
export function inLayout(
	layout: LoadModule,
	pages: [string | RegExp, LoadModule][],
) {
	for (const [path, page] of pages) {
		const uri =
			typeof path === 'string'
				? BASE + path
				: new RegExp(escape(BASE) + path.source, path.flags);
		router.register(uri, withLayout(layout, page));
	}
}

const escape = (s: string) => s.replace(/[/.]/g, '\\$&');

/** The page of a request, its data loaded, or where to go instead. */
async function resolve(
	req: Request,
	route: Route<PageModule> | null,
): Promise<{ page: Page } | { redirect: string }> {
	// an anchor on the same page is no new page
	const key = req.pathname + req.url.search + '|' + site.current;
	const NotFound = (await import('../shell/NotFound.svelte')).default;
	if (!route)
		return { page: { layout: null, component: NotFound, props: {}, key } };

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
			? (await import('../shell/ErrorPage.svelte')).default
			: mod.default;
		return {
			page: { layout: mod.layout ?? null, component, props, key },
		};
	} catch (e) {
		const ErrorPage = (await import('../shell/ErrorPage.svelte')).default;
		return {
			page: {
				layout: null,
				component: ErrorPage,
				props: { error: e },
				key,
			},
		};
	}
}

/**
 * Starts the router: `onfirst` is called when the first page is ready,
 * to mount the app.
 */
export function startRouter(onfirst: () => void) {
	onSessionLost(() => {
		session.invalidate();
		forgetSchema();
		toLogin();
	});
	router.currentRequest.subscribe(req => {
		if (req) current.path = req.pathname.slice(BASE.length) || '/';
	});
	let mounted = false;
	router.onRoute(async (req, route, routing) => {
		view.loading++;
		try {
			const r = await resolve(req, route);
			if ('redirect' in r) {
				router.open(r.redirect, {}, { history: 'replace' });
				return;
			}
			if (await routing.dataReady()) return;
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
		markAnchor(req);
	});
	router.initClient();
}

/** An anchor of the url: scrolled to and marked for a moment. */
function markAnchor(req: Request) {
	const id = decodeURIComponent(req.url.hash.slice(1));
	if (!id) return;
	// the page may still render what the anchor names
	let tries = 0;
	const find = () => {
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
