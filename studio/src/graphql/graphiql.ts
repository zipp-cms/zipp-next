// GraphiQL, a page of its own next to the studio: bundled like the
// studio, so the page with the session loads no scripts of other servers.
import { createElement, useState, type ComponentProps } from 'react';
import { createRoot } from 'react-dom/client';
import { GraphiQL } from 'graphiql';
import 'graphiql/style.css';

type Fetcher = ComponentProps<typeof GraphiQL>['fetcher'];
type GraphqlSchema = { handle: string; name: string };

// the session of the studio (the same origin keeps it), so an admin sees
// drafts like in the studio; without one only development is open
const token =
	localStorage.getItem('zipp-token') ?? sessionStorage.getItem('zipp-token');
const dev = document.body.hasAttribute('data-dev');

// like Craft's schema picker: the queries read as a frontend with a
// GraphQL schema of the code (its sections and sites, nothing
// unpublished), or everything
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

/** The schemas an admin can pick, none for others and without a session. */
async function listSchemas(): Promise<GraphqlSchema[]> {
	if (!token) return [];
	const resp = await fetch('/api/graphql/tokens', {
		headers: { 'session-token': token },
	});
	if (!resp.ok) return [];
	return (await resp.json()).schemas;
}

function SchemaPicker({ schemas }: { schemas: GraphqlSchema[] }) {
	const [value, setValue] = useState(schema);
	return createElement(
		'select',
		{
			value,
			title: 'Read as a frontend with this schema',
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
		...schemas.map(s =>
			createElement('option', { key: s.handle, value: s.handle }, s.name),
		),
	);
}

if (!token && !dev) location.replace('/admin');
else
	// without the list GraphiQL still opens, with the full schema
	listSchemas()
		.catch(() => [] as GraphqlSchema[])
		.then(schemas => {
			// a picked one the code doesn't have anymore, or a token's id
			// of before
			if (!schemas.some(s => s.handle === schema)) schema = '';
			const logo = schemas.length
				? createElement(
						GraphiQL.Logo,
						null,
						createElement(SchemaPicker, { schemas }),
					)
				: undefined;
			createRoot(document.getElementById('graphiql')!).render(
				createElement(GraphiQL, { fetcher }, logo),
			);
		});
