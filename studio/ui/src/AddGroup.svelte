<script lang="ts" generics="T extends string">
	import type { Snippet } from 'svelte';
	import Icon from './Icon.svelte';

	/**
	 * Adds one of a few kinds, side by side like one control: `+ Entry |
	 * URL`, `+ Text | Images | …`. `more` goes at the end, like a popover
	 * with the rest.
	 */
	let {
		options,
		onadd,
		more,
	}: {
		options: { value: T; label: string }[];
		onadd: (value: T) => void;
		more?: Snippet;
	} = $props();
</script>

<span class="add-group">
	{#each options as o, i (o.value)}
		<button type="button" onclick={() => onadd(o.value)}>
			{#if i === 0}<Icon name="add" size={14} />{/if}
			{o.label}
		</button>
	{/each}
	{@render more?.()}
</span>

<style lang="scss">
	// one border around all, a line between them
	.add-group {
		display: inline-flex;
		flex-wrap: wrap;
		flex-shrink: 0;
		max-width: 100%;
		border: 1px solid var(--border-strong);
		border-radius: var(--radius);
		background: var(--surface);

		:global(button) {
			border: none;
			border-radius: 0;
			border-right: 1px solid var(--line);
			color: var(--text);
		}

		// the hover stays inside the corners; no overflow: hidden, it would
		// cut the popover of `more`
		> button:first-child {
			border-radius: var(--radius) 0 0 var(--radius);
		}

		> :last-child,
		> :last-child :global(button) {
			border-right: none;
			border-radius: 0 var(--radius) var(--radius) 0;
		}

		> button:only-child {
			border-radius: var(--radius);
		}

		> button:first-child :global(.icon) {
			color: var(--accent);
		}
	}
</style>
