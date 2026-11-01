<script lang="ts">
	import { untrack } from 'svelte';
	import Table from 'zipp-ui/Table.svelte';
	import { Busy } from '../core/busy.svelte';
	import { time } from '../core/format';
	import { setRequestDone, type ChangeRequest } from './api';

	/**
	 * What editors asked to change in the content model, for admins: the
	 * open ones first, ticked off once the code does it.
	 */
	let { requests: loaded }: { requests: ChangeRequest[] } = $props();

	let requests = $state(untrack(() => loaded));
	const action = new Busy();

	async function toggle(r: ChangeRequest, done: boolean) {
		const ok = await action.run(() => setRequestDone(r.id, done));
		if (ok !== undefined) r.done = done;
	}
</script>

{#if action.error}<p class="error">{action.error}</p>{/if}

<div class="card">
	<p class="muted">
		Editors ask for changes with “Request a change” on the pages of Setup.
		Admins get a mail of each when mails are set up.
	</p>
	<Table
		rows={requests}
		key={r => r.id}
		columns={['Done', 'Request', 'Where', 'Who', 'When']}
		empty="No requests yet"
	>
		{#snippet row(r)}
			<td>
				<input
					type="checkbox"
					aria-label="Done"
					checked={r.done}
					disabled={action.busy}
					onchange={e => toggle(r, e.currentTarget.checked)}
				/>
			</td>
			<td class="text" class:done={r.done}>{r.text}</td>
			<td><a href={r.page}>{r.context}</a></td>
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
