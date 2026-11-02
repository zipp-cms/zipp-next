<script module lang="ts">
	/** Anyone may ask for a new password. */
	export const isPublic = true;
</script>

<script lang="ts">
	import Icon from 'zipp-ui/Icon.svelte';
	import { forgotPassword } from './api';
	import AuthLayout from './AuthLayout.svelte';
	import { t } from '../i18n/index.svelte';

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
			await forgotPassword(email);
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
			<h1>{t('auth.forgot')}</h1>
			<p class="muted">
				{sent ? t('auth.forgotSent', { email }) : t('auth.forgotIntro')}
			</p>
		</div>

		{#if !sent}
			<label class="stacked">
				{t('common.email')}
				<input
					type="email"
					autocomplete="username"
					placeholder={t('auth.emailPlaceholder')}
					bind:value={email}
					required
				/>
			</label>

			{#if error}<p class="error">{error}</p>{/if}

			<button class="primary submit" disabled={busy}>
				{t('auth.sendLink')}
				<Icon name="arrow-right" />
			</button>
		{/if}

		<p class="note"><a href="/admin/login">{t('auth.backToLogin')}</a></p>
	</form>
</AuthLayout>
