<script lang="ts">
	import type { Schema } from '../core/schema';
	import { isAdmin, roleName } from '../core/access';
	import { accessText, visibleRoles, visibleSections } from './setup';

	/** The sites, the image transforms and the roles of the code. */
	let { schema }: { schema: Schema } = $props();

	const sections = $derived(visibleSections(schema));
	const roles = $derived(visibleRoles(schema));
</script>

<div class="cards">
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
			You are {isAdmin(schema)
				? 'an admin and can do everything'
				: roleName(schema)}. View sees entries, Edit changes them as
			drafts and creates new ones, Publish makes drafts live and moves
			entries, Delete deletes them. Users without a role are admins.
		</p>
		<table class="table">
			<thead>
				<tr>
					<th>Role</th>
					{#each sections as s (s.handle)}<th>{s.name}</th>{/each}
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
							<td>{accessText(r.sections[s.handle])}</td>
						{/each}
						<td>{accessText(r.assets)}</td>
						<td>{r.ai ? 'Yes' : '–'}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</div>
