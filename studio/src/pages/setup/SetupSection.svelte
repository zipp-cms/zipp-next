<script lang="ts">
	import { listEntries, type Entry, type Schema } from '../../api';
	import { accessOf } from '../../lib/access';

	/**
	 * A section in the setup, read only: what it is and its entries with
	 * whether they are live in each site and have unpublished changes.
	 */
	let { schema, handle }: { schema: Schema; handle: string } = $props();

	const section = $derived(schema.sections.find(s => s.handle === handle));
	let entries = $state<Entry[]>([]);
	let published = $state<Record<number, Record<string, boolean>>>({});
	let drafts = $state<number[]>([]);
	let total = $state(0);
	let error = $state('');

	$effect(() => {
		if (!section) return;
		const params: Record<string, string> = {
			section: handle,
			site: schema.sites[0].handle,
			limit: '1000',
		};
		if (section.kind === 'structure') params.order = 'structure';
		listEntries(params)
			.then(r => {
				entries = r.entries;
				published = r.published;
				drafts = r.drafts;
				total = r.total;
			})
			.catch(e => (error = e.message));
	});

	// the depth of an entry in a structure, for indenting
	function depth(e: Entry): number {
		let d = 0;
		let parent = e.parentId;
		while (parent !== null && d < 10) {
			d++;
			parent = entries.find(x => x.id === parent)?.parentId ?? null;
		}
		return d;
	}

	const date = (ms: number) => new Date(ms).toLocaleDateString();
	const cap = (s: string) => s[0].toUpperCase() + s.slice(1);
</script>

{#if section}
	<div class="card">
		<h2>
			{section.name}
			<code>{section.handle}</code>
		</h2>
		<dl>
			<dt>Kind</dt>
			<dd>{section.kind}</dd>
			<dt>Entry types</dt>
			<dd>
				{#each section.types as t, i (t)}
					{i ? ', ' : ''}
					<a href="/admin/setup/types/{t}">{t}</a>
				{/each}
			</dd>
			<dt>URI</dt>
			<dd>
				<code>
					{section.uri ?? (section.type_uris ? '' : 'no pages')}
				</code>
				{#each Object.entries(section.type_uris ?? {}) as [t, u] (t)}
					<br />
					<small>
						{t}:
						<code>{u || '/'}</code>
					</small>
				{/each}
			</dd>
			<dt>You may</dt>
			<dd>{cap(accessOf(schema, handle) ?? 'nothing')}</dd>
		</dl>
	</div>

	{#if error}<p class="error">{error}</p>{/if}

	<div class="card">
		<h2>
			Entries <span class="muted">{total}</span>
			{#if total > entries.length}
				<span class="muted">(the first {entries.length})</span>
			{/if}
		</h2>
		<table>
			<thead>
				<tr>
					<th>Title</th>
					<th>Type</th>
					<th>URI</th>
					{#each schema.sites as s (s.handle)}<th>{s.name}</th>{/each}
					<th>Draft</th>
					<th>Changed</th>
				</tr>
			</thead>
			<tbody>
				{#each entries as e (e.id)}
					<tr>
						<td style:padding-left="{8 + depth(e) * 16}px">
							{e.title}
						</td>
						<td><code>{e.type}</code></td>
						<td>
							<code>{e.uri === null ? '–' : '/' + e.uri}</code>
						</td>
						{#each schema.sites as s (s.handle)}
							{@const on = published[e.id]?.[s.handle]}
							<td class:live={on}>{on ? 'Live' : 'Off'}</td>
						{/each}
						<td>
							{drafts.includes(e.id) ? 'Unpublished changes' : ''}
						</td>
						<td>{date(e.updatedAt)}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
{:else}
	<p class="error">No section {handle}</p>
{/if}

<style>
	.card {
		margin-bottom: 20px;
		overflow-x: auto;
	}

	dl {
		display: grid;
		grid-template-columns: 140px 1fr;
		gap: 6px 16px;
		margin: 0;
		font-size: 14px;
	}

	dt {
		color: var(--muted);
	}

	dd {
		margin: 0;
	}

	table {
		width: 100%;
		border-collapse: collapse;
		font-size: 14px;
	}

	th {
		padding: 6px 8px;
		text-align: left;
		font-weight: 600;
	}

	td {
		padding: 6px 8px;
		border-top: 1px solid var(--line);
		color: var(--muted);
	}

	td:first-child,
	td.live {
		color: var(--text);
	}

	code {
		font-size: 13px;
		color: var(--muted);
	}

	.muted {
		color: var(--muted);
		font-size: 14px;
	}
</style>
