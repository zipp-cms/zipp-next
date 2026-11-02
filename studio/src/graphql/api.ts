import { request } from '../core/client';

/** What a token reads; `null` is all sections or sites. */
export type Scope = {
	sections: string[] | null;
	sites: string[] | null;
	/** whether preview tokens show drafts */
	previews: boolean;
};

/** A token of a frontend, without the token itself. */
export type GraphqlToken = Scope & {
	id: number;
	name: string;
	createdAt: number;
	expiresAt: number | null;
	lastUsedAt: number | null;
};

export function listTokens(): Promise<{
	tokens: GraphqlToken[];
	/** whether `serve --graphql-token` set one, it reads everything */
	envToken: boolean;
}> {
	return request('GET', '/graphql/tokens');
}

/** A new token: `token` is the token itself, it shows only now. */
export function createToken(
	token: Scope & { name: string; expiresAt: number | null },
): Promise<GraphqlToken & { token: string }> {
	return request('POST', '/graphql/tokens', token);
}

/** Revokes a token, frontends that send it get 401. */
export function deleteToken(id: number): Promise<void> {
	return request('DELETE', `/graphql/tokens/${id}`);
}

/** GraphiQL, its queries run with the session of the studio. */
export function openGraphiql() {
	// not noopener: the tab keeps a session that isn't remembered; in
	// development vite serves the page itself
	window.open(import.meta.env.DEV ? '/graphiql.html' : '/api/graphql');
}
