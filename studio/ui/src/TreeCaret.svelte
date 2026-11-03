<script lang="ts">
	import { ui } from './texts.svelte';
	import Icon from './Icon.svelte';

	/**
	 * Opens and closes a row of a tree. Without `open` the row has
	 * nothing below and it is only the room, so the names line up.
	 */
	let {
		open,
		onclick,
	}: {
		open?: boolean;
		onclick?: () => void;
	} = $props();
</script>

{#if open === undefined}
	<span class="caret"></span>
{:else}
	<button
		type="button"
		class="caret"
		aria-label={open ? ui.texts.collapse : ui.texts.expand}
		aria-expanded={open}
		{onclick}
	>
		<Icon name={open ? 'chevron-down' : 'chevron-right'} size={14} />
	</button>
{/if}

<style lang="scss">
	.caret {
		display: grid;
		flex-shrink: 0;
		place-items: center;
		width: 1.25rem;
		height: 1.25rem;
		padding: 0;
		border: none;
		background: none;
		color: inherit;
	}
</style>
