import { inLayout } from '../core/router.svelte';
import { Shell } from '../shell/layouts';

/** The entries of a section and the editor of one, or of a new one. */
export function registerEntries() {
	inLayout(Shell, [
		[/\/entries\/(?<section>[^/]+)/, () => import('./List.svelte')],
		[
			/\/entries\/(?<section>[^/]+)\/(?<id>\d+|new)/,
			() => import('./edit/Edit.svelte'),
		],
	]);
}
