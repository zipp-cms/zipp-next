<script lang="ts">
	import { onMount } from 'svelte';
	import { mailStatus, sendTestMail, type MailStatus } from './api';
	import { Busy } from '../core/busy.svelte';
	import { session } from '../core/session.svelte';
	import { t } from '../i18n/index.svelte';

	/**
	 * How the cms sends mails, for admins: the SMTP server of `serve
	 * --smtp-url` or none (then mails are only logged), and a test mail
	 * to oneself.
	 */
	let status = $state<MailStatus | null>(null);
	let sent = $state(false);
	const action = new Busy();

	onMount(() => {
		action.run(async () => {
			status = await mailStatus();
		});
	});

	function test() {
		sent = false;
		action.run(async () => {
			await sendTestMail();
			sent = true;
		});
	}
</script>

<div class="card" id="mail">
	<h2>{t('setup.mail')}</h2>
	{#if status?.configured}
		{@const [a, b, c] = t('setup.mailsGoTo').split(/\s*\{\w+\}\s*/)}
		<p>
			{a}
			<code>{status.server}</code>
			{b}
			<code>{status.from}</code>
			{c}
		</p>
		<button class="light" disabled={action.busy} onclick={test}>
			{t('setup.sendTest')}
		</button>
		{#if sent}
			<span class="note">
				{t('setup.sentTo', { email: session.user?.email ?? '' })}
			</span>
		{/if}
	{:else if status}
		{@const [a, b, c] = t('setup.noSmtp').split(/\s*\{\w+\}\s*/)}
		<p class="muted">
			{a}
			<code>serve --smtp-url</code>
			{b}
			<code>--mail-from</code>
			{c}
		</p>
	{/if}
	{#if action.error}<p class="error">{action.error}</p>{/if}
</div>

<style lang="scss">
	p {
		margin: 0 0 0.75rem;
	}

	.note {
		margin-left: 0.5rem;
	}
</style>
