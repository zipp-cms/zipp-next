<script lang="ts">
	import { type Schema } from '../core/schema';
	import { listEntries } from '../entries/api';
	import { accessOf } from '../core/access';
	import { accessText, visibleSections } from './setup';
	import Table from 'zipp-ui/Table.svelte';

	/** The sections the role sees, with how many entries each has. */
	let { schema }: { schema: Schema } = $props();

	const sections = $derived(visibleSections(schema));

	// the number of entries of each section, in the primary site
	let counts = $state<Record<string, number>>({});
	$effect(() => {
		for (const s of sections) {
			listEntries({ section: s.handle, limit: '1' })
				.then(r => (counts[s.handle] = r.total))
				.catch(() => {});
		}
	});
</script>

<div class="card">
	<Table
		rows={sections}
		key={s => s.handle}
		columns={[
			'Name',
			'Handle',
			'Kind',
			'Entry types',
			'URI',
			'Entries',
			'You may',
		]}
		href={s => `/admin/setup/sections/${s.handle}`}
	>
		{#snippet row(s)}
			<td>
				<a href="/admin/setup/sections/{s.handle}">{s.name}</a>
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
			<td>{accessText(accessOf(schema, s.handle))}</td>
		{/snippet}
	</Table>
</div>
