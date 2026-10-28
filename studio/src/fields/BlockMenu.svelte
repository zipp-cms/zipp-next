<script lang="ts">
	import type { Component } from '../api';

	/**
	 * The list of block types to insert. After a `/` the line itself is
	 * the search and handles the keys, otherwise the menu has its own
	 * search input.
	 */
	let {
		items,
		index = $bindable(0),
		query = $bindable(''),
		search = false,
		onpick,
	}: {
		items: Component[];
		index?: number;
		query?: string;
		search?: boolean;
		onpick: (c: Component) => void;
	} = $props();

	function onkeydown(e: KeyboardEvent) {
		const n = items.length || 1;
		if (e.key === 'ArrowDown') index = (index + 1) % n;
		else if (e.key === 'ArrowUp') index = (index - 1 + n) % n;
		else if (e.key === 'Enter' && items[index]) onpick(items[index]);
		else return;
		e.preventDefault();
	}

	function focus(el: HTMLInputElement) {
		el.focus();
	}
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
	class="menu"
	role="listbox"
	tabindex="-1"
	onclick={e => e.stopPropagation()}
>
	{#if search}
		<input
			use:focus
			placeholder="Search blocks"
			bind:value={query}
			oninput={() => (index = 0)}
			{onkeydown}
		/>
	{/if}
	{#each items as c, i (c.handle)}
		<button
			class:active={i === index}
			onmouseenter={() => (index = i)}
			onmousedown={e => e.preventDefault()}
			onclick={() => onpick(c)}
		>
			{c.name}
		</button>
	{:else}
		<p>No blocks found</p>
	{/each}
</div>

<style>
	.menu {
		position: absolute;
		z-index: 5;
		top: 100%;
		left: 0;
		width: 15rem;
		margin-top: 0.15rem;
		padding: 0.3rem;
		background: white;
		border: 1px solid var(--line);
		border-radius: 0.4rem;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
		display: flex;
		flex-direction: column;
	}

	input {
		margin-bottom: 0.25rem;
	}

	button {
		border: none;
		text-align: left;
		padding: 0.35rem 0.6rem;
		background: none;
	}

	.active {
		background: var(--side);
	}

	p {
		margin: 0.3rem 0.6rem;
		color: var(--muted);
	}
</style>
