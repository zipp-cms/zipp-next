<script lang="ts">
	import { untrack } from 'svelte';
	import type { Values } from '../../core/schema';
	import type { NodeProps } from './extensions.svelte';
	import BlockFields from '../BlockFields.svelte';
	import ItemActions from 'zipp-ui/ItemActions.svelte';
	import ItemCard from 'zipp-ui/ItemCard.svelte';
	import NodeFrame from './NodeFrame.svelte';
	import { rememberOpen, wasOpen } from '../opened';
	import DragHandle from 'zipp-ui/DragHandle.svelte';
	import { blockSummary } from '../blocks.svelte';
	import { synced } from '../../core/synced.svelte';
	import { componentOf } from '../../core/schema';
	import { t } from '../../i18n/index.svelte';

	/**
	 * A block inside a rich text: a card with its summary, its fields like
	 * in a blocks field when it opens. A new one opens.
	 */
	let {
		attrs,
		schema,
		refs,
		editable,
		update,
		remove,
		insertLine,
		sortable,
		index,
	}: NodeProps<{ id: string | null; type: string; fields: Values }> =
		$props();

	const comp = $derived(componentOf(schema, attrs.type));
	// the ids of its inputs, unique with several blocks in the text
	const uid = $props.id();
	// changes from the text (undo) come in, changes of the fields go out
	const fields = synced(
		() => attrs.fields,
		f => update({ ...attrs, fields: f }),
	);
	// as it was in this browser, else a new one opens
	let editing = $state(
		untrack(
			() => wasOpen(attrs.id) ?? !Object.keys(attrs.fields ?? {}).length,
		),
	);
	$effect(() =>
		rememberOpen(
			untrack(() => attrs.id),
			editing,
		),
	);
</script>

<NodeFrame {editable} oninsert={insertLine}>
	<ItemCard
		kind={comp?.name ?? attrs.type}
		bind:open={editing}
		summary={preview}
		{handle}
	>
		{#snippet actions()}
			{#if editable}
				<ItemActions onremove={remove} />
			{/if}
		{/snippet}
		{#if comp}
			<fieldset disabled={!editable}>
				<BlockFields
					{schema}
					component={comp}
					values={fields.value}
					errors={{}}
					{refs}
					prefix="{uid}."
					id={attrs.id}
				/>
			</fieldset>
		{/if}
	</ItemCard>
</NodeFrame>

{#snippet handle()}
	{#if editable}
		<DragHandle {sortable} {index} label={t('fields.moveBlock')} />
	{/if}
{/snippet}

{#snippet preview()}
	{blockSummary(comp, fields.value, refs) || t('fields.empty')}
{/snippet}
