import { inLayout } from '../core/router.svelte';
import { Shell } from '../shell/layouts';

export function registerUsers() {
	inLayout(Shell, [['/users', () => import('./Users.svelte')]]);
}
