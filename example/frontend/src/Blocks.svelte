<script lang="ts">
	import { url, type Block } from './api';
	import Events from './Events.svelte';
	import Img from './Img.svelte';

	let { blocks }: { blocks: Block[] } = $props();
</script>

{#each blocks as block (block.id)}
	{#if block.typeHandle === 'heading'}
		<h2>{block.text}</h2>
	{:else if block.typeHandle === 'text'}
		<div class="rt">{@html block.content?.html ?? ''}</div>
	{:else if block.typeHandle === 'image'}
		<figure>
			{#each block.image as img (img.url)}
				<Img {img} sizes="40rem" />
			{/each}
			{#if block.caption}<figcaption>{block.caption}</figcaption>{/if}
		</figure>
	{:else if block.typeHandle === 'button' && block.link}
		<a class="button" href={block.link.url} target={block.link.target}>
			{block.link.label ?? block.link.defaultLabel}
		</a>
	{:else if block.typeHandle === 'events'}
		<Events city={block.city} />
	{/if}
{/each}

<style>
	figure {
		margin: 1.5rem 0;
	}

	figcaption {
		color: #666;
		font-size: 0.9rem;
	}

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
