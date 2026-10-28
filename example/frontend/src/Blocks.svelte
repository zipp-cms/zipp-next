<script lang="ts">
	import { url, type Refs } from './api';
	import type { PageFields } from './content';
	import Events from './Events.svelte';

	let {
		blocks,
		refs,
	}: { blocks: NonNullable<PageFields['body']>; refs: Refs } = $props();
</script>

{#each blocks as block (block.id)}
	{#if block.type === 'heading'}
		<h2>{block.fields.text}</h2>
	{:else if block.type === 'text'}
		<p>{block.fields.content}</p>
	{:else if block.type === 'button'}
		{@const target = refs[block.fields.link?.[0] ?? -1]}
		<a class="button" href={target ? url(target) : '#'}>
			{block.fields.label}
		</a>
	{:else if block.type === 'events'}
		<Events city={block.fields.city} />
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
	}
</style>
