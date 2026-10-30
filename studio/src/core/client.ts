import { toLogin } from './route.svelte';

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

function send(method: string, path: string, body?: unknown) {
	const headers: Record<string, string> = {};
	const t = token();
	if (t) headers['session-token'] = t;
	// a form is sent as it is, the browser sets its content type
	const form = body instanceof FormData;
	if (body !== undefined && !form)
		headers['content-type'] = 'application/json';
	return fetch('/api' + path, {
		method,
		headers,
		body:
			body === undefined || form
				? (body as FormData)
				: JSON.stringify(body),
	});
}

/** The error of a response, a session that is missing or gone logs out. */
function failed(resp: Response, data: any): ApiError {
	// a role that may not do something only gets the error
	const type = data?.error?.type;
	if (type === 'MISSING_SESSION_TOKEN' || type === 'INVALID_SESSION_TOKEN') {
		setToken(null);
		toLogin();
	}
	const fields = type === 'INVALID' ? data.error.detail : [];
	return new ApiError(resp.status, data?.message ?? resp.statusText, fields);
}

export async function request(method: string, path: string, body?: unknown) {
	const resp = await send(method, path, body);
	const text = await resp.text();
	const data = text ? JSON.parse(text) : null;
	if (!resp.ok) throw failed(resp, data);
	return data;
}

/** An event of a stream: its `event:`, `id:` and `data:` lines. */
export type StreamEvent = { event: string; id: string; data: string };

/**
 * The server-sent events of a request, as they come; the generator ends
 * when the server closes the stream.
 */
export async function* stream(
	method: string,
	path: string,
	body?: unknown,
): AsyncGenerator<StreamEvent> {
	const resp = await send(method, path, body);
	if (!resp.ok || !resp.body)
		throw failed(resp, await resp.json().catch(() => null));

	// blocks of `field: value` lines, split by an empty line
	const reader = resp.body.pipeThrough(new TextDecoderStream()).getReader();
	let buf = '';
	for (;;) {
		const { value, done } = await reader.read();
		if (done) return;
		buf += value;
		let end;
		while ((end = buf.indexOf('\n\n')) >= 0) {
			const lines = buf.slice(0, end).split('\n');
			buf = buf.slice(end + 2);
			const field = (name: string) =>
				lines
					.filter(l => l.startsWith(name + ':'))
					.map(l => l.slice(name.length + 1).trimStart())
					.join('\n');
			yield {
				event: field('event'),
				id: field('id'),
				data: field('data'),
			};
		}
	}
}
