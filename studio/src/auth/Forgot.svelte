<script module lang="ts">
	/** Anyone may ask for a new password. */
	export const isPublic = true;
</script>

<script lang="ts">
	import Icon from 'zipp-ui/Icon.svelte';
	import { request } from '../core/client';
	import AuthLayout from './AuthLayout.svelte';

	/**
	 * A forgotten password: the email gets a link to choose a new one,
	 * if there is an account with it; the page says the same either way.
	 */
	let email = $state('');
	let sent = $state(false);
	let error = $state('');
	let busy = $state(false);

	async function onsubmit(e: SubmitEvent) {
		e.preventDefault();
		error = '';
		busy = true;
		try {
			await request('POST', '/auth/forgot', { email });
			sent = true;
		} catch (e: any) {
			error = e.message;
		} finally {
			busy = false;
		}
	}
</script>

<AuthLayout>
	<form {onsubmit}>
		<div class="intro">
			<h1>Forgot your password?</h1>
			<p class="muted">
				{sent
					? `If ${email} has an account, a mail with a link is on its way. The link works for an hour.`
					: 'Your email gets a link to choose a new one.'}
			</p>
		</div>

		{#if !sent}
			<label class="stacked">
				Email
				<input
					type="email"
					autocomplete="username"
					placeholder="name@example.com"
					bind:value={email}
					required
				/>
			</label>

			{#if error}<p class="error">{error}</p>{/if}

			<button class="primary submit" disabled={busy}>
				Send the link
				<Icon name="arrow-right" />
			</button>
		{/if}

		<p class="note"><a href="/admin/login">Back to the login</a></p>
	</form>
</AuthLayout>
