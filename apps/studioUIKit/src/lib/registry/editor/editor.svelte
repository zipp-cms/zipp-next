<script lang="ts">
	import { componentContext, type Component } from './editor.ts';
	import Block from './block.svelte';
	import ContextMenu from './contextMenu/contextMenu.svelte';
	import BlockMenu from './blockMenu/blockMenu.svelte';
	import clickOutside from '$lib/utils.ts';

	export let components: Component[];
	export let root = 'page';

	const context = componentContext(components, root);
	const { blocks, rootBlock } = context;
</script>

<main
	class="mx-auto flex min-h-[100vh] max-w-4xl flex-col px-8"
	use:clickOutside
	on:clickoutside={() => context.ui.selection.activeBlock.set(null)}
>
	<Block block={$blocks.get($rootBlock)} {context}></Block>
</main>

<ContextMenu {context} />
