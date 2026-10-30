import { ApiError } from './client';

/**
 * An action of a page or panel: busy while it runs, its error shown
 * where it happened, the errors of fields by their path. `run` resolves
 * with the result, undefined on an error.
 */
export class Busy {
	busy = $state(false);
	error = $state('');
	fields = $state<Record<string, string>>({});

	async run<T>(f: () => Promise<T>): Promise<T | undefined> {
		this.busy = true;
		this.error = '';
		this.fields = {};
		try {
			return await f();
		} catch (e: any) {
			this.error = e.message;
			if (e instanceof ApiError)
				for (const f of e.fields) this.fields[f.path] = f.message;
		} finally {
			this.busy = false;
		}
	}
}
