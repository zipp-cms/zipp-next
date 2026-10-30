import { registerAi } from '../ai/routes';
import { registerAssets } from '../assets/routes';
import { registerAuth } from '../auth/routes';
import { inLayout } from '../core/router.svelte';
import { registerEntries } from '../entries/routes';
import { registerSetup } from '../setup/routes';
import { registerUsers } from '../users/routes';
import { Shell } from './layouts';

/** Every page of the studio: the areas register their own. */
export function registerRoutes() {
	inLayout(Shell, [
		// the start and the content without a section: the first page
		['/admin', () => import('./Start.svelte')],
		['/admin/entries', () => import('./Start.svelte')],
		[/\/admin\/docs\/(?<handle>[^/]+)/, () => import('./Doc.svelte')],
	]);
	registerAuth();
	registerEntries();
	registerAssets();
	registerSetup();
	registerAi();
	registerUsers();
}
