<script lang="ts">
	import PageTop from '../ui/PageTop.svelte';
</script>

<main class="page">
	<PageTop title="Not found" bleed={false} />
	<p class="empty">
		This page doesn't exist. <a href="/admin">To the start</a>
	</p>
</main>

<style>
	.page {
		max-width: 640px;
		margin: 0 auto;
	}
</style>
