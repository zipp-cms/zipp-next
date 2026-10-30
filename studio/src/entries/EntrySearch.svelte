<script lang="ts">
	import { type Schema } from '../core/schema';
	import { listEntries, type EntryRef } from './api';
	import { site } from '../core/router.svelte';
	import AddButton from '../ui/AddButton.svelte';
	import Popover from '../ui/Popover.svelte';
	import SearchList from '../ui/SearchList.svelte';
	import { componentName } from '../core/schema';

	/**
	 * Chooses entries for relations and links: a button that opens the
	 * entries of the types, newest or in their order, with a search.
	 * With `multiple` it stays open to choose more.
	 */
	let {
		schema,
		types,
		exclude = [],
		multiple = false,
		onpick,
	}: {
		schema: Schema;
		/** the entry types to choose from */
		types: string[];
		exclude?: number[];
		multiple?: boolean;
		onpick: (e: EntryRef) => void;
	} = $props();

	let open = $state(false);
	let search = $state('');
	let found = $state<EntryRef[]>([]);
	let loading = $state(false);

	const typeName = (t: string) => componentName(schema, t);
	// "Choose Seite" with one type, "Choose entry" with several
	const what = $derived(types.length === 1 ? typeName(types[0]) : 'entry');

	$effect(() => {
		if (!open) return;
		const q = search;
		loading = true;
		const params: Record<string, string> = {
			type: types.join(','),
			limit: '30',
			site: site.current,
		};
		if (q) params.search = q;
		listEntries(params)
			.then(r => {
				if (q === search) found = r.entries;
			})
			.catch(() => {
				if (q === search) found = [];
			})
			// only the newest search ends the loading
			.finally(() => {
				if (q === search) loading = false;
			});
	});

	const shown = $derived(found.filter(e => !exclude.includes(e.id)));

	function pick(e: EntryRef) {
		onpick(e);
		if (!multiple) open = false;
	}
</script>

<Popover bind:open width="26rem" label="Choose {what}">
	{#snippet trigger({ toggle, open })}
		<AddButton onclick={() => ((search = ''), toggle())} expanded={open}>
			Choose {what}
		</AddButton>
	{/snippet}
	<SearchList
		items={shown}
		bind:search
		{loading}
		placeholder="Search {types.map(typeName).join(', ')}"
		key={e => e.id}
		onpick={pick}
	>
		{#snippet item(e)}
			<span class="truncate">{e.title}</span>
			<span class="note type">{typeName(e.type)}</span>
		{/snippet}
	</SearchList>
	{#if multiple}
		<div class="foot">
			<button type="button" onclick={() => (open = false)}>Done</button>
		</div>
	{/if}
</Popover>

<style>
	.type {
		margin-left: auto;
		flex-shrink: 0;
	}

	.foot {
		display: flex;
		justify-content: flex-end;
		padding-top: 6px;
		border-top: 1px solid var(--line);
	}
</style>
