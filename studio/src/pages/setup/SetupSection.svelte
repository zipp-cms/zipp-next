<script lang="ts">
	import {
		entryUrl,
		listEntries,
		navGroups,
		type Entry,
		type Schema,
	} from '../../api';
	import { accessOf, isAdmin } from '../../lib/access';
	import PluginCards from './PluginCards.svelte';

	/**
	 * A section in the setup, read only: its settings in the main column
	 * (entry types, urls per site, who may do what, cards of plugins), its entries as a
	 * short list on the side, linking to the editor.
	 */
	let { schema, handle }: { schema: Schema; handle: string } = $props();

	const section = $derived(schema.sections.find(s => s.handle === handle));
	const types = $derived(
		(section?.types ?? []).map(
			t =>
				schema.components.find(c => c.handle === t) ?? {
					handle: t,
					name: t,
					entry: true,
					fields: [],
				},
		),
	);
	const group = $derived(
		navGroups(schema).find(g => g.sections.includes(handle))?.name,
	);
	// an admin sees every role, a role only itself
	const roles = $derived(
		isAdmin(schema)
			? schema.roles
			: schema.roles.filter(r => r.handle === schema.me.role),
	);

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

	/** The uri of an entry type, its own or the section's. */
	function uriOf(type: string, site?: string): string | null {
		const own = section?.type_uris?.[type];
		if (own !== undefined) return own;
		const inSite = site ? section?.site_uris?.[site] : undefined;
		return inSite ?? section?.uri ?? null;
	}

	const KINDS = {
		single: 'Single: exactly one entry',
		channel: 'Channel: a list, newest first',
		structure: 'Structure: a tree in a set order',
	};
	const cap = (s: string | null | undefined) =>
		s ? s[0].toUpperCase() + s.slice(1) : '–';
</script>

{#if section}
	<div class="layout">
		<div class="main">
			<div class="card">
				<h2>General</h2>
				<dl>
					<dt>Name</dt>
					<dd>{section.name}</dd>
					<dt>Handle</dt>
					<dd><code>{section.handle}</code></dd>
					<dt>Kind</dt>
					<dd>{KINDS[section.kind]}</dd>
					{#if section.uri_if}
						<dt>Pages</dt>
						<dd>
							only entries with <code>{section.uri_if}</code>
							on
						</dd>
					{/if}
					<dt>Navigation</dt>
					<dd>{group ?? '–'}</dd>
					<dt>GraphQL</dt>
					<dd>
						<code>
							{section.kind === 'single' && section.uri === null
								? `globalSet(handle: "${handle}")`
								: `entries(section: "${handle}")`}
						</code>
					</dd>
					<dt>You may</dt>
					<dd>{cap(accessOf(schema, handle) ?? 'nothing')}</dd>
				</dl>
			</div>

			<div class="card">
				<h2>Entry types</h2>
				<table class="table">
					<thead>
						<tr>
							<th>Name</th>
							<th>Handle</th>
							<th>Fields</th>
							<th>URI</th>
							<th>GraphQL</th>
						</tr>
					</thead>
					<tbody>
						{#each types as t (t.handle)}
							{@const uri = uriOf(t.handle)}
							<tr>
								<td>
									<a href="/admin/setup/types/{t.handle}">
										{t.name}
									</a>
								</td>
								<td><code>{t.handle}</code></td>
								<td>{t.fields.length}</td>
								<td>
									<code>
										{uri === null
											? 'no pages'
											: uri || '/ (home)'}
									</code>
								</td>
								<td><code>{t.handle}_Entry</code></td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>

			<div class="card">
				<h2>Sites</h2>
				<table class="table">
					<thead>
						<tr>
							<th>Site</th>
							<th>Language</th>
							<th>URLs</th>
							<th>Status</th>
						</tr>
					</thead>
					<tbody>
						{#each schema.sites as s (s.handle)}
							<tr>
								<td>{s.name}</td>
								<td>{s.language}</td>
								<td>
									{#each types as t (t.handle)}
										{@const uri = uriOf(t.handle, s.handle)}
										<div>
											{#if types.length > 1}
												<span class="muted">
													{t.name}:
												</span>
											{/if}
											<code>
												{uri === null
													? 'no pages'
													: (entryUrl(
															schema,
															s.handle,
															uri,
														) ?? '/' + uri)}
											</code>
										</div>
									{/each}
								</td>
								<td>
									{schema.offline.includes(s.handle)
										? 'Offline'
										: 'Online'}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>

			<div class="card">
				<h2>Access</h2>
				<table class="table">
					<thead>
						<tr>
							<th>Role</th>
							<th>May</th>
						</tr>
					</thead>
					<tbody>
						{#if isAdmin(schema)}
							<tr>
								<td>Admins (no role)</td>
								<td>Everything</td>
							</tr>
						{/if}
						{#each roles as r (r.handle)}
							<tr>
								<td>
									{r.name}
									<code>{r.handle}</code>
								</td>
								<td>
									{cap(
										r.sections[handle] ?? 'nothing, hidden',
									)}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
				<p class="muted">
					<a href="/admin/setup/sites#roles">All roles</a>
				</p>
			</div>

			<PluginCards {schema} {section} />
		</div>

		<aside class="card">
			<h2>
				Entries <span class="muted">{total}</span>
			</h2>
			{#if error}<p class="error">{error}</p>{/if}
			<ul>
				{#each entries as e (e.id)}
					<li style:padding-left="{depth(e) * 12}px">
						<a href="/admin/entries/{handle}/{e.id}">{e.title}</a>
						{#if drafts.includes(e.id)}
							<span
								class="dot"
								title="Unpublished changes"
							></span>
						{/if}
						<span class="sites">
							{#each schema.sites as s (s.handle)}
								{@const on = published[e.id]?.[s.handle]}
								<span
									class="badge"
									class:draft={!on}
									title="{s.name}: {on
										? 'published'
										: 'not published'}"
								>
									{s.handle.toUpperCase()}
								</span>
							{/each}
						</span>
					</li>
				{/each}
			</ul>
			{#if total > entries.length}
				<p class="muted">The first {entries.length}.</p>
			{/if}
			<p class="muted">
				<a href="/admin/entries/{handle}">Open in Content</a>
			</p>
		</aside>
	</div>
{:else}
	<p class="error">No section {handle}</p>
{/if}

<style>
	.layout {
		display: grid;
		grid-template-columns: minmax(0, 1fr) 300px;
		gap: 20px;
		align-items: start;
	}

	@media (max-width: 1100px) {
		.layout {
			grid-template-columns: 1fr;
		}
	}

	.card {
		margin-bottom: 20px;
		overflow-x: auto;
	}

	dl {
		display: grid;
		grid-template-columns: 140px 1fr;
		gap: 8px 16px;
		margin: 0;
	}

	dt {
		color: var(--muted);
	}

	dd {
		margin: 0;
	}

	ul {
		list-style: none;
		margin: 0;
		padding: 0;
		font-size: var(--fs-base);
	}

	li {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 5px 0;
		border-top: 1px solid var(--line);
	}

	li a {
		flex: 1;
		min-width: 0;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		color: inherit;
	}

	.dot {
		width: 7px;
		height: 7px;
		border-radius: 50%;
		background: var(--changed);
	}

	.sites {
		display: flex;
		gap: 3px;
	}
</style>
