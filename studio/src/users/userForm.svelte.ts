import { Busy } from '../core/busy.svelte';
import { session } from '../core/session.svelte';
import {
	createUser,
	deleteUser,
	endSessions,
	getUser,
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
			if (!this.account)
				return createUser({
					name,
					email,
					role,
					password: password ?? '',
				});
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
}
