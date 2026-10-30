<script lang="ts">
	import { onMount } from 'svelte';
	import { request } from '../core/client';
	import { Busy } from '../core/busy.svelte';
	import { session } from '../core/session.svelte';

	/**
	 * How the cms sends mails, for admins: the SMTP server of `serve
	 * --smtp-url` or none (then mails are only logged), and a test mail
	 * to oneself.
	 */
	type MailStatus = {
		configured: boolean;
		server: string | null;
		from: string;
	};

	let status = $state<MailStatus | null>(null);
	let sent = $state(false);
	const action = new Busy();

	onMount(() => {
		action.run(async () => {
			status = await request<MailStatus>('GET', '/mail');
		});
	});

	function test() {
		sent = false;
		action.run(async () => {
			await request('POST', '/mail/test');
			sent = true;
		});
	}
</script>

<div class="card" id="mail">
	<h2>Mail</h2>
	{#if status?.configured}
		<p>
			Mails go to <code>{status.server}</code>
			from
			<code>{status.from}</code>
			.
		</p>
		<button class="light" disabled={action.busy} onclick={test}>
			Send a test mail to me
		</button>
		{#if sent}
			<span class="note">Sent to {session.user?.email}.</span>
		{/if}
	{:else if status}
		<p class="muted">
			No SMTP server: mails are only logged. Start the cms with
			<code>serve --smtp-url</code>
			and
			<code>--mail-from</code>
			.
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
