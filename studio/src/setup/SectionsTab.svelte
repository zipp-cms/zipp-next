<script lang="ts">
	import { type Schema } from '../core/schema';
	import { listEntries } from '../entries/api';
	import { accessOf } from '../core/access';
	import { accessText, visibleSections } from './setup';

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
				</tr>
			{/each}
		</tbody>
	</table>
</div>
