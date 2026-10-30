import { inLayout } from '../core/router.svelte';
import { Shell } from '../shell/layouts';

/** The users for admins, a user's account (also one's own) and a new one. */
export function registerUsers() {
	inLayout(Shell, [
		['/admin/users', () => import('./Users.svelte')],
		[/\/admin\/users\/(?<id>\d+|new)/, () => import('./User.svelte')],
	]);
}
