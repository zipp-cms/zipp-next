<script lang="ts" generics="T">
	import type { Snippet } from 'svelte';
	import SearchInput from './SearchInput.svelte';

	/**
	 * A search box over a list to pick from, inside a popover: Enter picks
	 * the first one. The parent filters or loads `items` by `search`.
	 */
	let {
		items,
		search = $bindable(''),
		placeholder,
		loading = false,
		key,
		item,
		onpick,
	}: {
		items: T[];
		search?: string;
		placeholder?: string;
		loading?: boolean;
		key: (t: T) => string | number;
		item: Snippet<[T]>;
		onpick: (t: T) => void;
	} = $props();

	function onkeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && items.length) {
			e.preventDefault();
			onpick(items[0]);
		}
	}
</script>

<SearchInput bind:value={search} {placeholder} {onkeydown} />
<div class="results">
	{#each items as t (key(t))}
		<button type="button" class="popup-item" onclick={() => onpick(t)}>
			{@render item(t)}
		</button>
	{:else}
		<p class="empty">{loading ? 'Loading…' : 'Nothing found'}</p>
	{/each}
</div>

<style lang="scss">
	.results {
		display: flex;
		flex-direction: column;
		gap: 2px;
		max-height: 15.75rem;
		margin-top: 0.25rem;
		overflow-y: auto;
	}
</style>
