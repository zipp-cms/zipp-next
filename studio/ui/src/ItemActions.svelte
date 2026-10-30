<script lang="ts">
	import Icon from './Icon.svelte';

	/**
	 * What can be done with an item of a list besides moving it (that's
	 * the DragHandle): duplicate it, remove it. Every list of a field has
	 * these, in this order.
	 */
	let {
		onduplicate,
		onremove,
		removeTitle = 'Remove',
	}: {
		onduplicate?: () => void;
		onremove: () => void;
		removeTitle?: string;
	} = $props();
</script>

<span class="item-actions">
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

<style lang="scss">
	@use '../vars' as *;

	.item-actions {
		display: flex;
		flex-shrink: 0;
	}

	@include hover {
		.remove:hover:not(:disabled) {
			color: var(--danger);
		}
	}
</style>
