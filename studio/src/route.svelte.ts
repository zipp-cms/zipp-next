/**
 * Hash routes: `#/login`, `#/list/<type>`, `#/new/<type>`, `#/entry/<id>`.
 */
export type Route =
	| { page: 'login' }
	| { page: 'list'; type: string | null }
	| { page: 'new'; type: string }
	| { page: 'entry'; id: number };

function parse(hash: string): Route {
	const [page, arg] = hash.replace(/^#\/?/, '').split('/');
	switch (page) {
		case 'login':
			return { page: 'login' };
		case 'new':
			return { page: 'new', type: arg };
		case 'entry':
			return { page: 'entry', id: Number(arg) };
		case 'list':
			return { page: 'list', type: arg ?? null };
		default:
			return { page: 'list', type: null };
	}
}

export const route = $state({ current: parse(location.hash) });

window.addEventListener('hashchange', () => {
	route.current = parse(location.hash);
});

export function go(hash: string) {
	location.hash = hash;
}
