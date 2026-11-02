<script lang="ts">
	import type { Component } from '../core/schema';
	import SchemaIcon from '../core/SchemaIcon.svelte';
	import SearchList from 'zipp-ui/SearchList.svelte';
	import { plain } from '../core/format';
	import { t } from '../i18n/index.svelte';

	/**
	 * The block types to pick from in a popover, with a search when there
	 * are many. The search starts empty each time the popover opens.
	 */
	let {
		types,
		onpick,
	}: {
		types: Component[];
		onpick: (type: string) => void;
	} = $props();

	let search = $state('');
	const found = $derived(
		types.filter(c => plain(c.name).includes(plain(search))),
	);
</script>

{#if types.length > 5}
	<SearchList
		items={found}
		bind:search
		placeholder={t('fields.searchBlocks')}
		key={c => c.handle}
		onpick={c => onpick(c.handle)}
	>
		{#snippet item(c)}
			<SchemaIcon icon={c.icon} size={14} />{c.name}
		{/snippet}
	</SearchList>
{:else}
	{#each types as c (c.handle)}
		<button
			type="button"
			class="popup-item"
			onclick={() => onpick(c.handle)}
		>
			<SchemaIcon icon={c.icon} size={14} />
			{c.name}
		</button>
	{/each}
{/if}
