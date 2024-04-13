<script lang="ts">
	import type { ComponentField, BlockOf, ComponentContext } from '../../editor.ts';
	import Block from '../../block.svelte';
	export let block: BlockOf<ComponentField>;
	export let context: ComponentContext;
	import Button from '../../../../registry/button/button.svelte';

	const { blocks } = context;
</script>

<!-- {block.settings?.min ?? ''} -->

<div>
	{block.name}
	<!-- style:background={`rgb(${Math.random() * 255} ${Math.random() * 255} ${Math.random() * 255} / .2)`} -->
	{#each block.content as subBlock}
		<Block block={$blocks.get(subBlock)} {context} />
	{/each}
</div>

<div class="flex gap-2">
	{#if block.content.length < (block.settings?.max ?? Infinity)}
		{#each block.subComponentOptions as option}
			<Button on:click={context.choose(block, option)}>
				{option}
			</Button>
		{/each}
	{/if}
</div>
