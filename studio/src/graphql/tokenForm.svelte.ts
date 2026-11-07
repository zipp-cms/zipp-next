import { Busy } from '../core/busy.svelte';
import { createToken } from './api';

/**
 * A new token: its name, its schema and until when. Saving shows the
 * token once, see `created`.
 */
export class TokenForm extends Busy {
	name = $state('');
	/** the handle of a schema of the code */
	schema = $state('');
	/** `2026-12-31`, empty for never */
	expires = $state('');
	/** the token itself after saving, it doesn't show again */
	created = $state('');
	copied = $state(false);

	constructor(schema: string) {
		super();
		this.schema = schema;
	}

	async save() {
		const expiresAt = this.expires
			? // to the end of the day it names
				new Date(this.expires + 'T23:59:59').getTime()
			: null;
		const created = await this.run(() =>
			createToken({
				name: this.name,
				schema: this.schema,
				expiresAt,
			}),
		);
		if (created) this.created = created.token;
	}

	async copy() {
		await navigator.clipboard.writeText(this.created);
		this.copied = true;
	}
}
