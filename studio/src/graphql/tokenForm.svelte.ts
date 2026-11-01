import { Busy } from '../core/busy.svelte';
import { createToken } from './api';

/**
 * A new token: its name, what it reads and until when. Saving shows the
 * token once, see `created`.
 */
export class TokenForm extends Busy {
	name = $state('');
	/** all sections or sites, else the chosen ones */
	allSections = $state(true);
	sections = $state<string[]>([]);
	allSites = $state(true);
	sites = $state<string[]>([]);
	previews = $state(false);
	/** `2026-12-31`, empty for never */
	expires = $state('');
	/** the token itself after saving, it doesn't show again */
	created = $state('');
	copied = $state(false);

	toggle(list: 'sections' | 'sites', value: string, on: boolean) {
		const rest = this[list].filter(v => v !== value);
		this[list] = on ? [...rest, value] : rest;
	}

	async save() {
		const expiresAt = this.expires
			? // to the end of the day it names
				new Date(this.expires + 'T23:59:59').getTime()
			: null;
		const created = await this.run(() =>
			createToken({
				name: this.name,
				sections: this.allSections ? null : this.sections,
				sites: this.allSites ? null : this.sites,
				previews: this.previews,
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
