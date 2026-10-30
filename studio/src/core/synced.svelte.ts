import { untrack } from 'svelte';

/**
 * Values that go both ways between two owners, like a rich text and the
 * fields of a block in it: what one sends must not come back from the
 * other, or each change would go round in a loop.
 */

/** The last value as JSON, to tell a new value from the same one again. */
export class Last {
	private json: string;

	constructor(value?: unknown) {
		this.json = JSON.stringify(value);
	}

	/** Whether `value` is another one; it is the last one then. */
	changed(value: unknown): boolean {
		const json = JSON.stringify(value);
		if (json === this.json) return false;
		this.json = json;
		return true;
	}
}

/**
 * A copy of `outer` to edit in a component: changes from outside (an
 * undo) replace it, its changes go to `send`. Only in a component's
 * script, it uses effects.
 */
export function synced<T>(outer: () => T, send: (value: T) => void) {
	// a plain copy, the owners don't share objects
	const copy = (v: T) => $state.snapshot(v) as T;
	const state = $state({ value: copy(untrack(outer)) });
	const last = new Last(untrack(outer));

	$effect.pre(() => {
		const v = outer();
		if (last.changed(v)) state.value = copy(v);
	});
	$effect(() => {
		const v = $state.snapshot(state.value) as T;
		if (last.changed(v)) send(v);
	});
	return state;
}
