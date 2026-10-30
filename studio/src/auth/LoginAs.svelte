<script module lang="ts">
	/** The link of an admin, it is the key. */
	export const isPublic = true;
</script>

<script lang="ts">
	import { onMount } from 'svelte';
	import { session } from '../core/session.svelte';
	import AuthLayout from './AuthLayout.svelte';

	/**
	 * A link that logs in as a user, which an admin made on the user's
	 * page: used once, it opens the studio as that user. The admin's own
	 * session waits for "Back to your account".
	 */
	let { token }: { token: string } = $props();
	let error = $state('');

	onMount(async () => {
		try {
			await session.loginAs(token);
			// a new start, the schema is the user's role's
			location.assign('/admin');
		} catch (e: any) {
			error = e.message;
		}
	});
</script>

<AuthLayout>
	<div>
		<div class="intro">
			<h1>{error ? "This link doesn't work" : 'Logging in…'}</h1>
			{#if error}<p class="muted">{error}</p>{/if}
		</div>
		{#if error}
			<p class="note"><a href="/admin">To the studio</a></p>
		{/if}
	</div>
</AuthLayout>
