import { request, token } from './client';
import { type Values } from './schema';

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
export type FieldChange = { field: string; sites: string[] };

export type Revision = {
	id: number;
	user: string | null;
	createdAt: number;
	/** since the revision before, empty for the first */
	changes: FieldChange[];
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

/** A token that shows every draft on the website, for a day. */
export async function sitePreviewToken(): Promise<string> {
	return (await request('POST', '/preview')).token;
}
