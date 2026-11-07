<script lang="ts">
	import type { Account } from './api';
	import type { UserForm } from './userForm.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import { time } from '../core/format';
	import { t } from '../i18n/index.svelte';

	/**
	 * The side of an account: when it was made and used, its sessions,
	 * and for admins the invite or reset link and logging in as it. A
	 * member without the studio (`studio` false) gets links to the
	 * website and can't be logged in as.
	 */
	let {
		form,
		account,
		admin,
		title,
		studio,
	}: {
		form: UserForm;
		account: Account;
		admin: boolean;
		title: string;
		studio: boolean;
	} = $props();

	function endSessions() {
		const what = form.own
			? t('users.confirmEndOwn')
			: t('users.confirmEnd', { name: title });
		if (confirm(what)) form.endSessions();
	}
</script>

<dl>
	<dt>{t('users.created')}</dt>
	<dd>{time(account.createdAt)}</dd>
	<dt>{t('users.lastLogin')}</dt>
	<dd>
		{account.lastLoginAt ? time(account.lastLoginAt) : t('common.never')}
	</dd>
	<dt>{t('users.loggedIn')}</dt>
	<dd>
		{account.sessions === 0
			? t('users.notLoggedIn')
			: t('users.browsers', { n: account.sessions })}
	</dd>
</dl>
{#if account.sessions > (form.own ? 1 : 0)}
	<button type="button" onclick={endSessions} disabled={form.busy}>
		<Icon name="logout" />
		{form.own ? t('users.endOwn') : t('users.end')}
	</button>
{/if}
{#if admin && !form.own}
	<div class="access">
		<h3>{t('users.access')}</h3>
		{#if account.invited}
			<p class="note">{t('users.invitedNote')}</p>
		{/if}
		{#if !studio}
			<p class="note">{t('users.websiteNote')}</p>
		{/if}
		<div class="buttons">
			<button onclick={() => form.link(true)} disabled={form.busy}>
				<Icon name="mail" />
				{account.invited
					? t('users.sendInviteAgain')
					: t('users.sendReset')}
			</button>
			<button
				class="light"
				onclick={() => form.link(false)}
				disabled={form.busy}
			>
				<Icon name="copy-url" />
				{account.invited ? t('users.copyInvite') : t('users.copyReset')}
			</button>
			{#if account.role !== null && studio}
				<button
					onclick={() => form.loginAs(false)}
					disabled={form.busy}
				>
					<Icon name="users" />
					{t('users.loginAs', { name: title })}
				</button>
				<button
					class="light"
					onclick={() => form.loginAs(true)}
					disabled={form.busy}
					title={t('users.copyLoginTitle')}
				>
					<Icon name="copy-url" />
					{t('users.copyLogin')}
				</button>
			{/if}
		</div>
		{#if form.linkNote}
			<p class="note" role="status">{form.linkNote}</p>
		{/if}
	</div>
{/if}

<style lang="scss">
	dl {
		display: grid;
		grid-template-columns: max-content 1fr;
		gap: 0.375rem 1rem;
		margin: 0;
		font-size: var(--fs-md);
	}

	dt {
		color: var(--muted);
	}

	dd {
		margin: 0;
	}

	button {
		align-self: flex-start;
	}

	.access {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		padding-top: 0.875rem;
		border-top: 1px solid var(--line);

		p {
			margin: 0;
		}
	}

	.buttons {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		gap: 0.375rem;
	}
</style>
