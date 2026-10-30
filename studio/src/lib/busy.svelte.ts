/**
 * An action of a page or panel: busy while it runs, its error shown
 * where it happened. `run` resolves with the result, undefined on an
 * error.
 */
export class Busy {
	busy = $state(false);
	error = $state('');

	async run<T>(f: () => Promise<T>): Promise<T | undefined> {
		this.busy = true;
		this.error = '';
		try {
			return await f();
		} catch (e: any) {
			this.error = e.message;
		} finally {
			this.busy = false;
		}
	}
}
