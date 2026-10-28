export type LinkKind = 'entry' | 'url' | 'email' | 'phone' | 'asset';

export type Link = {
	type: LinkKind;
	value: string | number | null;
	label?: string;
	newTab?: boolean;
};

export type FieldKind =
	| { kind: 'text'; max: number | null; multiline: boolean }
	| { kind: 'number'; min: number | null; max: number | null }
	| { kind: 'boolean' }
	| { kind: 'relation'; types: string[]; max: number | null }
	| { kind: 'assets'; images: boolean; max: number | null }
	| { kind: 'link'; to: LinkKind[]; labels: boolean; newTab: boolean }
	| {
			kind: 'richText';
			headings: number[];
			lists: boolean;
			links: boolean;
			buttons: boolean;
			blocks: string[];
	  }
	| {
			kind: 'blocks';
			types: string[];
			min: number | null;
			max: number | null;
	  };

export type Field = FieldKind & {
	handle: string;
	label: string;
	required: boolean;
	indexed: boolean;
	translatable: boolean;
};

export type Component = {
	handle: string;
	name: string;
	entry: boolean;
	fields: Field[];
};

export type Site = { handle: string; name: string; language: string };

export type Section = {
	handle: string;
	name: string;
	kind: 'single' | 'channel' | 'structure';
	types: string[];
	uri: string | null;
};

export type NavGroup = { name: string; sections: string[] };

export type Schema = {
	sites: Site[];
	sections: Section[];
	components: Component[];
	nav: NavGroup[];
	/** the url of each site, by handle */
	siteUrls: Record<string, string>;
};

export type Block = { id?: string; type: string; fields: Values };
export type Values = Record<string, any>;

export type Entry = {
	id: number;
	section: string;
	type: string;
	site: string;
	title: string;
	slug: string;
	uri: string | null;
	published: boolean;
	parentId: number | null;
	fields: Values;
	createdAt: number;
	updatedAt: number;
};

export type EntryRef = {
	id: number;
	type: string;
	title: string;
	slug: string;
};
export type Refs = Record<number, EntryRef>;

export type FieldError = { path: string; message: string };

export class ApiError extends Error {
	status: number;
	fields: FieldError[];

	constructor(status: number, message: string, fields: FieldError[]) {
		super(message);
		this.status = status;
		this.fields = fields;
	}
}

export function token(): string | null {
	return localStorage.getItem('zipp-token');
}

export function setToken(t: string | null) {
	if (t) localStorage.setItem('zipp-token', t);
	else localStorage.removeItem('zipp-token');
}

async function request(method: string, path: string, body?: unknown) {
	const headers: Record<string, string> = {};
	const t = token();
	if (t) headers['session-token'] = t;
	// a form is sent as it is, the browser sets its content type
	const form = body instanceof FormData;
	if (body !== undefined && !form)
		headers['content-type'] = 'application/json';

	const resp = await fetch('/api' + path, {
		method,
		headers,
		body:
			body === undefined || form
				? (body as FormData)
				: JSON.stringify(body),
	});
	const text = await resp.text();
	const data = text ? JSON.parse(text) : null;

	if (!resp.ok) {
		if (resp.status === 401 || resp.status === 403) {
			setToken(null);
			location.hash = '#/login';
		}
		const fields = data?.error?.type === 'INVALID' ? data.error.detail : [];
		throw new ApiError(
			resp.status,
			data?.message ?? resp.statusText,
			fields,
		);
	}
	return data;
}

export async function login(email: string, password: string) {
	const d = await request('POST', '/auth/login', { email, password });
	setToken(d.token);
}

export async function logout() {
	await request('POST', '/auth/logout');
	setToken(null);
}

export function getSchema(): Promise<Schema> {
	return request('GET', '/schema');
}

export function listEntries(params: Record<string, string>): Promise<{
	entries: Entry[];
	total: number;
	relations: Refs;
	published: Record<number, Record<string, boolean>>;
}> {
	return request('GET', '/entries?' + new URLSearchParams(params));
}

export function getEntry(
	id: number,
	site: string,
): Promise<{ entry: Entry; relations: Refs }> {
	return request('GET', `/entries/${id}?site=${site}`);
}

export function moveEntry(
	id: number,
	parentId: number | null,
	index: number,
): Promise<void> {
	return request('POST', `/entries/${id}/move`, { parentId, index });
}

export type EntryInput = {
	section: string;
	type: string;
	site: string;
	parentId: number | null;
	title: string;
	slug: string;
	published: boolean;
	fields: Values;
};

export function saveEntry(
	id: number | null,
	input: EntryInput,
): Promise<Entry> {
	if (id === null) return request('POST', '/entries', input);
	return request('PUT', '/entries/' + id, input);
}

export function deleteEntry(id: number): Promise<void> {
	return request('DELETE', '/entries/' + id);
}

export type Asset = {
	id: number;
	filename: string;
	kind: 'image' | 'file';
	mime: string;
	size: number;
	width: number | null;
	height: number | null;
	alt: string;
	focalX: number;
	focalY: number;
	createdAt: number;
};

export function assetUrl(a: Asset, thumb = false): string {
	return thumb
		? `/assets/${a.id}/_thumb/${a.filename}`
		: `/assets/${a.id}/${a.filename}`;
}

export function listAssets(params: Record<string, string>): Promise<Asset[]> {
	return request('GET', '/assets?' + new URLSearchParams(params));
}

export function uploadAssets(files: FileList | File[]): Promise<Asset[]> {
	const form = new FormData();
	for (const f of files) form.append('file', f, f.name);
	return request('POST', '/assets', form);
}

export function updateAsset(
	id: number,
	data: { alt: string; focalX: number; focalY: number },
): Promise<Asset> {
	return request('PUT', '/assets/' + id, data);
}

export function deleteAsset(id: number): Promise<void> {
	return request('DELETE', '/assets/' + id);
}

/** The groups of the top bar, all sections in one without a nav. */
export function navGroups(schema: Schema): NavGroup[] {
	if (schema.nav.length) return schema.nav;
	return [{ name: 'Entries', sections: schema.sections.map(s => s.handle) }];
}

/** The url of an entry on the website, if the site has one. */
export function entryUrl(
	schema: Schema,
	site: string,
	uri: string | null,
): string | null {
	const base = schema.siteUrls[site];
	if (uri === null || !base) return null;
	return `${base.replace(/\/$/, '')}/${uri}`;
}

export type SiteDoc = {
	title: string;
	slug: string;
	published: boolean;
	fields: Values;
};

export type Draft = {
	doc: Record<string, SiteDoc>;
	user: string | null;
	updatedAt: number;
};

/** A field (or title, slug, published) that changed, in which sites. */
export type Change = { field: string; sites: string[] };

export type Revision = {
	id: number;
	user: string | null;
	createdAt: number;
	/** since the revision before, empty for the first */
	changes: Change[];
};

export function getRevision(
	id: number,
	rev: number,
): Promise<Record<string, SiteDoc>> {
	return request('GET', `/entries/${id}/revisions/${rev}`);
}

export async function getDraft(id: number): Promise<Draft | null> {
	return (await request('GET', `/entries/${id}/draft`)).draft;
}

/**
 * `base` is the version of the draft the editor knows, a 409 means someone
 * else changed it meanwhile.
 */
export function putDraft(
	id: number,
	site: string,
	content: SiteDoc,
	base: number | null,
): Promise<Draft> {
	return request('PUT', `/entries/${id}/draft`, { site, base, ...content });
}

export function discardDraft(id: number): Promise<void> {
	return request('DELETE', `/entries/${id}/draft`);
}

export function publishDraft(id: number): Promise<void> {
	return request('POST', `/entries/${id}/publish`);
}

export function getRevisions(id: number): Promise<Revision[]> {
	return request('GET', `/entries/${id}/revisions`);
}

export function restoreRevision(id: number, rev: number): Promise<Draft> {
	return request('POST', `/entries/${id}/revisions/${rev}/restore`);
}

/** A link that shows the draft on the website for a day. */
export async function previewToken(id: number): Promise<string> {
	return (await request('POST', `/entries/${id}/preview`)).token;
}
