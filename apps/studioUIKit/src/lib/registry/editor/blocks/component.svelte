<script lang="ts">
	import Button from '../../button/button.svelte';
	import type { ComponentField, BlockOf, ComponentContext, Component } from '../editor.ts';
	import Block from '../block.svelte';
	import { ChevronDown, ChevronsDownUp, ChevronsUpDown } from 'lucide-svelte';
	import { slide } from 'svelte/transition';
	export let block: BlockOf<Component>;
	export let context: ComponentContext;

	const { blocks } = context;
	let open = true;
</script>

<div>
	<header class="flex items-center gap-2">
		<Button variant="ghost" on:click={() => (open = !open)}>
			<span class="font-semibold underline">
				{block.name}
			</span>
			{#if open}
				<ChevronsUpDown strokeWidth={1} />
			{:else}
				<ChevronsDownUp strokeWidth={1} />
			{/if}
		</Button>
	</header>
	{#if open}
		<div class="pl-2" transition:slide>
			{#each block.content as subBlock}
				<Block block={$blocks.get(subBlock)} {context} />
			{/each}
		</div>
	{/if}
</div>
