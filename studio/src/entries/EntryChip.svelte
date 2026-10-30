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
		class="open"
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
	// a row like an input, the actions on the right
	:global(.chip) {
		display: flex;
		align-items: center;
		gap: 10px;
		min-height: 36px;
		padding: 2px 4px 2px 12px;
	}

	.open {
		display: flex;
		align-items: baseline;
		gap: 10px;
		flex: 1;
		min-width: 0;
		padding: 4px 0;
		border: none;
		background: none;
		font-weight: 500;
		text-align: left;

		&:hover:not(:disabled) {
			background: none;
			color: var(--accent);
		}
	}
</style>
