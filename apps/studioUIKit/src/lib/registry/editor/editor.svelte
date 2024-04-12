<script lang="ts">
	import { componentContext, type Component } from './editor.ts';

	// with glob import:
	const blockTypes = import.meta.glob('./blocks/*.svelte', { eager: true });

	console.log(blockTypes);

	function blockComponent(kind: string) {
		return blockTypes[`./blocks/${kind}.svelte`]?.default;
	}

	export let components: Component[];
	export let root = 'page';

	const { blocks, ...context } = componentContext(components, root);
</script>

{#each $blocks as block}
	<div style:padding-left="{block.level * 20}px">
		{block.name}
		<svelte:component this={blockComponent(block.kind)} />

		<div>
			{#each block.subComponentOptions as option}
				<button
					on:click={context.choose(block, option)}
					class="rounded border-2 border-slate-500 p-2"
				>
					{option}
				</button>
			{/each}
		</div>
	</div>
{/each}
