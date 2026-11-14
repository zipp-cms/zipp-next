/** @import { FieldData, FieldError, FormData, PageData } from './types' */

/**
 * @typedef {object} FormOptions
 * @property {string} site the handle of the site the form is sent from
 * @property {Record<string, string>} [messages] texts of the errors by
 * their code (`required`, `email`, …) and `failed` when the form can't
 * send; the cms's English ones else
 */

const FAILED = 'The form could not send.';

/**
 * A form of zipp-forms with what the visitor typed: `values` by handle,
 * `errors` by handle after sending, `sent` with the message after
 * sending, `page` the step on screen. Made by `loadForm`. Plain
 * JavaScript: vite compiles a dependency's `.svelte.js` as it is.
 */
export class ZippForm {
	/** @type {Record<string, string>} */
	values = $state({});
	/** @type {Record<string, string>} */
	errors = $state({});
	sending = $state(false);
	/** @type {string | null} the message after sending, null before */
	sent = $state(null);
	/** why the form couldn't send, empty if it could */
	failed = $state('');
	/** the index of the page on screen */
	page = $state(0);

	/**
	 * @param {FormData} data
	 * @param {FormOptions} opts
	 */
	constructor(data, opts) {
		/** @readonly */
		this.data = data;
		/** @readonly */
		this.site = opts.site;
		this.messages = opts.messages ?? {};
		this.reset();
	}

	/** @returns {PageData[]} */
	get pages() {
		return this.data.pages;
	}

	/** The fields of the page on screen. */
	get fields() {
		return this.data.pages[this.page].fields;
	}

	/** Whether the page on screen is the last, which sends. */
	get isLast() {
		return this.page === this.data.pages.length - 1;
	}

	/**
	 * An id for a field's input, unique when a page shows the form twice.
	 * @param {FieldData} field
	 */
	inputId(field) {
		return `zipp-form-${this.data.id}-${field.handle}`;
	}

	/** Empty values, the message gone: the form as it was loaded. */
	reset() {
		/** @type {Record<string, string>} */
		const values = { [this.data.honeypot]: '' };
		for (const p of this.data.pages)
			for (const f of p.fields) values[f.handle] = '';
		this.values = values;
		this.errors = {};
		this.sent = null;
		this.failed = '';
		this.page = 0;
	}

	/**
	 * Goes on: the next page once the cms finds this one's values right,
	 * or sends the form on the last.
	 * @returns {Promise<boolean>} whether it went on
	 */
	async next() {
		if (this.isLast) return this.submit();
		const ok = await this.post(this.page);
		if (ok) this.page += 1;
		return ok;
	}

	/** The page before, what was typed stays. */
	back() {
		if (this.page === 0) return;
		this.errors = {};
		this.failed = '';
		this.page -= 1;
	}

	/**
	 * Sends the values through the frontend's `/forms/{id}` (see
	 * `formRoutes`); with mistakes on another page it goes there.
	 * @returns {Promise<boolean>} whether it went
	 */
	async submit() {
		const ok = await this.post(null);
		const wrong = this.data.pages.findIndex(p =>
			p.fields.some(f => this.errors[f.handle]),
		);
		if (wrong >= 0) this.page = wrong;
		return ok;
	}

	/**
	 * Checks the values of a page (stores nothing) or sends them all.
	 * @param {number | null} page
	 */
	async post(page) {
		this.sending = true;
		this.errors = {};
		this.failed = '';
		try {
			const site = encodeURIComponent(this.site);
			const step = page === null ? '' : `&page=${page}`;
			const resp = await fetch(
				`/forms/${this.data.id}?site=${site}${step}`,
				{
					method: 'POST',
					headers: { 'content-type': 'application/json' },
					body: JSON.stringify(this.values),
				},
			);
			const body = await resp.json().catch(() => ({}));
			if (body.success) {
				if (page === null) this.sent = body.message;
				return true;
			}
			if (body.errors) {
				/** @type {FieldError[]} */
				const errors = body.errors;
				this.errors = Object.fromEntries(
					errors.map(e => [
						e.path,
						this.messages[e.code] ?? e.message,
					]),
				);
			} else {
				this.failed = this.messages.failed ?? body.message ?? FAILED;
			}
		} catch {
			this.failed = this.messages.failed ?? FAILED;
		} finally {
			this.sending = false;
		}
		return false;
	}
}
