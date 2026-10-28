<script>
	// a link like Craft's LinkData, external ones open in a new tab
	let { link } = $props();
</script>

{#if link}
	<a class="button" href={link.url} target={link.target}>
		{link.label ?? link.defaultLabel}
	</a>
{/if}

<style>
	.button {
		display: inline-block;
		margin: 0 0.5rem 0.5rem 0;
		padding: 0.5rem 1rem;
		border-radius: 0.3rem;
		background: #222;
		color: white;
		text-decoration: none;
	}
</style>
