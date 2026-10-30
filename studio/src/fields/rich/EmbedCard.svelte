<script lang="ts">
	import type { Snippet } from 'svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import ItemActions from 'zipp-ui/ItemActions.svelte';
	import ItemCard from 'zipp-ui/ItemCard.svelte';

	/**
	 * Something in a rich text that isn't text: the card of a block, with
	 * a button on its edges for an empty line above or below.
	 */
	let {
		kind,
		editable,
		editing = $bindable(false),
		onremove,
		oninsert,
		preview,
		children,
	}: {
		/** what it is, like Buttons or the name of a block */
		kind: string;
		editable: boolean;
		editing?: boolean;
		onremove: () => void;
		/** an empty line before or after, the cursor goes there */
		oninsert: (where: 'before' | 'after') => void;
		preview: Snippet;
		children: Snippet;
	} = $props();
</script>

<!-- not part of the text: the editor doesn't place the cursor in it -->
<div class="embed" contenteditable="false">
	{#if editable}
		<button
			type="button"
			class="line before reveal"
			title="Add a line above"
			onclick={() => oninsert('before')}
		>
			<Icon name="add" size={12} /> Line
		</button>
	{/if}
	<ItemCard {kind} bind:open={editing} summary={preview}>
		{#snippet actions()}
			{#if editable}
				<ItemActions index={0} count={1} {onremove} />
			{/if}
		{/snippet}
		{@render children()}
	</ItemCard>
	{#if editable}
		<button
			type="button"
			class="line after reveal"
			title="Add a line below"
			onclick={() => oninsert('after')}
		>
			<Icon name="add" size={12} /> Line
		</button>
	{/if}
</div>

<style lang="scss">
	.embed {
		position: relative;
		margin: 0.75rem 0;
		font-size: var(--fs-base);
		line-height: 1.45;
		white-space: normal;
	}

	// on the edge of the card, when the pointer is on it
	.line {
		position: absolute;
		left: 50%;
		z-index: var(--z-raised);
		gap: 3px;
		padding: 1px 0.5rem;
		font-size: var(--fs-xs);
		color: var(--accent);
		border-color: var(--border);
		border-radius: 999px;
		transform: translate(-50%, -50%);

		&.before {
			top: 0;
		}

		&.after {
			top: 100%;
		}
	}
</style>
