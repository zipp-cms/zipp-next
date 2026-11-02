import { request } from '../core/client';
import type { Authenticated } from '../core/session.svelte';

/** A mail with a reset link, if the email has an account. */
export function forgotPassword(email: string): Promise<void> {
	return request('POST', '/auth/forgot', { email });
}

/** What an invite, reset or login link is for. */
export type LinkInfo = {
	kind: 'invite' | 'reset' | 'loginAs';
	email: string;
	name: string;
	expiresAt: number;
};

export function getLink(token: string): Promise<LinkInfo> {
	return request('GET', `/auth/links/${token}`);
}

/** The password of an invite or reset (an invited user's name too). */
export function setPassword(
	token: string,
	password: string,
	name: string | null,
): Promise<Authenticated> {
	return request('POST', `/auth/links/${token}/password`, { password, name });
}
