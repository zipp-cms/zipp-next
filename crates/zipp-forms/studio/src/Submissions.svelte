<script>
	import PageTop from 'zipp-ui/PageTop.svelte';
	import Table from 'zipp-ui/Table.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import { SubmissionsList } from './submissions.svelte.js';
	import { texts } from './texts.js';

	/**
	 * The submissions of the forms, the newest first: a row shows the
	 * first values, a click opens all of them.
	 */
	let { schema, host } = $props();
	const tx = texts();

	// svelte-ignore state_referenced_locally
	const list = new SubmissionsList(host);
	list.init();

	/** @type {number | null} */
	let open = $state(null);

	const siteName = handle =>
		schema.sites.find(s => s.handle === handle)?.name ?? handle;
	const when = ms =>
		new Date(ms).toLocaleString(document.documentElement.lang, {
			dateStyle: 'medium',
			timeStyle: 'short',
		});
	// the first values that say who it is
	const summary = s =>
		s.answers
			.map(a => a.value)
			.filter(Boolean)
			.slice(0, 3)
			.join(' · ') || tx.empty;

	function pick(e) {
		const v = e.currentTarget.value;
		list.form = v ? Number(v) : null;
		open = null;
		list.load();
	}

	function remove(s) {
		if (!confirm(tx.confirmDelete)) return;
		list.delete(s.id);
	}
</script>

<PageTop title={tx.title} note={tx.note}>
	{#snippet actions()}
		{#if list.forms.length > 1}
			<select value={list.form ?? ''} onchange={pick}>
				<option value="">{tx.allForms}</option>
				{#each list.forms as f (f.id)}
					<option value={f.id}>{f.title}</option>
				{/each}
			</select>
		{/if}
	{/snippet}
</PageTop>

{#if list.error}<p class="error">{list.error}</p>{/if}

<div class="card">
	<Table
		rows={list.submissions}
		key={s => s.id}
		columns={[tx.date, tx.form, tx.site, tx.content]}
		empty={list.loading ? '…' : tx.none}
		isOpen={s => s.id === open}
		ontoggle={s => (open = open === s.id ? null : s.id)}
	>
		{#snippet row(s)}
			<td class="nowrap">{when(s.createdAt)}</td>
			<td>{s.form}</td>
			<td>{siteName(s.site)}</td>
			<td class="truncate">{summary(s)}</td>
		{/snippet}
		{#snippet detail(s)}
			<dl>
				{#each s.answers as a (a.handle)}
					<dt>{a.label}</dt>
					<dd>{a.value || tx.empty}</dd>
				{/each}
			</dl>
			<button class="danger" onclick={() => remove(s)}>
				<Icon name="delete" />
				{tx.delete}
			</button>
		{/snippet}
	</Table>
</div>

{#if list.submissions.length < list.total}
	<p class="more">
		<button class="light" disabled={list.loading} onclick={() => list.more()}>
			{tx.more}
		</button>
	</p>
{/if}

<style lang="scss">
	@use 'zipp-ui/vars.scss' as *;

	.nowrap {
		white-space: nowrap;
	}

	dl {
		display: grid;
		grid-template-columns: minmax(8rem, max-content) 1fr;
		gap: 0.375rem 1.5rem;
		margin: 0 0 1rem;
	}

	dt {
		color: var(--muted);
	}

	dd {
		margin: 0;
		white-space: pre-wrap;
		overflow-wrap: anywhere;
	}

	.more {
		text-align: center;
	}

	@include max-tablet {
		dl {
			grid-template-columns: 1fr;
		}

		dd {
			margin-bottom: 0.5rem;
		}
	}
</style>
