import { listAssets, type Asset } from '../api';
import { flyout } from './flyout.svelte';

/**
 * The assets of these ids in their order, loaded again when the ids
 * change and after the flyout saved one (a new title or focal point).
 * Call it in a component, it uses an effect.
 */
export function chosenAssets(ids: () => number[]): { list: Asset[] } {
	const chosen = $state({ list: [] as Asset[] });
	$effect(() => {
		void flyout.saved;
		const want = ids();
		if (!want.length) return void (chosen.list = []);
		const key = want.join(',');
		listAssets({ ids: key }).then(a => {
			if (ids().join(',') === key) chosen.list = a;
		});
	});
	return chosen;
}
