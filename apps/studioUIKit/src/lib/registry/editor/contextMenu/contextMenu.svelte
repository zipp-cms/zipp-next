<script lang="ts">
	import { writable } from 'svelte/store';
	import type { ComponentContext, Block } from '../editor.ts';

	export let context: ComponentContext;

	let state = writable({
		isOpen: false,
		left: 0,
		top: 0,
		x: 0,
		y: 0,
		block: null
	});

	function contextMenuPosition(target: HTMLElement) {
		const rect = target.getBoundingClientRect();

		return {
			x: rect.left + rect.width,
			y: rect.top
		};
	}

	$: context.ui.onContextMenuOpen((block: Block, event: MouseEvent) => {
		const position = contextMenuPosition(event.currentTarget as HTMLElement);

		state.set({
			isOpen: true,
			left: position.x,
			top: position.y,
			x: '-100%',
			y: 0,
			block
		});
	});

	function close() {
		state.update((s) => ({ ...s, isOpen: false }));
	}
</script>

<svelte:window on:keydown={(e) => e.key === 'Escape' && close()} />

{#if $state.isOpen}
	<div
		style:left="{$state.left}px"
		style:top="{$state.top}px"
		style:transform={`translate(${$state.x},${$state.y})`}
		class="bg-background fixed left-0 top-0 rounded-md border shadow-md"
	>
		<ul>
			<li>{$state.block.type}</li>
			<li>{$state.block.kind}</li>
		</ul>

		<pre class="font-mono text-sm text-gray-500">{JSON.stringify($state.block, null, 2)}</pre>
	</div>
{/if}
