<script lang="ts">
	/** Links in a line, with commas; one without `href` is text. */
	let {
		links,
		empty = '',
	}: {
		links: { text: string; href?: string }[];
		/** what shows without any */
		empty?: string;
	} = $props();
</script>

{#each links as l, i (i)}
	{i ? ', ' : ''}
	{#if l.href}
		<a href={l.href}>{l.text}</a>
	{:else}
		{l.text}
	{/if}
{:else}
	{empty}
{/each}
