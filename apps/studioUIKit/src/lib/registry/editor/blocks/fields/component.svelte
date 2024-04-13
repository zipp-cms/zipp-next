<script lang="ts">
	import { get } from 'svelte/store';
	import type { ComponentField, BlockOf, ComponentContext } from '../../editor.ts';
	import Block from '../../block.svelte';
	export let block: BlockOf<ComponentField>;
	export let context: ComponentContext;
	import Button from '../../../../registry/button/button.svelte';

	const { blocks } = context;

	const isRootChild = block.parent === get(context.rootBlock);
</script>

{#if isRootChild}
	<hr class=" my-8 border-2 border-black/20" />
{/if}

<div>
	{#if isRootChild}
		<h2 class="mb-4 font-serif text-3xl">{block.name}</h2>
	{:else}
		{block.name}
	{/if}

	{#each block.content as subBlock}
		<Block block={$blocks.get(subBlock)} {context} />
	{/each}
</div>

{#if block.content.length < (block.settings?.max ?? Infinity)}
	<div class="flex max-w-fit gap-2 rounded-3xl border p-2 shadow-lg">
		{#each block.subComponentOptions as option}
			<Button on:click={context.choose(block, option)}>
				{option}
			</Button>
		{/each}
	</div>
{/if}
