import { inLayout } from '../core/router.svelte';
import { Shell } from '../shell/layouts';
import { SETUP_TABS } from '../shell/nav';

// `/admin/setup/types/page`: a tab and maybe a handle of it
const TAB = new RegExp(
	`/admin/setup/(?<tab>${SETUP_TABS.map(([t]) => t).join('|')})(?:/(?<handle>[^/]+))?`,
);

/** Setup: a tab, or a section, entry type or block of its own. */
export function registerSetup() {
	const setup = () => import('./Setup.svelte');
	inLayout(Shell, [
		['/admin/setup', setup],
		[TAB, setup],
	]);
}
