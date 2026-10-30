<script lang="ts">
	import Icon from './Icon.svelte';

	/**
	 * What can be done with an item of a list: move it, duplicate it,
	 * remove it. Every list of a field has these, in this order.
	 */
	let {
		index,
		count,
		onmove,
		onduplicate,
		onremove,
		removeTitle = 'Remove',
		horizontal = false,
	}: {
		index: number;
		count: number;
		/** without it the list has no order to change */
		onmove?: (by: number) => void;
		onduplicate?: () => void;
		onremove: () => void;
		removeTitle?: string;
		/** the items are side by side, like images */
		horizontal?: boolean;
	} = $props();
</script>

<span class="item-actions" class:horizontal>
	{#if onmove && count > 1}
		<button
			type="button"
			class="icon"
			title={horizontal ? 'Move left' : 'Move up'}
			disabled={index === 0}
			onclick={() => onmove(-1)}
		>
			<span class="arrow"><Icon name="arrow-up" size={15} /></span>
		</button>
		<button
			type="button"
			class="icon"
			title={horizontal ? 'Move right' : 'Move down'}
			disabled={index === count - 1}
			onclick={() => onmove(1)}
		>
			<span class="arrow"><Icon name="arrow-down" size={15} /></span>
		</button>
	{/if}
	{#if onduplicate}
		<button
			type="button"
			class="icon"
			title="Duplicate"
			onclick={onduplicate}
		>
			<Icon name="copy" size={15} />
		</button>
	{/if}
	<button
		type="button"
		class="icon remove"
		title={removeTitle}
		onclick={onremove}
	>
		<Icon name="delete" size={15} />
	</button>
</span>

<style>
	.item-actions {
		display: flex;
		flex-shrink: 0;
	}

	.arrow {
		display: inline-flex;
	}

	/* side by side the arrows point left and right */
	.horizontal .arrow {
		transform: rotate(-90deg);
	}

	.remove:hover:not(:disabled) {
		color: var(--danger);
	}
</style>
