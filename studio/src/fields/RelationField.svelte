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
	const sort = listSortable(
		() => list,
		() => ids,
		set,
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
			<DragHandle
				sortable={sort}
				index={i}
				label={t('fields.moveEntry')}
			/>
			<EntryChip {schema} {refs} {id}>
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
		<div class="choose">
			<AddButton onclick={() => (picking = true)}>
				{chooseLabel(schema, types)}
			</AddButton>
		</div>
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
	.relation {
		position: relative;
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	// the button further from the chips than they are from each other,
	// none above it while the field is empty
	.item ~ .choose {
		margin-top: 0.375rem;
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
