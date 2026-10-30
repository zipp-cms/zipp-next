<script lang="ts">
	import type { FieldOf, Schema, Values } from '../core/schema';
	import { valueOf } from '../core/schema';
	import type { EntryRef, Refs } from '../entries/api';
	import EntryChip from '../entries/EntryChip.svelte';
	import EntrySearch from '../entries/EntrySearch.svelte';
	import ItemActions from 'zipp-ui/ItemActions.svelte';
	import DragHandle from 'zipp-ui/DragHandle.svelte';
	import SortLine from 'zipp-ui/SortLine.svelte';
	import { Sortable, moved } from 'zipp-ui/sortable';
	import { inserted, isFull, removed } from '../core/list';

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

	let list = $state<HTMLElement>();
	const sort = new Sortable({
		container: () => list,
		depths: () => ids.map(() => 0),
		onmove: (from, drop) => set(moved(ids, from, drop)),
	});

	function add(e: EntryRef) {
		refs[e.id] = e;
		set(inserted(ids, e.id));
	}
</script>

<div class="relation" bind:this={list}>
	{#each ids as id, i (id)}
		<div data-sort class="item" class:sort-dragging={sort.dragging === i}>
			<DragHandle sortable={sort} index={i} label="Move the entry" />
			<EntryChip {schema} {refs} {id}>
				{#snippet actions()}
					<ItemActions
						onremove={() => set(removed(ids, i))}
						removeLabel="Remove from the field"
					/>
				{/snippet}
			</EntryChip>
		</div>
	{/each}
	<SortLine sortable={sort} />

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
		position: relative;
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	// its handle, then the chip
	.item {
		display: flex;
		align-items: center;
		gap: 0.25rem;

		> :global(:last-child) {
			flex: 1;
			min-width: 0;
		}
	}
</style>
