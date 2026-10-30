<script lang="ts">
	import type { Snippet } from 'svelte';
	import Icon from './Icon.svelte';

	/** Adds something to a field: an entry, a file, a link, a row. */
	let {
		onclick,
		expanded,
		disabled = false,
		children,
	}: {
		onclick: () => void;
		expanded?: boolean;
		disabled?: boolean;
		children: Snippet;
	} = $props();
</script>

<button type="button" class="add" {onclick} {disabled} aria-expanded={expanded}>
	<Icon name="add" size={14} />
	{@render children()}
</button>

<style>
	.add {
		border-style: dashed;
		color: var(--accent);
	}
</style>
