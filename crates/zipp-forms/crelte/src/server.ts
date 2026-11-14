import type { CrelteServerRequest, ServerRouter } from 'crelte/server';

/**
 * The routes of zipp-forms in the frontend's server: the query of
 * `loadForm` and `/forms/{id}`, where a form is sent, both to the cms.
 *
 * ```js
 * // server.js
 * import { formRoutes } from 'zipp-forms/server';
 *
 * export async function routes(router) {
 * 	formRoutes(router);
 * }
 * ```
 *
 * The cms limits how many an address sends: this server passes the
 * visitor's address on as the last one of `X-Forwarded-For`, the one
 * the proxy in front of it (Caddy, Traefik) added. The cms trusts it
 * from this server with `serve --trusted-proxy`.
 */
export function formRoutes(router: ServerRouter) {
	router.post('/queries/zippForm', async csr => {
		const { id, site } = await csr.req.json();
		if (!Number.isInteger(id)) {
			return Response.json({ error: 'an id' }, { status: 400 });
		}
		const url = csr.backendUrl(`/forms/${id}`);
		if (typeof site === 'string') url.searchParams.set('site', site);
		const resp = await fetch(url);
		if (resp.status === 404) return Response.json({ data: { form: null } });
		if (!resp.ok) {
			return Response.json(
				{ error: `the form ${id}: ${resp.status}` },
				{ status: 502 },
			);
		}
		return Response.json({ data: { form: await resp.json() } });
	});

	router.post('/forms/:id', csr => send(csr));
}

async function send(csr: CrelteServerRequest): Promise<Response | null> {
	const url = new URL(csr.req.url);
	if (!/^\/forms\/\d+$/.test(url.pathname)) return null;
	// JSON only: a browser asks first before sending it from another
	// site, so other sites' pages can't send forms with their visitors
	const type = csr.req.headers.get('content-type') ?? '';
	if (!type.startsWith('application/json')) {
		return Response.json(
			{ success: false, message: 'a JSON object of the fields' },
			{ status: 415 },
		);
	}

	const to = csr.backendUrl(url.pathname);
	to.search = url.search;
	const hops = csr.req.headers.get('x-forwarded-for')?.split(',') ?? [];
	const ip = hops.at(-1)?.trim();
	const headers: Record<string, string> = {
		'content-type': 'application/json',
	};
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
