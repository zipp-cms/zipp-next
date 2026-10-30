import { listAssets, type Asset } from './api';
import { flyout } from '../core/flyout.svelte';

/**
 * The assets of the ids a field holds, by id: loaded when an id is new,
 * all again after the flyout closed (a new title or focal point). `get`
 * is undefined while loading and null for an asset that is gone. Call
 * it in a component, it uses an effect.
 */
export function chosenAssets(ids: () => number[]) {
	const cache = $state<Record<number, Asset | null>>({});
	let closed = -1;
	$effect(() => {
		const want = ids();
		const again = flyout.saved !== closed;
		closed = flyout.saved;
		const missing = again ? want : want.filter(id => !(id in cache));
		if (!missing.length) return;
		listAssets({ ids: missing })
			.then(found => {
				for (const id of missing)
					cache[id] = found.find(a => a.id === id) ?? null;
			})
			// stays as it was, the next change tries again
			.catch(() => {});
	});
	return { get: (id: number): Asset | null | undefined => cache[id] };
}
