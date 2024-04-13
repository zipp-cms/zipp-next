<script lang="ts">
	import type { Block, ComponentContext } from './editor.ts';

	export let block: Block;
	export let context: ComponentContext;

	// with glob import:
	const blockTypes = import.meta.glob('./blocks/*.svelte', { eager: true });

	function blockComponent(kind: string) {
		return blockTypes[`./blocks/${kind}.svelte`]?.default;
	}
</script>

<div class="relative m-4 rounded border border-blue-400 p-4">
	<pre
		class="absolute right-0 top-0 rounded bg-yellow-100 text-yellow-800">name={block.name}/kind={block.kind}/belongsto={block.belongsTo}]</pre>
	{#if block.kind === 'component'}
		<h3 class="text-xl font-bold">{block.name}</h3>
	{:else}
		{block.name}
	{/if}

	<svelte:component this={blockComponent(block.kind)} {block} {context} />
</div>
