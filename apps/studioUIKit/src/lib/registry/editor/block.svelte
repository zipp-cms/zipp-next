<script lang="ts">
	import type { Block, ComponentContext } from './editor.ts';

	export let block: Block;
	export let context: ComponentContext;

	// with glob import:
	const blockTypes = import.meta.glob('./blocks/fields/*.svelte', { eager: true });
	import ComponentBlock from './blocks/component.svelte';

	function blockComponent(block: Block) {
		if (block.type === 'field') {
			return blockTypes[`./blocks/fields/${block.kind}.svelte`]?.default;
		}

		return ComponentBlock;
	}
</script>

<div class="relative p-2">
	<button class="absolute right-full size-6 rounded hover:bg-gray-200"> :: </button>

	<svelte:component this={blockComponent(block)} {block} {context} />
</div>
