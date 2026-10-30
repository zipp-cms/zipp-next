<script lang="ts">
	import type { Component, Schema, Values } from '../core/schema';
	import Fields from '../fields/Fields.svelte';
	import TabBar from '../entries/edit/TabBar.svelte';

	/**
	 * The editor of an entry type or block as editors see it, empty and
	 * read only: the tabs, the title, the fields.
	 */
	let { schema, component }: { schema: Schema; component: Component } =
		$props();

	const tabs = $derived(component.tabs ?? []);
	let tab = $state('');
	$effect(() => {
		if (tabs.length && !tabs.some(t => t.name === tab)) tab = tabs[0].name;
	});
	// fields may fill in their empty values, they stay here
	let values = $state<Values>({});
</script>

{#if tabs.length}
	<TabBar {tabs} bind:current={tab} />
{/if}
<!-- a disabled fieldset disables every input in it -->
<fieldset disabled>
	{#if component.entry && (!tabs.length || tab === tabs[0].name)}
		<label class="title">
			<span>Title</span>
			<input placeholder="Title" />
		</label>
	{/if}
	<Fields
		{schema}
		{component}
		{values}
		errors={{}}
		refs={{}}
		only={tabs.find(t => t.name === tab)?.fields ?? null}
	/>
</fieldset>

<style lang="scss">
	.title {
		display: flex;
		flex-direction: column;
		gap: 0.625rem;
		margin-bottom: 1.875rem;
	}
</style>
