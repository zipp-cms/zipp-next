import { request, setToken, token } from './client';

/** A user of the studio, `role` null is an admin. */
export type User = { id: number; email: string; role: string | null };

type Authenticated = {
	session: { token: string; expiresAt: number };
	user: User;
};

/**
 * Who is logged in: the session's token, when it ends and its user. The
 * server renews it when the studio opens and it ends soon, so who uses
 * the studio stays logged in. One instance, `session`; pages get it in
 * `loadProps` too.
 */
export class Session {
	token = $state<string | null>(null);
	expiresAt = $state<Date | null>(null);
	user = $state<User | null>(null);

	get loggedIn(): boolean {
		return this.user !== null;
	}

	/** The session of the stored token, if it is still valid. */
	async init() {
		if (!token()) return;
		try {
			this.set(await request('GET', '/auth/session'));
		} catch {
			// gone or ended: the login
			this.invalidate();
		}
	}

	async login(email: string, password: string) {
		this.set(await request('POST', '/auth/login', { email, password }));
	}

	async logout() {
		await request('POST', '/auth/logout').catch(() => {});
		this.invalidate();
	}

	/** The session is gone: everything of it, also the stored token. */
	invalidate() {
		this.token = null;
		this.expiresAt = null;
		this.user = null;
		setToken(null);
	}

	private set(auth: Authenticated) {
		this.token = auth.session.token;
		this.expiresAt = new Date(auth.session.expiresAt);
		this.user = auth.user;
		setToken(auth.session.token);
	}
}

export const session = new Session();
