<script module lang="ts">
	import { getLink } from './api';

	/** Anyone with the link, it is the key. */
	export const isPublic = true;

	/** What the link is for; used or expired, the page says so. */
	export async function loadProps(props: { token: string }) {
		try {
			const link = await getLink(props.token);
			return { link, problem: '' };
		} catch (e: any) {
			return { link: null, problem: e.message as string };
		}
	}
</script>

<script lang="ts">
	import Icon from 'zipp-ui/Icon.svelte';
	import { session } from '../core/session.svelte';
	import { setPassword, type LinkInfo } from './api';
	import { Busy } from '../core/busy.svelte';
	import AuthLayout from './AuthLayout.svelte';
	import PasswordInput from './PasswordInput.svelte';
	import { t } from '../i18n/index.svelte';

	/**
	 * The link of an invite or a password reset: choose a password (an
	 * invited user their name too), then logged in.
	 */
	let {
		token,
		link,
		problem,
	}: { token: string; link: LinkInfo | null; problem: string } = $props();

	const invite = $derived(link?.kind === 'invite');
	let name = $state('');
	let password = $state('');
	const action = new Busy();
	// "At least 8 characters" rather than "Some fields are invalid"
	const error = $derived(Object.values(action.fields)[0] ?? action.error);

	function onsubmit(e: SubmitEvent) {
		e.preventDefault();
		action.run(async () => {
			const auth = await setPassword(
				token,
				password,
				invite ? name : null,
			);
			session.use(auth);
			location.assign('/admin');
		});
	}
</script>

<AuthLayout>
	{#if link}
		<form {onsubmit}>
			<div class="intro">
				<h1>
					{invite ? t('auth.welcome') : t('auth.newPasswordTitle')}
				</h1>
				<p class="muted">
					{invite
						? t('auth.inviteIntro', { email: link.email })
						: t('auth.resetIntro', { email: link.email })}
				</p>
			</div>

			{#if invite}
				<label class="stacked">
					{t('auth.yourName')}
					<input
						autocomplete="name"
						placeholder={t('common.namePlaceholder')}
						bind:value={name}
					/>
				</label>
			{/if}

			<label class="stacked">
				{t('common.password')}
				<PasswordInput
					autocomplete="new-password"
					placeholder={t('common.passwordPlaceholder')}
					bind:value={password}
				/>
			</label>

			{#if error}<p class="error">{error}</p>{/if}

			<button class="primary submit" disabled={action.busy}>
				{invite ? t('auth.setAndLogIn') : t('auth.saveAndLogIn')}
				<Icon name="arrow-right" />
			</button>
		</form>
	{:else}
		<div>
			<div class="intro">
				<h1>{t('auth.linkBroken')}</h1>
				<p class="muted">
					{t('auth.linkProblem', { problem })}
				</p>
			</div>
			<p class="note">
				<a href="/admin/forgot">{t('common.newPassword')}</a>
				·
				<a href="/admin/login">{t('auth.logIn')}</a>
			</p>
		</div>
	{/if}
</AuthLayout>
