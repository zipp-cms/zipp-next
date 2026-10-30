export type FieldError = { path: string; message: string };

/** The errors of the server, `Error` of zipp's error.rs. */
export type ErrorType =
	| {
			type:
				| 'LOGIN_INCORRECT'
				| 'MISSING_SESSION_TOKEN'
				| 'INVALID_SESSION_TOKEN'
				| 'MISSING_ENDPOINT_TOKEN'
				| 'FORBIDDEN'
				| 'NOT_FOUND';
	  }
	| { type: 'INVALID'; detail: FieldError[] }
	| { type: 'INTERNAL' | 'REQUEST' | 'CONFLICT'; detail: string };

/** An error of a request: the server's message and what it is. */
export class ApiError extends Error {
	constructor(
		message: string,
		readonly inner: ErrorType,
		readonly status = 0,
	) {
		super(message);
	}

	get type(): ErrorType['type'] {
		return this.inner.type;
	}

	/** The fields an INVALID error names. */
	get fields(): FieldError[] {
		return this.inner.type === 'INVALID' ? this.inner.detail : [];
	}

	/** From the JSON of an error response, `{ message, error }`. */
	static fromJson(status: number, data: any, fallback: string): ApiError {
		const inner: ErrorType = data?.error?.type
			? data.error
			: { type: 'REQUEST', detail: fallback };
		return new ApiError(data?.message ?? fallback, inner, status);
	}
}

let sessionLost = () => {};

/** What happens when the session is missing or gone: the login. */
export function onSessionLost(f: () => void) {
	sessionLost = f;
}

const TOKEN = 'zipp-token';

/** The token of the session, kept in this browser or only this tab. */
export function token(): string | null {
	return localStorage.getItem(TOKEN) ?? sessionStorage.getItem(TOKEN);
}

/** `remember`: in this browser, else it ends with the tab. */
export function setToken(t: string | null, remember = true) {
	localStorage.removeItem(TOKEN);
	sessionStorage.removeItem(TOKEN);
	if (t) (remember ? localStorage : sessionStorage).setItem(TOKEN, t);
}

function send(
	method: string,
	path: string,
	body?: unknown,
	signal?: AbortSignal,
) {
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
		signal,
		body:
			body === undefined || form
				? (body as FormData)
				: JSON.stringify(body),
	});
}

/** Whether an error says the session is missing or gone. */
export function isSessionError(e: ApiError): boolean {
	return (
		e.type === 'MISSING_SESSION_TOKEN' || e.type === 'INVALID_SESSION_TOKEN'
	);
}

/** The error of a response, a session that is missing or gone logs out. */
function failed(
	resp: { status: number; statusText: string },
	data: any,
): ApiError {
	const err = ApiError.fromJson(resp.status, data, resp.statusText);
	// a role that may not do something only gets the error
	if (isSessionError(err)) {
		setToken(null);
		sessionLost();
	}
	return err;
}

/**
 * A request to the api: its answer as `T`, which the caller names (the
 * server's `Response` of the request), or the ApiError.
 */
export async function request<T = unknown>(
	method: string,
	path: string,
	body?: unknown,
): Promise<T> {
	const resp = await send(method, path, body);
	const text = await resp.text();
	// an error page of a proxy is no JSON
	let data = null;
	try {
		data = text ? JSON.parse(text) : null;
	} catch {
		if (resp.ok) throw new Error(`${path}: the answer is no JSON`);
	}
	if (!resp.ok) throw failed(resp, data);
	return data as T;
}

/**
 * A form sent with its progress, `onprogress` gets the bytes sent and
 * all of them; fetch can't tell how far an upload is.
 */
export function upload<T = unknown>(
	path: string,
	form: FormData,
	onprogress: (sent: number, total: number) => void,
): Promise<T> {
	return new Promise((resolve, reject) => {
		const xhr = new XMLHttpRequest();
		xhr.open('POST', '/api' + path);
		const t = token();
		if (t) xhr.setRequestHeader('session-token', t);
		xhr.upload.onprogress = e => onprogress(e.loaded, e.total);
		xhr.onerror = () => reject(new Error(`${path}: the upload failed`));
		xhr.onload = () => {
			let data = null;
			try {
				data = xhr.responseText ? JSON.parse(xhr.responseText) : null;
			} catch {
				if (xhr.status < 300)
					return reject(new Error(`${path}: the answer is no JSON`));
			}
			if (xhr.status >= 200 && xhr.status < 300) resolve(data as T);
			else reject(failed(xhr, data));
		};
		xhr.send(form);
	});
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
	signal?: AbortSignal,
): AsyncGenerator<StreamEvent> {
	const resp = await send(method, path, body, signal);
	if (!resp.ok || !resp.body)
		throw failed(resp, await resp.json().catch(() => null));

	// blocks of `field: value` lines, split by an empty line
	const reader = resp.body.pipeThrough(new TextDecoderStream()).getReader();
	// leaving the loop early closes the stream
	try {
		yield* events(reader);
	} finally {
		reader.cancel().catch(() => {});
	}
}

async function* events(
	reader: ReadableStreamDefaultReader<string>,
): AsyncGenerator<StreamEvent> {
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
