<script lang="ts">
	import type { Snippet } from 'svelte';
	import ItemActions from 'zipp-ui/ItemActions.svelte';
	import ItemCard from 'zipp-ui/ItemCard.svelte';
	import NodeFrame from './NodeFrame.svelte';

	/** A block in a rich text: its card, opened to edit its fields. */
	let {
		kind,
		editable,
		editing = $bindable(false),
		onremove,
		oninsert,
		preview,
		handle,
		children,
	}: {
		/** the name of the block */
		kind: string;
		editable: boolean;
		editing?: boolean;
		onremove: () => void;
		/** an empty line before or after, the cursor goes there */
		oninsert: (where: 'before' | 'after') => void;
		preview: Snippet;
		/** the DragHandle, at the start of the head */
		handle?: Snippet;
		children: Snippet;
	} = $props();
</script>

<NodeFrame {editable} {oninsert}>
	<ItemCard {kind} bind:open={editing} summary={preview} {handle}>
		{#snippet actions()}
			{#if editable}
				<ItemActions {onremove} />
			{/if}
		{/snippet}
		{@render children()}
	</ItemCard>
</NodeFrame>
