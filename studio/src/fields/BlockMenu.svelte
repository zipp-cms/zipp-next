<script lang="ts">
	import Icon from '../lib/Icon.svelte';
	import type { Component } from '../api';

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

	// as many buttons as fit a line, the rest in the popup
	const SHOWN = 6;
	const buttons = $derived(inline ? [] : types.slice(0, SHOWN));
	const rest = $derived(inline ? types : types.slice(SHOWN));

	let open = $state(false);
	let search = $state('');
	let input = $state<HTMLInputElement>();

	const found = $derived(
		rest.filter(t => t.name.toLowerCase().includes(search.toLowerCase())),
	);

	function toggle() {
		// with a single type there is nothing to choose
		if (inline && types.length === 1) return onpick(types[0].handle);
		open = !open;
		search = '';
		if (open) setTimeout(() => input?.focus());
	}

	function pick(handle: string) {
		open = false;
		onpick(handle);
	}

	function onkeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') open = false;
		if (e.key === 'Enter' && found.length) pick(found[0].handle);
	}
</script>

<div class="menu" class:inline>
	{#if inline}
		<button
			type="button"
			class="insert"
			title="Add a block here"
			onclick={toggle}
		>
			<Icon name="add" size={14} />
		</button>
	{:else}
		<div class="bar">
			{#each buttons as t, i (t.handle)}
				<button type="button" onclick={() => pick(t.handle)}>
					{#if i === 0}<Icon name="add" size={14} />{/if}
					{t.name}
				</button>
			{/each}
			{#if rest.length}
				<button type="button" onclick={toggle}>
					More
					<Icon name="chevron-down" size={14} />
				</button>
			{/if}
		</div>
	{/if}
	{#if open}
		<div class="popup">
			{#if rest.length > 5}
				<input
					bind:this={input}
					placeholder="Search blocks"
					bind:value={search}
					{onkeydown}
				/>
			{/if}
			{#each found as t (t.handle)}
				<button
					type="button"
					class="item"
					onclick={() => pick(t.handle)}
				>
					{t.name}
				</button>
			{/each}
		</div>
	{/if}
</div>

<style lang="scss">
	.menu {
		position: relative;
	}

	// the types side by side like one control
	.bar {
		display: inline-flex;
		flex-wrap: wrap;
		margin-top: 8px;
		border: 1px solid var(--border-strong);
		border-radius: var(--radius);
		background: var(--surface);
		overflow: hidden;

		button {
			border: none;
			border-radius: 0;
			border-right: 1px solid var(--line);

			&:last-child {
				border-right: none;
			}
		}
	}

	// between two blocks, it shows when the pointer is near
	.inline {
		display: flex;
		justify-content: center;
		height: 10px;
		margin: -1px 0;
		z-index: 2;
	}

	.insert {
		align-self: center;
		padding: 2px;
		border-radius: 50%;
		color: var(--accent);
		opacity: 0;
		transition: opacity 0.15s;
	}

	.inline:hover .insert,
	.insert:focus-visible,
	.inline:has(.popup) .insert {
		opacity: 1;
	}

	.popup {
		position: absolute;
		z-index: 20;
		top: 100%;
		left: 0;
		margin-top: 4px;
		width: 16rem;
		padding: 6px;
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
		box-shadow: var(--shadow-pop);
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.inline .popup {
		left: 50%;
		transform: translateX(-50%);
	}

	.item {
		border: none;
		text-align: left;
		font-weight: 400;
	}
</style>
