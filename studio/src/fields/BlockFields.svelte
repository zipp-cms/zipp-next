<script lang="ts">
	import { untrack } from 'svelte';
	import type { Component, Schema, Values } from '../core/schema';
	import type { Refs } from '../entries/api';
	import TabBar from './TabBar.svelte';
	import { tabShown } from './conditions';
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

	// the tabs whose conditions hold, the first one open
	const tabs = $derived(
		(component.tabs ?? []).filter(t => tabShown(component, t, values)),
	);
	let tab = $state(untrack(() => lastTab(id)) ?? '');
	$effect(() => {
		if (tabs.length && !tabs.some(t => t.name === tab)) tab = tabs[0].name;
	});
	const only = $derived(tabs.find(t => t.name === tab)?.fields ?? null);

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

{#if tabs.length}
	<div class="tabs">
		<TabBar
			{tabs}
			bind:current={
				() => tab,
				v => {
					tab = v;
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
	{only}
/>

<style lang="scss">
	.tabs {
		margin-bottom: 0.875rem;
	}
</style>
