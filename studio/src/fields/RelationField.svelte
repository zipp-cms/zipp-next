<script lang="ts">
	import type { EntryRef, Field, Refs, Schema, Values } from '../api';
	import EntryChip from '../lib/EntryChip.svelte';
	import EntrySearch from '../lib/EntrySearch.svelte';
	import ItemActions from '../lib/ItemActions.svelte';
	import { inserted, isFull, moved, removed } from '../lib/list';

	/** Entries in an order, each a chip that opens it. */
	let {
		schema,
		field,
		values,
		refs,
	}: { schema: Schema; field: Field; values: Values; refs: Refs } = $props();

	const types = $derived(field.kind === 'relation' ? field.types : []);
	const max = $derived(field.kind === 'relation' ? field.max : null);
	const ids = $derived<number[]>(values[field.handle] ?? []);
	const set = (next: number[]) => (values[field.handle] = next);

	function add(e: EntryRef) {
		refs[e.id] = e;
		set(inserted(ids, e.id));
	}
</script>

<div class="relation">
	{#each ids as id, i (id)}
		<EntryChip {schema} {refs} {id}>
			{#snippet actions()}
				<ItemActions
					index={i}
					count={ids.length}
					onmove={by => set(moved(ids, i, by))}
					onremove={() => set(removed(ids, i))}
					removeTitle="Remove from the field"
				/>
			{/snippet}
		</EntryChip>
	{/each}

	{#if !isFull(ids.length, max)}
		<div>
			<EntrySearch
				{schema}
				{types}
				exclude={ids}
				multiple={max !== 1}
				onpick={add}
			/>
		</div>
	{/if}
</div>

<style>
	.relation {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}
</style>
