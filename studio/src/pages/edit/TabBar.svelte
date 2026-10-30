<script lang="ts">
	import type { Tab } from '../../api';
	import Segmented from '../../lib/Segmented.svelte';

	/**
	 * The tabs of an entry type, like Craft's: a dot on a tab with changes,
	 * the number of its errors in red, so nothing hides in another tab.
	 */
	let {
		tabs,
		current = $bindable(),
		changed = new Set(),
		errors = {},
	}: {
		tabs: Tab[];
		current: string;
		changed?: Set<string>;
		errors?: Record<string, string>;
	} = $props();

	// a path like `body.0.text` belongs to the tab of `body`
	const count = (tab: Tab, paths: Iterable<string>) =>
		[...paths].filter(p => tab.fields.includes(p.split('.')[0])).length;
</script>

<Segmented
	tabs
	options={tabs.map(t => ({ value: t.name, label: t.name }))}
	value={current}
	onchange={v => (current = v)}
>
	{#snippet item({ value })}
		{@const tab = tabs.find(t => t.name === value)!}
		{@const errs = count(tab, Object.keys(errors))}
		{#if count(tab, changed)}<span class="dot" title="Changed"></span>{/if}
		{value}
		{#if errs}
			<span class="errors" title="{errs} errors">{errs}</span>
		{/if}
	{/snippet}
</Segmented>

<style lang="scss">
	.dot {
		margin: 0;
	}

	.errors {
		min-width: 18px;
		padding: 0 5px;
		border-radius: 999px;
		background: var(--danger);
		color: white;
		font-size: var(--fs-xs);
		font-weight: 600;
		line-height: 17px;
		text-align: center;
	}
</style>
