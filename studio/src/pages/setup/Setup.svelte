<script lang="ts">
	import { listEntries, type Component, type Schema } from '../../api';
	import { accessOf, can, isAdmin, roleName } from '../../lib/access';
	import { describeField } from '../../lib/fieldKind';
	import SetupSection from './SetupSection.svelte';
	import Crumbs from '../../lib/Crumbs.svelte';
	import TypePreview from './TypePreview.svelte';
	import { shown } from '../../route.svelte';

	/**
	 * The content model, read only like Craft's settings when admin
	 * changes are off: it is code, this shows what the code says, with the
	 * handles and GraphQL types a frontend uses.
	 */
	let {
		schema,
		tab,
		handle,
	}: { schema: Schema; tab: string; handle: string | null } = $props();

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

	/** Where a component is used: sections and fields, as links here. */
	function usedIn(c: Component): { text: string; href: string }[] {
		const out = sections
			.filter(s => s.types.includes(c.handle))
			.map(s => ({
				text: `section ${s.name}`,
				href: `/admin/setup/sections/${s.handle}`,
			}));
		for (const other of schema.components) {
			for (const f of other.fields) {
				const uses =
					(f.kind === 'blocks' && f.types.includes(c.handle)) ||
					(f.kind === 'richText' && f.blocks.includes(c.handle)) ||
					(f.kind === 'group' && f.component === c.handle) ||
					(f.kind === 'relation' && f.types.includes(c.handle));
				if (uses)
					out.push({
						text: `${other.handle}.${f.handle}`,
						href: `/admin/setup/${other.entry ? 'types' : 'blocks'}/${other.handle}#${f.handle}`,
					});
			}
		}
		return out;
	}

	const TAB_NAMES: Record<string, string> = {
		sections: 'Sections',
		types: 'Entry types',
		blocks: 'Blocks',
		sites: 'Sites, transforms and roles',
	};
	// Setup, the tab, the section or type of a page of its own
	const crumbs = $derived.by(() => {
		const out: { label: string; href?: string }[] = [
			{ label: 'Setup', href: '/admin/setup/sections' },
			{ label: TAB_NAMES[tab] ?? tab, href: `/admin/setup/${tab}` },
		];
		if (handle && tab === 'sections') {
			const s = schema.sections.find(s => s.handle === handle);
			out.push({ label: s?.name ?? handle });
		} else if (handle) {
			const c = schema.components.find(c => c.handle === handle);
			out.push({ label: c?.name ?? handle });
		}
		return out;
	});

	function access(level: string | null | undefined) {
		return level ? level[0].toUpperCase() + level.slice(1) : '–';
	}
</script>

<div class="page-top">
	<h1><Crumbs items={crumbs} /></h1>
	<span class="muted">Read only, the content model is code</span>
</div>

{#if tab === 'sections' && handle}
	<SetupSection {schema} {handle} />
{:else if tab === 'sections'}
	<div class="card">
		<table class="table">
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
							<a href="/admin/setup/sections/{s.handle}">
								{s.name}
							</a>
						</td>
						<td><code>{s.handle}</code></td>
						<td>{s.kind}</td>
						<td>
							{#each s.types as t, i (t)}
								{i ? ', ' : ''}
								<a href="/admin/setup/types/{t}">{t}</a>
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
{:else if (tab === 'types' || tab === 'blocks') && handle}
	{@const c = schema.components.find(c => c.handle === handle)}
	{#if c}
		<div class="layout">
			<div>
				<div class="card preview">
					<h2>
						Editor
						<span class="muted">how editors see it, read only</span>
					</h2>
					{#key c.handle}
						<TypePreview {schema} component={c} />
					{/key}
				</div>
				<div class="card">
					<h2>
						Fields
						<code class="gql">
							{c.handle}_{c.entry ? 'Entry' : 'Block'}
						</code>
					</h2>
					{#if c.fields.length}
						<table class="table">
							<thead>
								<tr>
									<th>Label</th>
									<th>Handle</th>
									{#if c.tabs?.length}<th>Tab</th>{/if}
									<th>Kind</th>
									<th>Sites</th>
									<th></th>
								</tr>
							</thead>
							<tbody>
								{#each c.fields as f (f.handle)}
									<tr id={f.handle}>
										<td>{f.label}</td>
										<td><code>{f.handle}</code></td>
										{#if c.tabs?.length}
											<td>
												{c.tabs.find(t =>
													t.fields.includes(f.handle),
												)?.name}
											</td>
										{/if}
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
			</div>
			<aside class="card">
				<h2>Used in</h2>
				<ul>
					{#each usedIn(c) as u (u.text)}
						<li><a href={u.href}>{u.text}</a></li>
					{:else}
						<li class="muted">nothing</li>
					{/each}
				</ul>
			</aside>
		</div>
	{:else}
		<p class="error">
			No {tab === 'types' ? 'entry type' : 'block'}
			{handle}
		</p>
	{/if}
{:else if tab === 'types' || tab === 'blocks'}
	<div class="card">
		<table class="table">
			<thead>
				<tr>
					<th>Name</th>
					<th>Handle</th>
					<th>Fields</th>
					<th>Used in</th>
					<th>GraphQL</th>
				</tr>
			</thead>
			<tbody>
				{#each tab === 'types' ? entryTypes : blocks as c (c.handle)}
					<tr>
						<td>
							<a href="/admin/setup/{tab}/{c.handle}">{c.name}</a>
						</td>
						<td><code>{c.handle}</code></td>
						<td>{c.fields.length}</td>
						<td>
							{#each usedIn(c) as u, i (u.text)}
								{i ? ', ' : ''}
								<a href={u.href}>{u.text}</a>
							{:else}
								–
							{/each}
						</td>
						<td>
							<code>
								{c.handle}_{c.entry ? 'Entry' : 'Block'}
							</code>
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
{:else}
	<div class="card">
		<h2>Sites</h2>
		<table class="table">
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
						<td>
							{s.name}{i === 0
								? ' (primary)'
								: ''}{schema.offline.includes(s.handle)
								? ' (offline, not on the website)'
								: ''}
						</td>
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
		<table class="table">
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
		<table class="table">
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
	tr:global(.target) td {
		background: var(--hover);
		transition: background 0.3s;
	}

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

	ul {
		list-style: none;
		margin: 0;
		padding: 0;
		font-size: var(--fs-base);
	}

	li {
		padding: 5px 0;
		border-top: 1px solid var(--line);
	}

	.card {
		margin-bottom: 20px;
		overflow-x: auto;
	}

	h2 {
		display: flex;
		align-items: baseline;
		gap: 10px;
	}

	.gql {
		margin-left: auto;
	}

	.flags {
		color: var(--muted);
	}
</style>
