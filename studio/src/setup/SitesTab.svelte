<script lang="ts">
	import type { Schema } from '../core/schema';
	import { isAdmin, roleName } from '../core/access';
	import { accessText, visibleRoles, visibleSections } from './setup';
	import Table from 'zipp-ui/Table.svelte';
	import MailCard from './MailCard.svelte';

	/**
	 * The sites, the image transforms and the roles of the code, for
	 * admins how mails are sent.
	 */
	let { schema }: { schema: Schema } = $props();

	const sections = $derived(visibleSections(schema));
	const roles = $derived(visibleRoles(schema));
</script>

<div class="cards">
	<div class="card">
		<h2>Sites</h2>
		<Table
			rows={schema.sites}
			key={s => s.handle}
			columns={['Name', 'Handle', 'Language', 'URL']}
		>
			{#snippet row(s, i)}
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
			{/snippet}
		</Table>
	</div>

	<div class="card">
		<h2>Image transforms</h2>
		<Table rows={schema.transforms} key={t => t.handle}>
			{#snippet row(t)}
				<td><code>{t.handle}</code></td>
				<td>
					{`${t.width ?? 'auto'} × ${t.height ?? 'auto'}${t.crop ? ', cropped around the focal point' : ''}`}
				</td>
			{/snippet}
		</Table>
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
		<Table
			rows={roles}
			key={r => r.handle}
			columns={['Role', ...sections.map(s => s.name), 'Assets', 'AI']}
		>
			{#snippet row(r)}
				<td>
					{r.name}
					<code>{r.handle}</code>
				</td>
				{#each sections as s (s.handle)}
					<td>{accessText(r.sections[s.handle])}</td>
				{/each}
				<td>{accessText(r.assets)}</td>
				<td>{r.ai ? 'Yes' : '–'}</td>
			{/snippet}
		</Table>
	</div>

	{#if isAdmin(schema)}
		<MailCard />
	{/if}
</div>
