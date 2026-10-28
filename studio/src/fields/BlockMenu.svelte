<script lang="ts">
	import type { Component } from '../api';

	let {
		types,
		onpick,
	}: { types: Component[]; onpick: (type: string) => void } = $props();

	let open = $state(false);
	let search = $state('');
	let input = $state<HTMLInputElement>();

	const found = $derived(
		types.filter(t => t.name.toLowerCase().includes(search.toLowerCase())),
	);

	// with a single type there is nothing to choose
	function toggle() {
		if (types.length === 1) return onpick(types[0].handle);
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

<div class="menu">
	<button onclick={toggle}>
		+ Add {types.length === 1 ? types[0].name : 'block'}
	</button>
	{#if open}
		<div class="popup">
			<input
				bind:this={input}
				placeholder="Search blocks"
				bind:value={search}
				{onkeydown}
			/>
			{#each found as t (t.handle)}
				<button class="item" onclick={() => pick(t.handle)}>
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

	.popup {
		position: absolute;
		z-index: 3;
		top: 100%;
		margin-top: 0.25rem;
		width: 16rem;
		padding: 0.4rem;
		background: white;
		border: 1px solid var(--line);
		border-radius: 0.4rem;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
		display: flex;
		flex-direction: column;
		gap: 0.2rem;
	}

	.item {
		border: none;
		text-align: left;
	}
</style>
