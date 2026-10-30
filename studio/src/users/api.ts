import { request } from '../core/client';
import type { User } from '../core/session.svelte';

/** A studio user, `role: null` is an admin. */
export type { User };

/** A user with what the studio shows about the account. */
export type Account = User & {
	createdAt: number;
	lastLoginAt: number | null;
	/** browsers it is logged in */
	sessions: number;
};

export function listUsers(): Promise<Account[]> {
	return request('GET', '/users');
}

export function getUser(id: number): Promise<Account> {
	return request('GET', `/users/${id}`);
}

export function createUser(user: {
	email: string;
	name: string;
	password: string;
	role: string | null;
}): Promise<Account> {
	return request('POST', '/users', user);
}

/**
 * The account: `password` a new one, which needs `currentPassword` for
 * oneself; the own role stays.
 */
export function updateUser(
	id: number,
	user: {
		email: string;
		name: string;
		role: string | null;
		password: string | null;
		currentPassword: string | null;
	},
): Promise<Account> {
	return request('PUT', `/users/${id}`, user);
}

export function deleteUser(id: number): Promise<void> {
	return request('DELETE', `/users/${id}`);
}

/** Logs the user out everywhere, oneself except here. */
export function endSessions(id: number): Promise<void> {
	return request('DELETE', `/users/${id}/sessions`);
}

/** The name of a user, else the email. */
export function displayName(user: Pick<User, 'name' | 'email'>): string {
	return user.name || user.email;
}
