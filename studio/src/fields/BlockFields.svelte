<script lang="ts">
	import { untrack } from 'svelte';
	import type { Component, Schema, Values } from '../core/schema';
	import type { Refs } from '../entries/api';
	import TabBar from './TabBar.svelte';
	import { Tabs } from './tabs.svelte';
	import Fields from './Fields.svelte';
	import { lastTab, rememberTab } from './opened';

	/**
	 * The fields of a block, in its tabs when it has some: a bar on top
	 * like the entry's, the tab it showed is kept by the block's id.
	 */
	let {
		schema,
		component,
		values,
		errors,
		refs,
		prefix,
		changed = new Set(),
		id,
	}: {
		schema: Schema;
		component: Component;
		values: Values;
		errors: Record<string, string>;
		refs: Refs;
		prefix: string;
		changed?: Set<string>;
		/** the block's id, to remember its tab */
		id?: string | null;
	} = $props();

	// the tab it showed last in this browser
	const tabs = new Tabs(
		() => component,
		() => values,
		untrack(() => lastTab(id)),
	);

	// the bar counts by the field's handle, without the block's path
	const inBlock = (p: string) => p.startsWith(prefix);
	const strip = (p: string) => p.slice(prefix.length);
	const tabErrors = $derived(
		Object.fromEntries(
			Object.entries(errors)
				.filter(([p]) => inBlock(p))
				.map(([p, e]) => [strip(p), e]),
		),
	);
	const tabChanged = $derived(
		new Set([...changed].filter(inBlock).map(strip)),
	);
</script>

{#if tabs.shown.length}
	<div class="tabs">
		<TabBar
			tabs={tabs.shown}
			bind:current={
				() => tabs.current,
				v => {
					tabs.current = v;
					rememberTab(id, v);
				}
			}
			changed={tabChanged}
			errors={tabErrors}
		/>
	</div>
{/if}
<Fields
	{schema}
	{component}
	{values}
	{errors}
	{refs}
	{prefix}
	{changed}
	only={tabs.fields}
/>

<style lang="scss">
	.tabs {
		margin-bottom: 0.875rem;
	}
</style>
