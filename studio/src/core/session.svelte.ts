import {
	ApiError,
	isSessionError,
	request,
	setToken,
	token,
	tokenRemembered,
} from './client';

/** A user of the studio, `role` null is an admin. */
export type User = {
	id: number;
	email: string;
	/** empty when it isn't set */
	name: string;
	role: string | null;
};

export type Authenticated = {
	session: {
		token: string;
		expiresAt: number;
		/** the email of the admin who logged in as the user */
		impersonatedBy: string | null;
	};
	user: User;
};

/** The admin's token while they are logged in as someone else. */
const BEFORE = 'zipp-token-before';

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
	/** the admin who is logged in as the user, see `loginAs` */
	impersonatedBy = $state<string | null>(null);
	/**
	 * The session ended while the studio was open: the page stays and a
	 * dialog asks for the login again, see `expire`.
	 */
	expired = $state(false);

	/** where the token was kept, the new one goes there too */
	private remember = true;
	/** what waits for a new login of the same user, see `onRenewed` */
	private renewed = new Set<() => void>();

	get loggedIn(): boolean {
		return this.user !== null;
	}

	/**
	 * The session of the stored token, if it is still valid. Only a
	 * session the server doesn't know logs out; when the server can't be
	 * reached the token stays and the error comes back.
	 */
	async init() {
		if (!token()) return;
		try {
			this.set(await request<Authenticated>('GET', '/auth/session'));
		} catch (e) {
			if (!(e instanceof ApiError) || !isSessionError(e)) throw e;
			this.invalidate();
		}
	}

	/** `remember`: stays logged in this browser, else only in this tab. */
	async login(email: string, password: string, remember = true) {
		const auth = await request<Authenticated>('POST', '/auth/login', {
			email,
			password,
		});
		this.use(auth, remember);
	}

	/** A session the api gave, e.g. after setting a password. */
	use(auth: Authenticated, remember = true) {
		this.set(auth);
		setToken(auth.session.token, remember);
	}

	/**
	 * Logs in with a link an admin made for another user; the own
	 * session waits for `endLoginAs`.
	 */
	async loginAs(link: string) {
		const auth = await request<Authenticated>(
			'POST',
			`/auth/links/${link}/login-as`,
		);
		const own = token();
		if (own && !this.impersonatedBy) localStorage.setItem(BEFORE, own);
		this.use(auth);
	}

	/**
	 * The session ended while the studio is open: the user stays for the
	 * dialog, the token goes. Requests that come back without one after
	 * it don't change where the token was kept.
	 */
	expire() {
		if (this.expired) return;
		this.remember = tokenRemembered();
		this.expired = true;
		setToken(null);
	}

	/**
	 * Logs in again after `expire`; whether it is the same user, then
	 * what waits for it goes on (`onRenewed`). Another user has other
	 * rights, the caller loads the studio new.
	 */
	async renew(email: string, password: string): Promise<boolean> {
		const before = this.user?.id;
		const auth = await request<Authenticated>('POST', '/auth/login', {
			email,
			password,
		});
		this.use(auth, this.remember);
		this.expired = false;
		if (auth.user.id !== before) return false;
		for (const f of this.renewed) f();
		return true;
	}

	/** `f` after a new login of the same user; the result removes it. */
	onRenewed(f: () => void): () => void {
		this.renewed.add(f);
		return () => this.renewed.delete(f);
	}

	/** Back to the own session of before, if there was one; whether. */
	async endLoginAs(): Promise<boolean> {
		await request('POST', '/auth/logout').catch(() => {});
		const before = localStorage.getItem(BEFORE);
		localStorage.removeItem(BEFORE);
		this.invalidate();
		if (!before) return false;
		setToken(before);
		await this.init();
		return this.loggedIn;
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
		this.impersonatedBy = null;
		this.expired = false;
		setToken(null);
	}

	private set(auth: Authenticated) {
		this.token = auth.session.token;
		this.expiresAt = new Date(auth.session.expiresAt);
		this.user = auth.user;
		this.impersonatedBy = auth.session.impersonatedBy;
	}
}

export const session = new Session();
