<script lang="ts">
	import PageTop from 'zipp-ui/PageTop.svelte';
</script>

<main class="page">
	<PageTop title="Not found" bleed={false} />
	<p class="empty">
		This page doesn't exist. <a href="/admin">To the start</a>
	</p>
</main>

<style lang="scss">
	.page {
		max-width: 40rem;
		margin: 0 auto;
	}
</style>
