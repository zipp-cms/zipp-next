<script lang="ts">
	import type { Schema } from '../core/schema';
	import { gqlType, usedIn, visibleEntryTypes } from './setup';

	/** The entry types or the blocks, with where they are used. */
	let { schema, tab }: { schema: Schema; tab: 'types' | 'blocks' } = $props();

	const components = $derived(
		tab === 'types'
			? visibleEntryTypes(schema)
			: schema.components.filter(c => !c.entry),
	);
</script>

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
			{#each components as c (c.handle)}
				<tr>
					<td>
						<a href="/admin/setup/{tab}/{c.handle}">{c.name}</a>
					</td>
					<td><code>{c.handle}</code></td>
					<td>{c.fields.length}</td>
					<td>
						{#each usedIn(schema, c) as u, i (u.text)}
							{i ? ', ' : ''}
							<a href={u.href}>{u.text}</a>
						{:else}
							–
						{/each}
					</td>
					<td><code>{gqlType(c)}</code></td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
