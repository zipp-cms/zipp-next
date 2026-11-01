<script module lang="ts">
	/** Anyone may open the login. */
	export const isPublic = true;
</script>

<script lang="ts">
	import { session } from '../core/session.svelte';
	import { router } from '../core/router.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import AuthLayout from './AuthLayout.svelte';
	import { t } from '../i18n/index.svelte';

	/** The login, a link to a new password if it is forgotten. */
	let email = $state('');
	let password = $state('');
	let remember = $state(true);
	let showPassword = $state(false);
	let error = $state('');
	let busy = $state(false);

	async function onsubmit(e: SubmitEvent) {
		e.preventDefault();
		error = '';
		busy = true;
		try {
			await session.login(email, password, remember);
			// back to the page that wanted the login
			const url = new URLSearchParams(location.search).get('url');
			const back =
				url?.startsWith('/admin') && !url.startsWith('/admin/login');
			router.open(back ? url! : '/admin', {}, { history: 'replace' });
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
			<span class="password">
				<input
					type={showPassword ? 'text' : 'password'}
					autocomplete="current-password"
					bind:value={password}
					required
				/>
				<button
					type="button"
					class="icon"
					title={showPassword
						? t('auth.hidePassword')
						: t('auth.showPassword')}
					onclick={() => (showPassword = !showPassword)}
				>
					<Icon name={showPassword ? 'hide' : 'view'} />
				</button>
			</span>
		</label>

		<label class="check">
			<input type="checkbox" bind:checked={remember} />
			{t('auth.stayLoggedIn')}
		</label>

		{#if error}<p class="error">{error}</p>{/if}

		<button class="primary submit" disabled={busy}>
			{t('auth.logIn')}
			<Icon name="arrow-right" />
		</button>

		<p class="note">
			<a href="/admin/forgot">{t('auth.forgot')}</a>
		</p>
	</form>
</AuthLayout>
