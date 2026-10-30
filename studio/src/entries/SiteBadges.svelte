<script lang="ts">
	import type { Schema } from '../core/schema';
	import { site } from '../core/route.svelte';

	/**
	 * Whether an entry is live in each site: a badge per site, grey where
	 * it isn't published, the site the studio edits marked. With
	 * `onselect` a click on one switches to its site.
	 */
	let {
		schema,
		published,
		onselect,
	}: {
		schema: Schema;
		/** published per site handle */
		published: Record<string, boolean> | undefined;
		onselect?: (site: string) => void;
	} = $props();
</script>

<span class="sites">
	{#each schema.sites as s (s.handle)}
		{@const on = published?.[s.handle] ?? false}
		{@const title = `${s.name}: ${on ? 'published' : 'not published'}`}
		{#if onselect}
			<button
				type="button"
				class="badge"
				class:draft={!on}
				class:current={s.handle === site.current}
				title="{title}, open"
				onclick={() => onselect(s.handle)}
			>
				{s.handle.toUpperCase()}
			</button>
		{:else}
			<span
				class="badge"
				class:draft={!on}
				class:current={s.handle === site.current}
				{title}
			>
				{s.handle.toUpperCase()}
			</span>
		{/if}
	{/each}
</span>

<style>
	.sites {
		display: inline-flex;
		flex-wrap: wrap;
		gap: 4px;
	}

	button {
		border: none;
		cursor: pointer;
	}

	.current {
		outline: 1px solid currentColor;
		outline-offset: -1px;
	}
</style>
