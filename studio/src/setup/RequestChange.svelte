<script lang="ts">
	import Popover from 'zipp-ui/Popover.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import { Busy } from '../core/busy.svelte';
	import { createRequest } from './api';

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

<Popover bind:open align="end" width="22rem" label="Request a change">
	{#snippet trigger({ toggle })}
		<button onclick={toggle} title="Ask the developers for a change here">
			<Icon name="edit" />
			Request a change
		</button>
	{/snippet}
	<form class="request" onsubmit={send}>
		{#if sent}
			<p class="note">Sent, the developers see it under Requests.</p>
		{/if}
		<label class="stacked">
			What do you need on {context}?
			<textarea
				rows="4"
				bind:value={text}
				placeholder="I need a subtitle field"
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
				Close
			</button>
			<button class="primary" disabled={action.busy || !text.trim()}>
				Send
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
