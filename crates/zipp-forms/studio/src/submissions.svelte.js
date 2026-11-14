// The state of the page Submissions: the forms to pick from, the loaded
// submissions of the picked one (or all), more on request.

export class SubmissionsList {
	/** @type {{ id: number, title: string }[]} */
	forms = $state([]);
	/** @type {number | null} */
	form = $state(null);
	/** @type {any[]} */
	submissions = $state([]);
	total = $state(0);
	loading = $state(false);
	error = $state('');

	/** @param {any} host */
	constructor(host) {
		this.host = host;
	}

	async init() {
		await this.run(async () => {
			const forms = await this.host.request(
				'GET',
				'/entries?section=forms&limit=500&order=title',
			);
			this.forms = forms.entries.map(e => ({ id: e.id, title: e.title }));
		});
		await this.load();
	}

	/** The first page of the picked form. */
	async load() {
		await this.run(async () => {
			const resp = await this.request(0);
			this.submissions = resp.submissions;
			this.total = resp.total;
		});
	}

	async more() {
		await this.run(async () => {
			const resp = await this.request(this.submissions.length);
			this.submissions = [...this.submissions, ...resp.submissions];
			this.total = resp.total;
		});
	}

	/** @param {number} id */
	async delete(id) {
		await this.run(async () => {
			await this.host.request('DELETE', `/forms/submissions/${id}`);
			this.submissions = this.submissions.filter(s => s.id !== id);
			this.total -= 1;
		});
	}

	/** @param {number} offset */
	request(offset) {
		const form = this.form === null ? '' : `&form=${this.form}`;
		return this.host.request(
			'GET',
			`/forms/submissions?offset=${offset}${form}`,
		);
	}

	/** @param {() => Promise<void>} fn */
	async run(fn) {
		this.loading = true;
		this.error = '';
		try {
			await fn();
		} catch (e) {
			this.error = e.message;
		} finally {
			this.loading = false;
		}
	}
}
