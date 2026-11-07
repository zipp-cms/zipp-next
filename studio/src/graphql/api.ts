import { request } from '../core/client';

/** What a token reads, a GraphQL schema of the code. */
export type GraphqlSchema = {
	handle: string;
	name: string;
	sections: string[];
	/** `null` all */
	sites: string[] | null;
	/** whether it lists the assets */
	assets: boolean;
	/** whether it reads the public profiles of users */
	users: boolean;
	/** whether preview tokens show drafts */
	previews: boolean;
};

/** A token of a frontend, without the token itself. */
export type GraphqlToken = {
	id: number;
	name: string;
	/** the handle of its schema, one the code may not have anymore */
	schema: string;
	createdAt: number;
	expiresAt: number | null;
	lastUsedAt: number | null;
};

export function listTokens(): Promise<{
	tokens: GraphqlToken[];
	schemas: GraphqlSchema[];
	/** whether `serve --graphql-token` set one, it reads everything */
	envToken: boolean;
}> {
	return request('GET', '/graphql/tokens');
}

/** A new token: `token` is the token itself, it shows only now. */
export function createToken(token: {
	name: string;
	schema: string;
	expiresAt: number | null;
}): Promise<GraphqlToken & { token: string }> {
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
