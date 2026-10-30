import { router } from '../core/router.svelte';

/**
 * The login and the pages of links without the frame: a forgotten
 * password, the link of an invite or reset, logging in as a user.
 */
export function registerAuth() {
	router.register('/admin/login', () => import('./Login.svelte'));
	router.register('/admin/forgot', () => import('./Forgot.svelte'));
	router.register(
		/\/admin\/password\/(?<token>[0-9a-f]+)/,
		() => import('./SetPassword.svelte'),
	);
	router.register(
		/\/admin\/login-as\/(?<token>[0-9a-f]+)/,
		() => import('./LoginAs.svelte'),
	);
}
