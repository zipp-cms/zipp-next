<script lang="ts">
	import type { FieldOf, Schema, Values } from '../core/schema';
	import { t } from '../i18n/index.svelte';
	import { valueOf } from '../core/schema';
	import type { EntryRef, Refs } from '../entries/api';
	import EntryChip from '../entries/EntryChip.svelte';
	import EntryPicker from '../entries/EntryPicker.svelte';
	import { chooseLabel } from '../entries/picker';
	import AddButton from 'zipp-ui/AddButton.svelte';
	import ItemActions from 'zipp-ui/ItemActions.svelte';
	import DragHandle from 'zipp-ui/DragHandle.svelte';
	import SortLine from 'zipp-ui/SortLine.svelte';
	import { listSortable } from 'zipp-ui/sortable';
	import { inserted, isFull, removed } from '../core/list';

	/**
	 * Entries in an order, each a compact chip that opens it, side by
	 * side and wrapping like Craft's, the Choose button at the end.
	 */
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
	const sort = listSortable(
		() => list,
		() => ids,
		set,
		{ grid: () => true },
	);

	let picking = $state(false);

	/** In the picker a click adds an entry, or takes a chosen one back. */
	function toggle(e: EntryRef) {
		refs[e.id] = e;
		const i = ids.indexOf(e.id);
		if (i >= 0) set(removed(ids, i));
		else if (!isFull(ids.length, max)) set(inserted(ids, e.id));
	}
</script>

<div class="relation" bind:this={list}>
	{#each ids as id, i (id)}
		<div data-sort class="item" class:sort-dragging={sort.moving(i)}>
			<EntryChip {schema} {refs} {id}>
				{#snippet handle()}
					<DragHandle
						sortable={sort}
						index={i}
						label={t('fields.moveEntry')}
					/>
				{/snippet}
				{#snippet actions()}
					<ItemActions
						onremove={() => set(removed(ids, i))}
						removeLabel={t('fields.removeFromField')}
					/>
				{/snippet}
			</EntryChip>
		</div>
	{/each}
	<SortLine sortable={sort} />

	{#if !isFull(ids.length, max)}
		<AddButton onclick={() => (picking = true)}>
			{chooseLabel(schema, types)}
		</AddButton>
	{/if}
</div>

{#if picking}
	<EntryPicker
		{schema}
		{types}
		sections={field.sections}
		chosen={ids}
		multiple={max !== 1}
		{max}
		onpick={toggle}
		onclose={() => (picking = false)}
	/>
{/if}

<style lang="scss">
	// the chips side by side, wrapping, the button after the last
	.relation {
		position: relative;
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 0.375rem;
	}

	// a long title gets an ellipsis instead of a wider chip than the field
	.item {
		max-width: 100%;
		min-width: 0;
	}
</style>
