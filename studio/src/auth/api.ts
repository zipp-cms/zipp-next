import { request, setToken } from '../core/client';

/** Logging in and out of the studio, the token is kept by the client. */

export async function login(email: string, password: string) {
	const d = await request('POST', '/auth/login', { email, password });
	setToken(d.token);
}

export async function logout() {
	await request('POST', '/auth/logout');
	setToken(null);
}
