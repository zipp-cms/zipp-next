/** German lives on `/`, English on `/en`. */
export const sites = [
	{ handle: 'de', prefix: '', label: 'DE' },
	{ handle: 'en', prefix: '/en', label: 'EN' },
];

const path = location.pathname;
const current = path === '/en' || path.startsWith('/en/') ? sites[1] : sites[0];

export const site = current.handle;
/** The uri of the page in the cms, without the site prefix. */
export const uri = path.slice(current.prefix.length).replace(/^\/|\/$/g, '');

/** Runs a GraphQL query against the cms, only published entries. */
export async function gql<T>(
	query: string,
	variables: Record<string, unknown> = {},
): Promise<T> {
	const resp = await fetch('/api/graphql', {
		method: 'POST',
		headers: { 'content-type': 'application/json' },
		body: JSON.stringify({ query, variables: { site, ...variables } }),
	});
	const res = await resp.json();
	if (res.errors?.length) throw new Error(res.errors[0].message);
	return res.data;
}

/** The url of an entry in a site, by default this one. */
export function url(e: { uri: string | null }, siteHandle = site): string {
	const prefix = sites.find(s => s.handle === siteHandle)!.prefix;
	return `${prefix}/${e.uri ?? ''}`;
}

export type Link = { title: string; uri: string | null };

/** A link field, like Craft's LinkData. */
export type LinkData = {
	url: string;
	label: string | null;
	defaultLabel: string;
	target: string | null;
};

/** An image with the url of one transform and a srcset of several. */
export type Image = {
	url: string;
	srcset: string | null;
	width: number | null;
	height: number | null;
	alt: string;
};

export type Block =
	| { id: string; typeHandle: 'heading'; text: string }
	| { id: string; typeHandle: 'text'; content: { html: string } | null }
	| {
			id: string;
			typeHandle: 'image';
			image: Image[];
			caption: string | null;
	  }
	| { id: string; typeHandle: 'button'; link: LinkData | null }
	| { id: string; typeHandle: 'events'; city: string | null };

export type Entry = {
	id: number;
	typeHandle: 'home' | 'page' | 'event';
	title: string;
	// home and page
	hero?: Image[];
	intro?: string | null;
	body?: Block[];
	// event
	date?: string;
	city?: string;
	price?: number | null;
	description?: { html: string } | null;
	image?: Image[];
	page?: Link[];
};
