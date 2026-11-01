import { inLayout } from '../core/router.svelte';
import { Shell } from '../shell/layouts';

/** The tokens of frontends for admins, and a new one. */
export function registerGraphql() {
	inLayout(Shell, [
		['/admin/graphql', () => import('./Tokens.svelte')],
		['/admin/graphql/new', () => import('./NewToken.svelte')],
	]);
}
