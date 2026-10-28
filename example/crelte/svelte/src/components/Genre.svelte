<script>
	// the option of the button group field, in its color
	let { genre } = $props();
</script>

{#if genre}
	<span class="genre" style:background={genre.color}>{genre.label}</span>
{/if}

<style>
	.genre {
		padding: 0.1rem 0.5rem;
		border-radius: 0.2rem;
		color: white;
		font-size: 0.8rem;
	}
</style>
