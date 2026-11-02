<script lang="ts">
	import { untrack } from 'svelte';
	import Table from 'zipp-ui/Table.svelte';
	import { Busy } from '../core/busy.svelte';
	import { time } from '../core/format';
	import { setRequestDone, type ChangeRequest } from './api';
	import { t } from '../i18n/index.svelte';

	/**
	 * What editors asked to change in the content model, for admins: the
	 * open ones first, ticked off once the code does it.
	 */
	let { requests: loaded }: { requests: ChangeRequest[] } = $props();

	let requests = $state(untrack(() => loaded));
	const action = new Busy();

	async function toggle(r: ChangeRequest, box: HTMLInputElement) {
		const done = box.checked;
		const ok = await action.run(() => setRequestDone(r.id, done));
		if (ok !== undefined) r.done = done;
		// a failed save leaves r.done, the box has to go back itself
		else box.checked = r.done;
	}
</script>

{#if action.error}<p class="error">{action.error}</p>{/if}

<div class="card">
	<p class="muted">
		{t('setup.requestsIntro')}
	</p>
	<Table
		rows={requests}
		key={r => r.id}
		columns={[
			t('common.done'),
			t('setup.request'),
			t('setup.where'),
			t('setup.who'),
			t('setup.when'),
		]}
		empty={t('setup.noRequests')}
	>
		{#snippet row(r)}
			<td>
				<input
					type="checkbox"
					aria-label={t('common.done')}
					checked={r.done}
					disabled={action.busy}
					onchange={e => toggle(r, e.currentTarget)}
				/>
			</td>
			<td class="text" class:done={r.done}>{r.text}</td>
			<td>
				<!-- only a path of the studio becomes a link, like the server checks -->
				{#if r.page.startsWith('/admin/')}
					<a href={r.page}>{r.context}</a>
				{:else}
					{r.context}
				{/if}
			</td>
			<td>{r.user ?? '–'}</td>
			<td class="muted">{time(r.createdAt)}</td>
		{/snippet}
	</Table>
</div>

<style lang="scss">
	p {
		margin: 0 0 0.75rem;
	}

	.text {
		white-space: pre-line;
	}

	.done {
		color: var(--muted);
		text-decoration: line-through;
	}
</style>
