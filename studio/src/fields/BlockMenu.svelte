<script lang="ts">
	import type { Component } from '../api';
	import Icon from '../lib/Icon.svelte';
	import Popover from '../lib/Popover.svelte';
	import SearchList from '../lib/SearchList.svelte';

	/**
	 * Adds a block: as a bar with the types (the first ones as buttons,
	 * the rest under More), or `inline` a small + between blocks.
	 */
	let {
		types,
		onpick,
		inline = false,
	}: {
		types: Component[];
		onpick: (type: string) => void;
		inline?: boolean;
	} = $props();

	// as many buttons as fit a line, the rest in the popover
	const SHOWN = 6;
	const buttons = $derived(inline ? [] : types.slice(0, SHOWN));
	const rest = $derived(inline ? types : types.slice(SHOWN));

	let open = $state(false);
	let search = $state('');

	const found = $derived(
		rest.filter(t => t.name.toLowerCase().includes(search.toLowerCase())),
	);

	function toggle(popover: () => void) {
		// with a single type there is nothing to choose
		if (inline && types.length === 1) return onpick(types[0].handle);
		search = '';
		popover();
	}

	function pick(handle: string) {
		open = false;
		onpick(handle);
	}
</script>

{#snippet list()}
	{#if rest.length > 5}
		<SearchList
			items={found}
			bind:search
			placeholder="Search blocks"
			key={t => t.handle}
			onpick={t => pick(t.handle)}
		>
			{#snippet item(t)}{t.name}{/snippet}
		</SearchList>
	{:else}
		{#each found as t (t.handle)}
			<button
				type="button"
				class="popup-item"
				onclick={() => pick(t.handle)}
			>
				{t.name}
			</button>
		{/each}
	{/if}
{/snippet}

{#if inline}
	<div class="inline" class:open>
		<Popover bind:open align="center" width="16rem" label="Add a block">
			{#snippet trigger({ toggle: t })}
				<button
					type="button"
					class="insert"
					title="Add a block here"
					onclick={() => toggle(t)}
				>
					<Icon name="add" size={14} />
				</button>
			{/snippet}
			{@render list()}
		</Popover>
	</div>
{:else}
	<div class="bar">
		{#each buttons as t, i (t.handle)}
			<button type="button" onclick={() => pick(t.handle)}>
				{#if i === 0}<Icon name="add" size={14} />{/if}
				{t.name}
			</button>
		{/each}
		{#if rest.length}
			<Popover bind:open width="16rem" label="More blocks">
				{#snippet trigger({ toggle: t })}
					<button type="button" onclick={() => toggle(t)}>
						More
						<Icon name="chevron-down" size={14} />
					</button>
				{/snippet}
				{@render list()}
			</Popover>
		{/if}
	</div>
{/if}

<style lang="scss">
	// the types side by side like one control
	.bar {
		display: inline-flex;
		flex-wrap: wrap;
		margin-top: 8px;
		border: 1px solid var(--border-strong);
		border-radius: var(--radius);
		background: var(--surface);

		button {
			border: none;
			border-radius: 0;
			border-right: 1px solid var(--line);
		}

		> :last-child,
		> :last-child :global(> button) {
			border-right: none;
		}
	}

	// between two blocks, it shows when the pointer is near
	.inline {
		position: relative;
		z-index: 2;
		display: flex;
		justify-content: center;
		height: 10px;
		margin: -1px 0;
	}

	.insert {
		margin-top: -6px;
		padding: 2px;
		border-radius: 50%;
		color: var(--accent);
		opacity: 0;
		transition: opacity 0.15s;
	}

	.inline:hover .insert,
	.insert:focus-visible,
	.open .insert {
		opacity: 1;
	}
</style>
