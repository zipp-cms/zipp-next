<script lang="ts">
	import { componentContext, type Component } from './editor.ts';
	import Block from './block.svelte';
	import ContextMenu from './contextMenu/contextMenu.svelte';

	export let components: Component[];
	export let root = 'page';

	const context = componentContext(components, root);
	const { blocks, rootBlock } = context;
</script>

<main class="mx-auto max-w-4xl px-8">
	<Block block={$blocks.get($rootBlock)} {context}></Block>
</main>

<ContextMenu {context}></ContextMenu>
