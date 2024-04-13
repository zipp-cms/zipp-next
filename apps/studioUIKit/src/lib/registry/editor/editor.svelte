<script lang="ts">
	import { componentContext, type Component } from './editor.ts';
	import Block from './block.svelte';

	export let components: Component[];
	export let root = 'page';

	const context = componentContext(components, root);
	const { blocks, rootBlocks } = context;
</script>

{#each $rootBlocks.map((b) => $blocks.get(b)) as block}
	<Block {block} {context}></Block>
{/each}
