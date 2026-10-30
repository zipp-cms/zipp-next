<script lang="ts">
	import type { FieldOf, Schema, Values } from '../core/schema';
	import { valueOf } from '../core/schema';
	import type { EntryRef, Refs } from '../entries/api';
	import EntryChip from '../entries/EntryChip.svelte';
	import EntrySearch from '../entries/EntrySearch.svelte';
	import ItemActions from 'zipp-ui/ItemActions.svelte';
	import { inserted, isFull, moved, removed } from '../core/list';

	/** Entries in an order, each a chip that opens it. */
	let {
		schema,
		field,
		values,
		refs,
	}: {
		schema: Schema;
		field: FieldOf<'relation'>;
		values: Values;
		refs: Refs;
	} = $props();

	const types = $derived(field.types);
	const max = $derived(field.max);
	const ids = $derived(valueOf(values, field) ?? []);
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

<style lang="scss">
	.relation {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}
</style>
