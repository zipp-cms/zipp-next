<script lang="ts">
	import { get } from 'svelte/store';
	import type { ComponentField, BlockOf, ComponentContext } from '../../editor.ts';
	import Block from '../../block.svelte';
	export let block: BlockOf<ComponentField>;
	export let context: ComponentContext;
	import Button from '../../../../registry/button/button.svelte';
	import Component from '../component.svelte';
	import BlockMenu from '../../blockMenu/blockMenu.svelte';

	const { blocks } = context;

	const isRootChild = block.parent === get(context.rootBlock);
</script>

{#if isRootChild}
	<hr class=" border-1 border-foreground/30 my-8" />
{/if}

<div>
	{#if isRootChild}
		<h2 class="mb-4 font-serif text-3xl">{block.name}</h2>
	{:else}
		<span class="text-foreground/50 font-bold">
			{block.name}
		</span>
	{/if}

	{#if block.settings?.max === 1 && block.content.length === 1}
		<!-- collapse subComponent -->
		{@const subComponent = $blocks.get(block.content[0])}
		<span class="bg-foreground/10 inline-block rounded-xl px-3 py-0.5 text-opacity-30"
			>{subComponent?.name}</span
		>
		{#each subComponent?.content as field}
			<Block block={$blocks.get(field)} {context} />
		{/each}
	{:else}
		{#each block.content as subBlock}
			<Block block={$blocks.get(subBlock)} {context} />
		{/each}
	{/if}
</div>

{#if block.content.length < (block.settings?.max ?? Infinity)}
	{#if block.settings?.min > 0}
		<div class="flex max-w-fit gap-2 rounded-3xl bg-gray-100 p-2">
			{#each block.settings.component as option}
				<Button variant="outline" class="bg-white" on:click={context.choose(block, option)}>
					{option}
				</Button>
			{/each}
		</div>
	{:else}
		<BlockMenu {block} {context}></BlockMenu>
	{/if}
{/if}

<!-- fill rest of page -->
{#if isRootChild}
	<div class="h-full min-h-10 flex-grow grow"></div>
{/if}
