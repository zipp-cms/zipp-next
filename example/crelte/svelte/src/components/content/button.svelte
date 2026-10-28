<script>
	let { label, link } = $props();
</script>

<a class="button" href={link[0]?.url ?? '#'}>{label}</a>

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
