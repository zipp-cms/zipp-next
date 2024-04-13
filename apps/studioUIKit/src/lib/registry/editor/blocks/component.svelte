<script lang="ts">
	import type { ComponentField, BlockOf, ComponentContext, Component } from '../editor.ts';
	import Block from '../block.svelte';
	export let block: BlockOf<Component>;
	export let context: ComponentContext;

	const { blocks } = context;
</script>

<div>
	{block.name}
	{#each block.content as subBlock}
		<Block block={$blocks.get(subBlock)} {context} />
	{/each}
</div>
