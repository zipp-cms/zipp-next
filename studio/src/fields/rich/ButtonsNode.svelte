<script lang="ts">
	import { untrack } from 'svelte';
	import type { Link, Schema } from '../../core/schema';
	import type { Refs } from '../../entries/api';
	import LinkInput from '../LinkInput.svelte';
	import EmbedCard from './EmbedCard.svelte';
	import AddButton from '../../ui/AddButton.svelte';
	import ItemActions from '../../ui/ItemActions.svelte';
	import { BUTTON_LINKS, linkLabel, linkTarget } from '../link';
	import { moved, removed } from '../../core/list';
	import { synced } from '../../core/synced.svelte';

	/** A row of buttons in a rich text, each one a link with a label. */
	let {
		attrs,
		schema,
		refs,
		editable,
		update,
		remove,
		insertLine,
	}: {
		attrs: { links: Link[] };
		schema: Schema;
		refs: Refs;
		editable: boolean;
		update: (attrs: Record<string, unknown>) => void;
		remove: () => void;
		insertLine: (where: 'before' | 'after') => void;
	} = $props();

	// changes from the text (undo) come in, changes of the buttons go out
	const buttons = synced(
		() => attrs.links,
		links => update({ links }),
	);

	// a new row without a target opens to be filled in
	let editing = $state(
		untrack(
			() =>
				!attrs.links.length || attrs.links.some(l => l.value === null),
		),
	);

	const summary = $derived(
		buttons.value
			.map(
				link =>
					`${linkLabel(link, refs, 'Button')} → ${linkTarget(link, refs)}`,
			)
			.join(' · ') || 'No buttons yet',
	);
</script>

<EmbedCard
	kind="Buttons"
	{editable}
	bind:editing
	onremove={remove}
	oninsert={insertLine}
>
	{#snippet preview()}{summary}{/snippet}
	<fieldset disabled={!editable} class="buttons">
		{#each buttons.value as item, i (i)}
			<div class="item">
				<LinkInput
					{schema}
					{refs}
					link={item}
					to={BUTTON_LINKS}
					labels
					onchange={link => (buttons.value[i] = link)}
				/>
				<ItemActions
					index={i}
					count={buttons.value.length}
					onmove={by => (buttons.value = moved(buttons.value, i, by))}
					onremove={() => (buttons.value = removed(buttons.value, i))}
					removeTitle="Remove the button"
				/>
			</div>
		{/each}
		<div>
			<AddButton
				onclick={() =>
					buttons.value.push({ type: 'entry', value: null })}
			>
				Add a button
			</AddButton>
		</div>
	</fieldset>
</EmbedCard>

<style>
	.buttons {
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	.item {
		display: flex;
		gap: 8px;
		align-items: flex-start;
		padding-bottom: 10px;
		border-bottom: 1px solid var(--line);

		> :global(:first-child) {
			flex: 1;
		}
	}
</style>
