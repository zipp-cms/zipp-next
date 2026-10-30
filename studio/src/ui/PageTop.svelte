<script lang="ts">
	import type { Snippet } from 'svelte';
	import Crumbs from './Crumbs.svelte';

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
</script>

<div class="page-top" class:bleed>
	<h1>
		{#if crumbs}<Crumbs items={crumbs} />{:else}{title}{/if}
	</h1>
	{#if note}<span class="note">{note}</span>{/if}
	{#if state}<span class="state">{@render state()}</span>{/if}
	<span class="actions">{@render actions?.()}</span>
</div>

<style>
	.page-top {
		position: sticky;
		top: 0;
		z-index: var(--z-bar);
		display: flex;
		align-items: center;
		gap: 8px;
		flex-wrap: wrap;
		min-height: 48px;
		padding: 8px 16px;
		background: var(--surface);
		border-bottom: 1px solid var(--line);
	}

	.bleed {
		margin: 0 calc(-1 * var(--page-pad)) 20px;
		padding: 8px var(--page-pad);
	}

	h1 {
		min-width: 0;
		font-size: var(--fs-lg);
		line-height: 20px;
	}

	.state {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.actions {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-left: auto;
	}
</style>
