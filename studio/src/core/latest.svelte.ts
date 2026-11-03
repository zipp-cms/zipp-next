/** How long a search waits after the last key before it asks. */
const SEARCH_DELAY_MS = 250;

/**
 * Loads that can overlap, like a search as you type or a list that loads
 * again after a move: only the newest one's answer counts, an older one
 * would show old results. `loading` and `error` are the newest one's.
 */
export class Latest {
	loading = $state(false);
	error = $state('');
	#n = 0;
	#wait: ReturnType<typeof setTimeout> | undefined;

	/** Runs `load` now and gives `done` its answer, unless a newer started. */
	async run<T>(load: () => Promise<T>, done: (r: T) => void) {
		clearTimeout(this.#wait);
		const n = ++this.#n;
		this.loading = true;
		try {
			const r = await load();
			if (n !== this.#n) return;
			done(r);
			this.error = '';
		} catch (e) {
			if (n === this.#n) this.error = (e as Error).message;
		} finally {
			if (n === this.#n) this.loading = false;
		}
	}

	/** The same a moment after the last call: after typing. */
	later<T>(load: () => Promise<T>, done: (r: T) => void) {
		clearTimeout(this.#wait);
		this.loading = true;
		this.#wait = setTimeout(() => this.run(load, done), SEARCH_DELAY_MS);
	}

	/** Drops the waiting one and the answer of the one under way. */
	stop() {
		clearTimeout(this.#wait);
		this.#n++;
		this.loading = false;
	}
}
