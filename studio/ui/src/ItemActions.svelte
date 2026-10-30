<script lang="ts">
	import Icon from './Icon.svelte';

	/**
	 * What can be done with an item of a list besides moving it (that's
	 * the DragHandle): switch it off and on, duplicate it, remove it.
	 * Every list of a field has these, in this order.
	 */
	let {
		onswitch,
		off = false,
		onduplicate,
		onremove,
		removeLabel = 'Remove',
	}: {
		/** switches it off (the website leaves it out) and on again */
		onswitch?: () => void;
		off?: boolean;
		onduplicate?: () => void;
		onremove: () => void;
		removeLabel?: string;
	} = $props();
</script>

<span class="item-actions">
	{#if onswitch}
		<button
			type="button"
			class="icon"
			title={off ? 'Show it on the website' : 'Hide it on the website'}
			aria-pressed={off}
			onclick={onswitch}
		>
			<Icon name={off ? 'view' : 'hide'} size={15} />
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
		title={removeLabel}
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
