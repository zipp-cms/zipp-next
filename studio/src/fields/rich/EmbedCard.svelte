<script lang="ts">
	import type { Snippet } from 'svelte';
	import Icon from '../../ui/Icon.svelte';
	import ItemActions from '../../ui/ItemActions.svelte';
	import ItemCard from '../../ui/ItemCard.svelte';

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
			class="line before"
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
			class="line after"
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
		margin: 12px 0;
		font-size: var(--fs-base);
		line-height: 1.45;
		white-space: normal;
	}

	// on the edge of the card, when the pointer is on it
	.line {
		position: absolute;
		left: 50%;
		z-index: 2;
		gap: 3px;
		padding: 1px 8px;
		font-size: var(--fs-xs);
		color: var(--accent);
		border-color: var(--border);
		border-radius: 999px;
		transform: translate(-50%, -50%);
		opacity: 0;
		transition: opacity 0.15s;

		&.before {
			top: 0;
		}

		&.after {
			top: 100%;
		}
	}

	.embed:hover > .line,
	.line:focus-visible {
		opacity: 1;
	}

	@media (hover: none) {
		.line {
			opacity: 1;
		}
	}
</style>
