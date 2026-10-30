import { BASE, router } from '../core/router.svelte';

/** The login, without the frame. */
export function registerAuth() {
	router.register(BASE + '/login', () => import('./Login.svelte'));
}
