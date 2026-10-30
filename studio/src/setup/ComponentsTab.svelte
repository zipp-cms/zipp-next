<script lang="ts">
	import type { Schema } from '../core/schema';
	import SchemaIcon from '../core/SchemaIcon.svelte';
	import { gqlType, usedIn, visibleEntryTypes } from './setup';
	import Table from 'zipp-ui/Table.svelte';

	/** The entry types or the blocks, with where they are used. */
	let { schema, tab }: { schema: Schema; tab: 'types' | 'blocks' } = $props();

	const components = $derived(
		tab === 'types'
			? visibleEntryTypes(schema)
			: schema.components.filter(c => !c.entry),
	);
</script>

<div class="card">
	<Table
		rows={components}
		key={c => c.handle}
		columns={['Name', 'Handle', 'Fields', 'Used in', 'GraphQL']}
		href={c => `/admin/setup/${tab}/${c.handle}`}
	>
		{#snippet row(c)}
			<td>
				<a class="name" href="/admin/setup/{tab}/{c.handle}">
					<SchemaIcon icon={c.icon} size={14} />
					{c.name}
				</a>
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
		{/snippet}
	</Table>
</div>

<style lang="scss">
	.name {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
	}
</style>
