<script lang="ts">
	import Popover from 'zipp-ui/Popover.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import { Busy } from '../core/busy.svelte';
	import { createRequest } from './api';
	import { t } from '../i18n/index.svelte';

	/**
	 * Setup is read only: here an editor writes what they need changed on
	 * the page they look at, like a field on an entry type. Admins find it
	 * under Requests.
	 */
	let { page, context }: { page: string; context: string } = $props();

	let open = $state(false);
	let text = $state('');
	let sent = $state(false);
	const action = new Busy();

	async function send(e: SubmitEvent) {
		e.preventDefault();
		const done = await action.run(() =>
			createRequest({ page, context, text }),
		);
		if (done) {
			text = '';
			sent = true;
		}
	}

	// the note of the last one goes when it closes
	$effect(() => {
		if (!open) sent = false;
	});
</script>

<Popover bind:open align="end" width="22rem" label={t('setup.requestChange')}>
	{#snippet trigger({ toggle })}
		<button onclick={toggle} title={t('setup.requestChangeTitle')}>
			<Icon name="edit" />
			{t('setup.requestChange')}
		</button>
	{/snippet}
	<form class="request" onsubmit={send}>
		{#if sent}
			<p class="note">{t('setup.requestSent')}</p>
		{/if}
		<label class="stacked">
			{t('setup.whatDoYouNeed', { context })}
			<textarea
				rows="4"
				bind:value={text}
				placeholder={t('setup.requestPlaceholder')}
			></textarea>
			{#if action.fields.text}
				<span class="error">{action.fields.text}</span>
			{/if}
		</label>
		{#if action.error && !action.fields.text}
			<p class="error">{action.error}</p>
		{/if}
		<div class="buttons">
			<button type="button" class="light" onclick={() => (open = false)}>
				{t('common.close')}
			</button>
			<button class="primary" disabled={action.busy || !text.trim()}>
				{t('setup.send')}
			</button>
		</div>
	</form>
</Popover>

<style lang="scss">
	.request {
		display: flex;
		flex-direction: column;
		gap: 0.625rem;
		padding: 0.5rem;
	}

	p {
		margin: 0;
	}

	.buttons {
		display: flex;
		justify-content: flex-end;
		gap: 0.5rem;
	}
</style>
