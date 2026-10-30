import { router } from '../core/router.svelte';

/** The login, without the frame. */
export function registerAuth() {
	router.register('/admin/login', () => import('./Login.svelte'));
}
