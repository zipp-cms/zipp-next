import { toLogin } from '../route.svelte';

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

export async function request(method: string, path: string, body?: unknown) {
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
			toLogin();
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
