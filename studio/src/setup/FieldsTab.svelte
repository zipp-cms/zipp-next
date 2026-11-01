<script lang="ts">
	import type { Schema } from '../core/schema';
	import { componentOf } from '../core/schema';
	import SearchInput from 'zipp-ui/SearchInput.svelte';
	import Table from 'zipp-ui/Table.svelte';
	import { componentHref, fieldUsage, heldBy, type FieldUse } from './setup';

	/**
	 * Every field handle and where it is, like Craft's field usage: its
	 * label and kind, the blocks or groups it holds and the entry types
	 * and blocks it is in. The search finds a handle, a label or a block,
	 * so `button` shows every field that holds the button block.
	 */
	let { schema }: { schema: Schema } = $props();

	let search = $state('');

	const all = $derived(fieldUsage(schema));

	// the blocks and groups the field holds, in any of its places
	function held(u: FieldUse) {
		const handles = [...new Set(u.places.flatMap(p => heldBy(p.field)))];
		return handles.map(
			h => componentOf(schema, h) ?? { handle: h, name: h, entry: false },
		);
	}

	const shown = $derived.by(() => {
		const q = search.trim().toLowerCase();
		if (!q) return all;
		return all.filter(u =>
			[
				u.handle,
				...u.labels,
				...u.kinds,
				...held(u).flatMap(c => [c.handle, c.name]),
			].some(t => t.toLowerCase().includes(q)),
		);
	});
</script>

<div class="card">
	<div class="top">
		<SearchInput
			bind:value={search}
			placeholder="Search a field or block"
		/>
		<span class="muted">{shown.length} of {all.length}</span>
	</div>
	<Table
		rows={shown}
		key={u => u.handle}
		columns={['Handle', 'Label', 'Kind', 'Holds', 'In']}
		empty="No field like that"
	>
		{#snippet row(u)}
			<td><code>{u.handle}</code></td>
			<td>{u.labels.join(', ')}</td>
			<td>{u.kinds.join('; ')}</td>
			<td>
				{#each held(u) as c, i (c.handle)}
					{i ? ', ' : ''}
					<a href={componentHref(c)}>{c.name}</a>
				{/each}
			</td>
			<td>
				{#each u.places as p, i (p.component.handle)}
					{i ? ', ' : ''}
					<a href="{componentHref(p.component)}#field-{u.handle}">
						{p.component.name}
					</a>
				{/each}
			</td>
		{/snippet}
	</Table>
</div>

<style lang="scss">
	.top {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 0.75rem;
	}
</style>
