import * as app from './App.svelte';
import * as errorPage from './Error.svelte';
import { main, mainError } from 'crelte/server';
import { seoRoutes } from 'meierlabs-seo/server';

export const queries = import.meta.glob('@/queries/*', { eager: true });

export async function render(serverData) {
	return await main({ app, serverData });
}

export async function renderError(error, serverData) {
	return await mainError({ error, errorPage, serverData });
}

/** @param {import('crelte/server').ServerRouter} router */
export async function routes(router) {
	// sitemaps and robots.txt from zipp
	seoRoutes(router);
	// the forms of zipp-forms, the cms mails them
	router.post('/forms/:handle', csr => forward(csr));
}

/**
 * A submission to the cms, with who sent it for its limit. crelte gives
 * no socket address: the proxy in front of this server (Caddy, Traefik)
 * adds the visitor's address at the end of X-Forwarded-For, whatever the
 * visitor wrote before it doesn't count. The cms trusts it from this
 * server with `--trusted-proxy`, see docs/operations.md.
 */
async function forward(csr) {
	const url = new URL(csr.req.url);
	const to = csr.backendUrl(url.pathname);
	to.search = url.search;
	const hops = csr.req.headers.get('x-forwarded-for')?.split(',') ?? [];
	const ip = hops.at(-1)?.trim();
	const headers = { 'content-type': 'application/json' };
	if (ip) headers['x-forwarded-for'] = ip;
	const resp = await fetch(to, {
		method: 'POST',
		headers,
		body: await csr.req.text(),
	});
	return new Response(await resp.text(), {
		status: resp.status,
		headers: { 'content-type': 'application/json' },
	});
}
