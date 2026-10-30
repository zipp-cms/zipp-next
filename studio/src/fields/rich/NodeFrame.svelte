<script lang="ts">
	import type { Snippet } from 'svelte';
	import Icon from 'zipp-ui/Icon.svelte';

	/**
	 * Something in a rich text that isn't text (a block, a button), with a
	 * button on its edges for an empty line above or below.
	 */
	let {
		editable,
		oninsert,
		children,
	}: {
		editable: boolean;
		/** an empty line before or after, the cursor goes there */
		oninsert: (where: 'before' | 'after') => void;
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
	{@render children()}
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
