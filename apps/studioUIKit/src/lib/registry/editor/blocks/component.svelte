<script lang="ts">
	import type { ComponentField, BlockOf, ComponentContext } from '../editor.ts';
	import Block from '../block.svelte';
	export let block: BlockOf<ComponentField>;
	export let context: ComponentContext;

	const { blocks } = context;
</script>

<!-- {block.settings?.min ?? ''} -->

{#if block.content.length < (block.settings?.max ?? Infinity)}
	{#each block.subComponentOptions as option}
		<button on:click={context.choose(block, option)} class="rounded border-2 border-slate-500 p-2">
			{option}
		</button>
	{/each}
{/if}

<div
	style:background={`rgb(${Math.random() * 255} ${Math.random() * 255} ${Math.random() * 255} / .2)`}
>
	{#each block.content as subBlock}
		<Block block={$blocks.get(subBlock)} {context} />
	{/each}
</div>
