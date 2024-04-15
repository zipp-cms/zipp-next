<script lang="ts">
	import type { Block, ComponentContext } from './editor.ts';

	export let block: Block;
	export let context: ComponentContext;

	// with glob import:
	const blockTypes = import.meta.glob('./blocks/fields/*.svelte', { eager: true });
	import ComponentBlock from './blocks/component.svelte';
	import { GripVertical } from 'lucide-svelte';
	import { scale } from 'svelte/transition';

	function blockComponent(block: Block) {
		if (block.type === 'field') {
			return blockTypes[`./blocks/fields/${block.kind}.svelte`]?.default;
		}

		return ComponentBlock;
	}

	const activeBlock = context.ui.selection.activeBlock;

	function handleMouseEnter() {
		activeBlock.set(block.id);
	}
</script>

<div
	transition:scale={{ duration: 200, start: 0.98 }}
	class="relative ml-2 rounded-xl outline-2 outline-blue-500 data-[activeBlock]:outline"
	data-activeBlock={$activeBlock === block.id || undefined}
	on:click|stopPropagation={handleMouseEnter}
	role="group"
>
	{#if $activeBlock === block.id}
		<button
			on:click={(e) => context.ui.openContextMenu(block, e)}
			class="absolute right-full flex size-6 items-center justify-center rounded hover:bg-black/10"
		>
			<GripVertical strokeWidth={2} size={16} class="text-black/30" />
		</button>
	{/if}

	<svelte:component this={blockComponent(block)} {block} {context} />
</div>
