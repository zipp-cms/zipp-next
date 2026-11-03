<script module lang="ts">
	/** Anyone may open the login. */
	export const isPublic = true;
</script>

<script lang="ts">
	import { session } from '../core/session.svelte';
	import { router } from '../core/router.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import Checkbox from 'zipp-ui/Checkbox.svelte';
	import AuthLayout from './AuthLayout.svelte';
	import PasswordInput from './PasswordInput.svelte';
	import { Busy } from '../core/busy.svelte';
	import { t } from '../i18n/index.svelte';

	/** The login, a link to a new password if it is forgotten. */
	let email = $state('');
	let password = $state('');
	let remember = $state(true);
	const action = new Busy();

	function onsubmit(e: SubmitEvent) {
		e.preventDefault();
		action.run(async () => {
			await session.login(email, password, remember);
			// back to the page that wanted the login
			const url = new URLSearchParams(location.search).get('url');
			const back =
				url?.startsWith('/admin') && !url.startsWith('/admin/login');
			router.open(back ? url! : '/admin', {}, { history: 'replace' });
		});
	}
</script>

<AuthLayout>
	<form {onsubmit}>
		<div class="intro">
			<h1>{t('auth.logIn')}</h1>
			<p class="muted">{t('auth.welcomeBack')}</p>
		</div>

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

		<label class="stacked">
			{t('common.password')}
			<PasswordInput
				autocomplete="current-password"
				bind:value={password}
			/>
		</label>

		<Checkbox bind:checked={remember}>{t('auth.stayLoggedIn')}</Checkbox>

		{#if action.error}<p class="error">{action.error}</p>{/if}

		<button class="primary submit" disabled={action.busy}>
			{t('auth.logIn')}
			<Icon name="arrow-right" />
		</button>

		<p class="note">
			<a href="/admin/forgot">{t('auth.forgot')}</a>
		</p>
	</form>
</AuthLayout>
