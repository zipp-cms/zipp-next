<script lang="ts">
	import { accessText, uriOf, visibleRoles } from './setup';
	import { depth } from '../entries/tree';
	import SiteBadges from '../entries/SiteBadges.svelte';
	import { entryUrl, navGroups, type Schema } from '../core/schema';
	import type { EntryList } from '../entries/api';
	import { accessOf, isAdmin } from '../core/access';
	import PluginCards from './PluginCards.svelte';
	import Table from 'zipp-ui/Table.svelte';
	import { componentOf, sectionOf } from '../core/schema';

	/**
	 * A section in the setup, read only: its settings in the main column
	 * (entry types, urls per site, who may do what, cards of plugins), its entries as a
	 * short list on the side, linking to the editor.
	 */
	let {
		schema,
		handle,
		list,
	}: { schema: Schema; handle: string; list: EntryList } = $props();

	const section = $derived(sectionOf(schema, handle));
	const types = $derived(
		(section?.types ?? []).map(
			t =>
				componentOf(schema, t) ?? {
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
	const roles = $derived(visibleRoles(schema));
	// who may what here, admins first
	const access = $derived([
		...(isAdmin(schema)
			? [{ name: 'Admins (no role)', handle: null, may: 'Everything' }]
			: []),
		...roles.map(r => ({
			name: r.name,
			handle: r.handle,
			may: accessText(r.sections[handle] ?? 'nothing, hidden'),
		})),
	]);

	// the first entries in the primary site, see Setup's loadProps
	const { entries, published, drafts, total } = $derived(list);

	const KINDS = {
		single: 'Single: exactly one entry',
		channel: 'Channel: a list, newest first',
		structure: 'Structure: a tree in a set order',
	};
</script>

{#if section}
	<div class="with-aside">
		<div class="cards">
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
					<dd>{accessText(accessOf(schema, handle) ?? 'nothing')}</dd>
				</dl>
			</div>

			<div class="card">
				<h2>Entry types</h2>
				<Table
					rows={types}
					key={t => t.handle}
					columns={['Name', 'Handle', 'Fields', 'URI', 'GraphQL']}
					href={t => `/admin/setup/types/${t.handle}`}
				>
					{#snippet row(t)}
						{@const uri = uriOf(section, t.handle)}
						<td>
							<a href="/admin/setup/types/{t.handle}">
								{t.name}
							</a>
						</td>
						<td><code>{t.handle}</code></td>
						<td>{t.fields.length}</td>
						<td>
							<code>
								{uri === null ? 'no pages' : uri || '/ (home)'}
							</code>
						</td>
						<td><code>{t.handle}_Entry</code></td>
					{/snippet}
				</Table>
			</div>

			<div class="card">
				<h2>Sites</h2>
				<Table
					rows={schema.sites}
					key={s => s.handle}
					columns={['Site', 'Language', 'URLs', 'Status']}
				>
					{#snippet row(s)}
						<td>{s.name}</td>
						<td>{s.language}</td>
						<td>
							{#each types as t (t.handle)}
								{@const uri = uriOf(
									section,
									t.handle,
									s.handle,
								)}
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
					{/snippet}
				</Table>
			</div>

			<div class="card">
				<h2>Access</h2>
				<Table
					rows={access}
					key={a => a.name}
					columns={['Role', 'May']}
				>
					{#snippet row(a)}
						<td>
							{a.name}
							{#if a.handle}<code>{a.handle}</code>{/if}
						</td>
						<td>{a.may}</td>
					{/snippet}
				</Table>
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
			<ul class="plain">
				{#each entries as e (e.id)}
					<li style:padding-left="{depth(entries, e) * 0.75}rem">
						<a href="/admin/entries/{handle}/{e.id}">{e.title}</a>
						{#if drafts.includes(e.id)}
							<span
								class="badge changes"
								title="Unpublished changes"
							>
								Draft
							</span>
						{/if}
						<SiteBadges {schema} published={published[e.id]} />
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

<style lang="scss">
	dl {
		display: grid;
		grid-template-columns: 8.75rem 1fr;
		gap: 0.5rem 1rem;
		margin: 0;
	}

	dt {
		color: var(--muted);
	}

	dd {
		margin: 0;
	}

	li {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.3125rem 0;
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
</style>
