import { inLayout } from '../core/router.svelte';
import { Shell } from '../shell/layouts';

export function registerUsers() {
	inLayout(Shell, [['/admin/users', () => import('./Users.svelte')]]);
}
