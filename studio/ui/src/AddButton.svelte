<script lang="ts">
	import type { Snippet } from 'svelte';
	import Icon from './Icon.svelte';

	/** Adds something to a field: an entry, a file, a link, a row. */
	let {
		onclick,
		expanded,
		children,
	}: {
		onclick: () => void;
		expanded?: boolean;
		children: Snippet;
	} = $props();
</script>

<button type="button" class="add" {onclick} aria-expanded={expanded}>
	<Icon name="add" size={14} />
	{@render children()}
</button>

<style lang="scss">
	.add {
		border-style: dashed;
		color: var(--accent);
	}
</style>
