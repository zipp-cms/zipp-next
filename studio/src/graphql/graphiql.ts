// GraphiQL, a page of its own next to the studio: bundled like the
// studio, so the page with the session loads no scripts of other servers.
import { createElement, type ComponentProps } from 'react';
import { createRoot } from 'react-dom/client';
import { GraphiQL } from 'graphiql';
import 'graphiql/style.css';

type Fetcher = ComponentProps<typeof GraphiQL>['fetcher'];

// the session of the studio (the same origin keeps it), so an admin sees
// drafts like in the studio; without one only development is open
const token =
	localStorage.getItem('zipp-token') ?? sessionStorage.getItem('zipp-token');
const dev = document.body.hasAttribute('data-dev');

const fetcher: Fetcher = async params => {
	const headers: Record<string, string> = {
		'content-type': 'application/json',
	};
	if (token) headers['session-token'] = token;
	const resp = await fetch('/api/graphql', {
		method: 'POST',
		headers,
		body: JSON.stringify(params),
	});
	return resp.json();
};

if (!token && !dev) location.replace('/admin');
else
	createRoot(document.getElementById('graphiql')!).render(
		createElement(GraphiQL, { fetcher }),
	);
