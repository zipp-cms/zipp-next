import { Busy } from '../core/busy.svelte';
import { session } from '../core/session.svelte';
import {
	createUser,
	deleteUser,
	endSessions,
	getUser,
	loginAsLink,
	passwordLink,
	updateUser,
	type Account,
} from './api';

/**
 * The account page of a user, or of a new one (`account` null): what it
 * changes, saving, deleting, logging out everywhere. Its errors are on the
 * fields, see Busy.
 */
export class UserForm extends Busy {
	account = $state<Account | null>(null);
	name = $state('');
	email = $state('');
	role = $state<string | null>(null);
	password = $state('');
	currentPassword = $state('');
	/** a new user without a password gets the invite by mail */
	sendInvite = $state(true);
	/** what the last link did, shown under its buttons */
	linkNote = $state('');
	/** what Save would change */
	private initial = $state('');

	/** `role` of a new user: not an admin unless chosen */
	constructor(account: Account | null, role: string | null = null) {
		super();
		this.reset(account);
		if (!account) this.role = role;
	}

	/** Whether it is the account of who is logged in. */
	get own(): boolean {
		return this.account?.id === session.user?.id;
	}

	get changed(): boolean {
		return this.snapshot() !== this.initial || this.password !== '';
	}

	private snapshot() {
		return JSON.stringify([this.name, this.email, this.role]);
	}

	private reset(account: Account | null) {
		this.account = account;
		this.name = account?.name ?? '';
		this.email = account?.email ?? '';
		this.role = account?.role ?? null;
		this.password = '';
		this.currentPassword = '';
		this.initial = this.snapshot();
	}

	/** Saves; a new user's id, else null. */
	async save(): Promise<number | null> {
		const { name, email, role } = this;
		const password = this.password || null;
		const saved = await this.run(async () => {
			if (!this.account) {
				const created = await createUser({
					name,
					email,
					role,
					password: password ?? '',
				});
				// the user is there even if the mail fails, its page sends again
				if (!password && this.sendInvite)
					await passwordLink(created.id, true).catch(e =>
						alert(`The invite mail failed: ${e.message}`),
					);
				return created;
			}
			const current = this.own && password ? this.currentPassword : null;
			return updateUser(this.account.id, {
				name,
				email,
				role,
				password,
				currentPassword: current,
			});
		});
		if (!saved) return null;
		const created = this.account ? null : saved.id;
		this.reset(saved);
		// the user menu shows the new name
		if (this.own && session.user)
			session.user = { ...session.user, name, email };
		return created;
	}

	/** Deletes the user; whether it is gone. */
	async remove(): Promise<boolean> {
		const id = this.account!.id;
		return (await this.run(() => deleteUser(id).then(() => true))) ?? false;
	}

	/** Logs the user out everywhere, oneself except here. */
	async endSessions() {
		const id = this.account!.id;
		const account = await this.run(() =>
			endSessions(id).then(() => getUser(id)),
		);
		if (account) this.account = account;
	}

	/**
	 * The invite or reset link of the user: mailed, or copied to paste
	 * somewhere; the note says which and how long it works.
	 */
	async link(send: boolean) {
		const id = this.account!.id;
		const link = await this.run(() => passwordLink(id, send));
		if (!link) return;
		const what = link.kind === 'invite' ? 'The invite' : 'The reset link';
		const until = new Date(link.expiresAt).toLocaleString();
		if (send) {
			this.linkNote = `${what} is sent to ${this.account!.email}, it works until ${until}.`;
		} else {
			await navigator.clipboard.writeText(link.url);
			this.linkNote = `${what} is copied, it works once until ${until}.`;
		}
	}

	/**
	 * Logs in as the user here, or copies the link for another browser
	 * (a private window keeps the own login).
	 */
	async loginAs(copy: boolean) {
		const id = this.account!.id;
		const link = await this.run(() => loginAsLink(id));
		if (!link) return;
		if (copy) {
			await navigator.clipboard.writeText(link.url);
			this.linkNote =
				'The login link is copied: open it once, in the next 10 minutes.';
			return;
		}
		const token = link.url.split('/').pop()!;
		await this.run(() => session.loginAs(token));
		if (session.impersonatedBy) location.assign('/admin');
	}
}
