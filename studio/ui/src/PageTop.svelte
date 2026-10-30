<script lang="ts">
	import type { Snippet } from 'svelte';
	import Crumbs from './Crumbs.svelte';
	import { flyoutBar } from './flyoutBar';

	/**
	 * The bar at the top of every page: where it is (crumbs or a title), a
	 * note, the state of what it shows, and its actions on the right. It
	 * sticks to the top while the page scrolls.
	 */
	let {
		crumbs,
		title,
		note,
		state,
		actions,
		bleed = true,
	}: {
		crumbs?: { label: string; href?: string }[];
		title?: string;
		note?: string;
		state?: Snippet;
		actions?: Snippet;
		/** reaches to the edges of the page's padding */
		bleed?: boolean;
	} = $props();

	// in a flyout it is the flyout's bar
	const host = flyoutBar();
	$effect(() => {
		if (!host) return;
		host.content = inner;
		return () => (host.content = null);
	});
</script>

{#snippet inner()}
	<h1>
		{#if crumbs}<Crumbs items={crumbs} />{:else}{title}{/if}
	</h1>
	{#if note}<span class="note">{note}</span>{/if}
	{#if state}<span class="state">{@render state()}</span>{/if}
	<span class="actions">{@render actions?.()}</span>
{/snippet}

{#if !host}
	<div class="page-top" class:bleed>{@render inner()}</div>
{/if}

<style lang="scss">
	.page-top {
		position: sticky;
		top: 0;
		z-index: var(--z-bar);
		display: flex;
		align-items: center;
		gap: 0.5rem;
		flex-wrap: wrap;
		min-height: 3rem;
		padding: 0.5rem 1rem;
		background: var(--surface);
		border-bottom: 1px solid var(--line);
	}

	.bleed {
		margin: 0 calc(-1 * var(--page-pad)) 1.25rem;
		padding: 0.5rem var(--page-pad);
	}

	h1 {
		min-width: 0;
		font-size: var(--fs-lg);
		line-height: 1.25rem;
	}

	.state {
		display: flex;
		align-items: center;
		gap: 0.375rem;
	}

	.actions {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-left: auto;
	}
</style>
