<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { Schema } from '../core/schema';
	import type { Refs } from './api';
	import { openEntry } from '../core/flyout.svelte';
	import { componentName } from '../core/schema';
	import { t } from '../i18n/index.svelte';

	/**
	 * A chosen entry, like Craft's element chips: compact, side by side
	 * with others. Its handle, its title opens it in the flyout, its type
	 * next to it, the actions on the right.
	 */
	let {
		schema,
		refs,
		id,
		handle,
		actions,
	}: {
		schema: Schema;
		refs: Refs;
		id: number;
		handle?: Snippet;
		actions: Snippet;
	} = $props();

	const entry = $derived(refs[id]);
</script>

<div class="chip box">
	{@render handle?.()}
	<button
		type="button"
		class="open plain"
		title={t('entries.openInFlyout')}
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
	// as high as the Choose button next to it, the handle and the
	// actions close to the edges
	.chip {
		gap: 0.375rem;
		min-height: 2.125rem;
		padding: 0 0.125rem;
	}

	// the type on the line of the title, it never shrinks: the title
	// takes the ellipsis
	.open {
		align-items: baseline;
		gap: 0.5rem;
		padding: 0.25rem 0;

		&:first-child {
			padding-left: 0.5rem;
		}

		.note {
			flex-shrink: 0;
			white-space: nowrap;
		}
	}
</style>
