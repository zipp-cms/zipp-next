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
	/** invited, no password yet: the invite link sets it */
	invited: boolean;
};

/** A link for a user, see `passwordLink` and `loginAsLink`. */
export type UserLink = {
	kind: 'invite' | 'reset' | 'loginAs';
	url: string;
	expiresAt: number;
	/** whether it was mailed */
	sent: boolean;
};

export function listUsers(): Promise<Account[]> {
	return request('GET', '/users');
}

export function getUser(id: number): Promise<Account> {
	return request('GET', `/users/${id}`);
}

/** A new user; an empty password invites them, see `passwordLink`. */
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

/**
 * A link for the user to choose a password: an invite while they have
 * none, else a reset; `send` mails it.
 */
export function passwordLink(id: number, send: boolean): Promise<UserLink> {
	return request('POST', `/users/${id}/password-link`, { send });
}

/** A link that logs in as the user once, for admins. */
export function loginAsLink(id: number): Promise<UserLink> {
	return request('POST', `/users/${id}/login-as`);
}

/** The name of a user, else the email. */
export function displayName(user: Pick<User, 'name' | 'email'>): string {
	return user.name || user.email;
}
