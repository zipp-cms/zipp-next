<script lang="ts">
	import type { Component } from '../core/schema';
	import AddGroup from 'zipp-ui/AddGroup.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import Popover from 'zipp-ui/Popover.svelte';
	import SearchList from 'zipp-ui/SearchList.svelte';

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
		<Popover bind:open align="center" width="14rem" label="Add a block">
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
		<AddGroup
			options={buttons.map(t => ({ value: t.handle, label: t.name }))}
			onadd={pick}
		>
			{#snippet more()}
				{#if rest.length}
					<Popover bind:open width="14rem" label="More blocks">
						{#snippet trigger({ toggle: t })}
							<button type="button" onclick={() => toggle(t)}>
								More
								<Icon name="chevron-down" size={14} />
							</button>
						{/snippet}
						{@render list()}
					</Popover>
				{/if}
			{/snippet}
		</AddGroup>
	</div>
{/if}

<style lang="scss">
	.bar {
		margin-top: 0.5rem;
	}

	// between two blocks, it shows when the pointer is near
	.inline {
		position: relative;
		z-index: var(--z-raised);
		display: flex;
		justify-content: center;
		height: 0.625rem;
		margin: -1px 0;
	}

	.insert {
		margin-top: -0.375rem;
		padding: 2px;
		border-radius: 50%;
		color: var(--accent);
	}

	// only where a pointer can come near, always on touch
	@include hover {
		.insert {
			opacity: 0;
			transition: opacity 0.15s;
		}

		.inline:hover .insert,
		.insert:focus-visible,
		.open .insert {
			opacity: 1;
		}
	}
</style>
