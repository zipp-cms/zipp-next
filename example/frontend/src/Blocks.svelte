<script lang="ts">
	import { url, type Block } from './api';
	import Events from './Events.svelte';

	let { blocks }: { blocks: Block[] } = $props();
</script>

{#each blocks as block (block.id)}
	{#if block.typeHandle === 'heading'}
		<h2>{block.text}</h2>
	{:else if block.typeHandle === 'text'}
		<p>{block.content}</p>
	{:else if block.typeHandle === 'button'}
		<a class="button" href={block.link[0] ? url(block.link[0]) : '#'}>
			{block.label}
		</a>
	{:else if block.typeHandle === 'events'}
		<Events city={block.city} />
	{/if}
{/each}

<style>
	.button {
		display: inline-block;
		padding: 0.5rem 1rem;
		background: #222;
		color: white;
		text-decoration: none;
		border-radius: 0.3rem;
		margin: 0 0.5rem 0.5rem 0;
	}
</style>
