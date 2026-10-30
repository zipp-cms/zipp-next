<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { Schema } from '../core/schema';
	import type { Refs } from './api';
	import { openEntry } from '../core/flyout.svelte';
	import { componentName } from '../core/schema';

	/**
	 * A chosen entry, like Craft's element chips: its title opens it in
	 * the flyout, its type next to it, the actions on the right.
	 */
	let {
		schema,
		refs,
		id,
		actions,
	}: {
		schema: Schema;
		refs: Refs;
		id: number;
		actions: Snippet;
	} = $props();

	const entry = $derived(refs[id]);
</script>

<div class="chip box">
	<button
		type="button"
		class="open plain"
		title="Open in the flyout"
		onclick={() => openEntry(id)}
	>
		<span class="truncate">{entry?.title ?? `#${id}`}</span>
		{#if entry}
			<span class="note">{componentName(schema, entry.type)}</span>
		{/if}
	</button>
	{@render actions()}
</div>

<style lang="scss">
	// the type on the line of the title
	.open {
		align-items: baseline;
		padding: 4px 0;
	}
</style>
