<script module lang="ts">
	/** How long a list waits after the last key before it searches. */
	export const SEARCH_DELAY_MS = 250;
</script>

<script lang="ts">
	import Icon from './Icon.svelte';

	/** A search box with its icon, for lists and pickers. */
	let {
		value = $bindable(''),
		placeholder = 'Search',
		onkeydown,
	}: {
		value?: string;
		placeholder?: string;
		onkeydown?: (e: KeyboardEvent) => void;
	} = $props();
</script>

<span class="search">
	<Icon name="search" size={14} />
	<input type="search" {placeholder} bind:value {onkeydown} />
</span>

<style lang="scss">
	.search {
		position: relative;
		display: flex;
		align-items: center;
		color: var(--subtle);

		:global(.icon) {
			position: absolute;
			left: 0.625rem;
			pointer-events: none;
		}

		input {
			padding-left: 2rem;
		}
	}
</style>
