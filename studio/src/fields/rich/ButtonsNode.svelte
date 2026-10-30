<script lang="ts">
	import { untrack } from 'svelte';
	import type { Link, Schema } from '../../core/schema';
	import type { Refs } from '../../entries/api';
	import LinkField from '../LinkField.svelte';
	import EmbedCard from './EmbedCard.svelte';
	import AddButton from '../../ui/AddButton.svelte';
	import ItemActions from '../../ui/ItemActions.svelte';
	import { linkField, linkLabel, linkTarget } from '../link';
	import { clone, moved, removed } from '../../core/list';

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

	// what the link field of a button allows
	const field = linkField(['entry', 'url', 'email', 'phone'], {
		label: 'Button',
	});

	let items = $state(
		untrack(() => attrs.links.map(link => ({ link: clone(link) }))),
	);
	let last = JSON.stringify(untrack(() => attrs.links));

	// changes from the text (undo) come in, changes of the buttons go out
	$effect(() => {
		const outer = JSON.stringify(attrs.links);
		if (outer !== last) {
			last = outer;
			items = attrs.links.map(link => ({ link: clone(link) }));
		}
	});
	$effect(() => {
		const links = $state.snapshot(items).map(i => i.link);
		const inner = JSON.stringify(links);
		if (inner === last) return;
		last = inner;
		update({ links });
	});

	// a new row without a target opens to be filled in
	let editing = $state(
		untrack(
			() =>
				!attrs.links.length || attrs.links.some(l => l.value === null),
		),
	);

	const summary = $derived(
		items
			.map(
				({ link }) =>
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
		{#each items as item, i (i)}
			<div class="item">
				<LinkField {schema} {field} values={item} {refs} />
				<ItemActions
					index={i}
					count={items.length}
					onmove={by => (items = moved(items, i, by))}
					onremove={() => (items = removed(items, i))}
					removeTitle="Remove the button"
				/>
			</div>
		{/each}
		<div>
			<AddButton
				onclick={() =>
					items.push({ link: { type: 'entry', value: null } })}
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
