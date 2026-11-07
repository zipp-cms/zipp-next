// GraphiQL, a page of its own next to the studio: bundled like the
// studio, so the page with the session loads no scripts of other servers.
import { createElement, useState, type ComponentProps } from 'react';
import { createRoot } from 'react-dom/client';
import { GraphiQL } from 'graphiql';
import 'graphiql/style.css';

type Fetcher = ComponentProps<typeof GraphiQL>['fetcher'];
type Token = { id: number; name: string; expiresAt: number | null };

// the session of the studio (the same origin keeps it), so an admin sees
// drafts like in the studio; without one only development is open
const token =
	localStorage.getItem('zipp-token') ?? sessionStorage.getItem('zipp-token');
const dev = document.body.hasAttribute('data-dev');

// like Craft's schema picker: the queries read as the frontend of a token
// (its sections and sites, nothing unpublished), or everything
const SCHEMA_KEY = 'zipp-graphiql-schema';
let schema = localStorage.getItem(SCHEMA_KEY) ?? '';

const fetcher: Fetcher = async params => {
	const headers: Record<string, string> = {
		'content-type': 'application/json',
	};
	if (token) headers['session-token'] = token;
	if (token && schema) headers['x-zipp-schema'] = schema;
	const resp = await fetch('/api/graphql', {
		method: 'POST',
		headers,
		body: JSON.stringify(params),
	});
	return resp.json();
};

/** The tokens an admin can pick, none for others and without a session. */
async function listTokens(): Promise<Token[]> {
	if (!token) return [];
	const resp = await fetch('/api/graphql/tokens', {
		headers: { 'session-token': token },
	});
	if (!resp.ok) return [];
	const tokens: Token[] = (await resp.json()).tokens;
	// an expired one reads nothing, like its frontend
	return tokens.filter(t => !t.expiresAt || t.expiresAt > Date.now());
}

function SchemaPicker({ tokens }: { tokens: Token[] }) {
	const [value, setValue] = useState(schema);
	return createElement(
		'select',
		{
			value,
			title: 'Read as the frontend of a token',
			style: {
				font: 'inherit',
				fontSize: '0.875rem',
				padding: '2px 4px',
			},
			onChange: (e: { target: { value: string } }) => {
				schema = e.target.value;
				localStorage.setItem(SCHEMA_KEY, schema);
				setValue(schema);
			},
		},
		createElement('option', { value: '' }, 'Full schema'),
		...tokens.map(t =>
			createElement('option', { key: t.id, value: String(t.id) }, t.name),
		),
	);
}

if (!token && !dev) location.replace('/admin');
else
	// without the list GraphiQL still opens, with the full schema
	listTokens()
		.catch(() => [] as Token[])
		.then(tokens => {
			// a picked token that was revoked or expired since
			if (!tokens.some(t => String(t.id) === schema)) schema = '';
			const logo = tokens.length
				? createElement(
						GraphiQL.Logo,
						null,
						createElement(SchemaPicker, { tokens }),
					)
				: undefined;
			createRoot(document.getElementById('graphiql')!).render(
				createElement(GraphiQL, { fetcher }, logo),
			);
		});
