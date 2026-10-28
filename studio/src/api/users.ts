import { request } from './client';

/** A studio user, `role: null` is an admin. */
export type User = { id: number; email: string; role: string | null };

export function listUsers(): Promise<User[]> {
	return request('GET', '/users');
}

/** A new user, or a new password and role of the one with the email. */
export function saveUser(
	email: string,
	password: string,
	role: string | null,
): Promise<void> {
	return request('POST', '/users', { email, password, role });
}

export function setRole(id: number, role: string | null): Promise<void> {
	return request('PUT', `/users/${id}`, { role });
}

export function deleteUser(id: number): Promise<void> {
	return request('DELETE', `/users/${id}`);
}
