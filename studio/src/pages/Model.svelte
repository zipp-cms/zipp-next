<script lang="ts">
	import { listEntries, type Component, type Schema } from '../api';
	import { accessOf, can, isAdmin, roleName } from '../lib/access';
	import { describeField } from '../lib/fieldKind';
	import { shown } from '../route.svelte';

	/**
	 * The content model, read only like Craft's settings when admin
	 * changes are off: it is code, this shows what the code says, with the
	 * handles and GraphQL types a frontend uses.
	 */
	let { schema, tab }: { schema: Schema; tab: string } = $props();

	$effect(() => {
		shown.section = '';
	});

	// a role sees its sections and itself, so editors see what they may do
	const admin = $derived(isAdmin(schema));
	const sections = $derived(
		schema.sections.filter(s => can(schema, s.handle, 'view')),
	);
	const entryTypes = $derived(
		schema.components.filter(
			c => c.entry && sections.some(s => s.types.includes(c.handle)),
		),
	);
	const roles = $derived(
		admin
			? schema.roles
			: schema.roles.filter(r => r.handle === schema.me.role),
	);
	const blocks = $derived(schema.components.filter(c => !c.entry));

	// the number of entries of each section, in the primary site
	let counts = $state<Record<string, number>>({});
	$effect(() => {
		for (const s of sections) {
			listEntries({ section: s.handle, limit: '1' })
				.then(r => (counts[s.handle] = r.total))
				.catch(() => {});
		}
	});

	/** Where a component is used: sections and fields. */
	function usedIn(c: Component): string[] {
		const out = sections
			.filter(s => s.types.includes(c.handle))
			.map(s => `section ${s.name}`);
		for (const other of schema.components) {
			for (const f of other.fields) {
				const uses =
					(f.kind === 'blocks' && f.types.includes(c.handle)) ||
					(f.kind === 'richText' && f.blocks.includes(c.handle)) ||
					(f.kind === 'group' && f.component === c.handle) ||
					(f.kind === 'relation' && f.types.includes(c.handle));
				if (uses) out.push(`${other.handle}.${f.handle}`);
			}
		}
		return out;
	}

	function access(level: string | null | undefined) {
		return level ? level[0].toUpperCase() + level.slice(1) : '–';
	}
</script>

<div class="page-top">
	<h1>
		{tab === 'types'
			? 'Entry types'
			: tab === 'blocks'
				? 'Blocks'
				: tab === 'sites'
					? 'Sites, transforms and roles'
					: 'Sections'}
	</h1>
	<span class="muted">Read only, the content model is code</span>
</div>

{#if tab === 'sections'}
	<div class="card">
		<table>
			<thead>
				<tr>
					<th>Name</th>
					<th>Handle</th>
					<th>Kind</th>
					<th>Entry types</th>
					<th>URI</th>
					<th>Entries</th>
					<th>You may</th>
				</tr>
			</thead>
			<tbody>
				{#each sections as s (s.handle)}
					<tr>
						<td>
							<a href="/admin/entries/{s.handle}">{s.name}</a>
						</td>
						<td><code>{s.handle}</code></td>
						<td>{s.kind}</td>
						<td>
							{#each s.types as t, i (t)}
								{i ? ', ' : ''}
								<a href="/admin/model/types#{t}">{t}</a>
							{/each}
						</td>
						<td>
							<code>{s.uri ?? '–'}</code>
							{#each Object.entries(s.type_uris ?? {}) as [t, u] (t)}
								<br />
								<small>
									{t}:
									<code>{u || '/'}</code>
								</small>
							{/each}
						</td>
						<td>{counts[s.handle] ?? ''}</td>
						<td>{access(accessOf(schema, s.handle))}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
{:else if tab === 'types' || tab === 'blocks'}
	{#each tab === 'types' ? entryTypes : blocks as c (c.handle)}
		<div class="card" id={c.handle}>
			<h2>
				{c.name}
				<code>{c.handle}</code>
				<code class="gql">
					{c.handle}_{c.entry ? 'Entry' : 'Block'}
				</code>
			</h2>
			<p class="muted">Used in: {usedIn(c).join(', ') || 'nothing'}</p>
			{#if c.fields.length}
				<table>
					<thead>
						<tr>
							<th>Label</th>
							<th>Handle</th>
							<th>Kind</th>
							<th>Sites</th>
							<th></th>
						</tr>
					</thead>
					<tbody>
						{#each c.fields as f (f.handle)}
							<tr>
								<td>{f.label}</td>
								<td><code>{f.handle}</code></td>
								<td>{describeField(f)}</td>
								<td>
									{c.entry
										? f.translatable
											? 'per site'
											: 'all sites'
										: ''}
								</td>
								<td class="flags">
									{f.required ? 'required' : ''}
									{f.indexed ? 'indexed' : ''}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			{:else}
				<p class="muted">No fields, only a title.</p>
			{/if}
		</div>
	{/each}
{:else}
	<div class="card">
		<h2>Sites</h2>
		<table>
			<thead>
				<tr>
					<th>Name</th>
					<th>Handle</th>
					<th>Language</th>
					<th>URL</th>
				</tr>
			</thead>
			<tbody>
				{#each schema.sites as s, i (s.handle)}
					<tr>
						<td>{s.name}{i === 0 ? ' (primary)' : ''}</td>
						<td><code>{s.handle}</code></td>
						<td>{s.language}</td>
						<td>{schema.siteUrls[s.handle] ?? '–'}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>

	<div class="card">
		<h2>Image transforms</h2>
		<table>
			<tbody>
				{#each schema.transforms as t (t.handle)}
					<tr>
						<td><code>{t.handle}</code></td>
						<td>
							{`${t.width ?? 'auto'} × ${t.height ?? 'auto'}${t.crop ? ', cropped around the focal point' : ''}`}
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>

	<div class="card" id="roles">
		<h2>Roles</h2>
		<p class="muted">
			You are {admin
				? 'an admin and can do everything'
				: roleName(schema)}. View sees entries, Edit changes them as
			drafts and creates new ones, Publish makes drafts live and moves
			entries, Delete deletes them. Users without a role are admins.
		</p>
		<table>
			<thead>
				<tr>
					<th>Role</th>
					{#each sections as s (s.handle)}<th>
							{s.name}
						</th>{/each}
					<th>Assets</th>
					<th>AI</th>
				</tr>
			</thead>
			<tbody>
				{#each roles as r (r.handle)}
					<tr>
						<td>
							{r.name}
							<code>{r.handle}</code>
						</td>
						{#each sections as s (s.handle)}
							<td>{access(r.sections[s.handle])}</td>
						{/each}
						<td>{access(r.assets)}</td>
						<td>{r.ai ? 'Yes' : '–'}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
{/if}

<style>
	.card {
		margin-bottom: 20px;
		overflow-x: auto;
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
		vertical-align: top;
	}

	code {
		font-size: 13px;
		color: var(--muted);
	}

	h2 {
		display: flex;
		align-items: baseline;
		gap: 10px;
	}

	.gql {
		margin-left: auto;
	}

	.muted {
		color: var(--muted);
		font-size: 14px;
	}

	.flags {
		color: var(--muted);
	}
</style>
