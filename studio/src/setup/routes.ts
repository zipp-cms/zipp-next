import { inLayout } from '../core/router.svelte';
import { Shell } from '../shell/layouts';

/** Setup: a tab, or a section, entry type or block of its own. */
export function registerSetup() {
	const setup = () => import('./Setup.svelte');
	inLayout(Shell, [
		['/admin/setup', setup],
		[
			/\/admin\/setup\/(?<tab>sections|types|blocks|sites)(?:\/(?<handle>[^/]+))?/,
			setup,
		],
	]);
}
