<script>
	import Img from '@/components/Img.svelte';

	// rich text in chunks: html, rows of buttons and embedded images; zipp
	// removes empty paragraphs, the spacing is * + * without margin hacks
	let { text } = $props();
</script>

<div class="rt">
	{#each text?.chunks ?? [] as chunk, i (i)}
		{#if chunk.html}
			<div class="html">{@html chunk.html}</div>
		{:else if chunk.buttons}
			<div class="buttons">
				{#each chunk.buttons as b (b.url)}
					<a class="button" href={b.url} target={b.target}>
						{b.label ?? b.defaultLabel}
					</a>
				{/each}
			</div>
		{:else if chunk.images}
			<figure>
				{#each chunk.images as img (img.url)}<Img {img} />{/each}
				{#if chunk.caption}<figcaption>{chunk.caption}</figcaption>{/if}
			</figure>
		{/if}
	{/each}
</div>

<style>
	.rt > :global(* + *),
	.html > :global(* + *) {
		margin-top: 1em;
	}

	.buttons {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.button {
		padding: 0.5rem 1rem;
		border-radius: 0.3rem;
		background: #222;
		color: white;
		text-decoration: none;
	}

	figcaption {
		margin-top: 0.4rem;
		color: #666;
		font-size: 0.9rem;
	}
</style>
