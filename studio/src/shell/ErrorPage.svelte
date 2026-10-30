<script lang="ts">
	import PageTop from '../ui/PageTop.svelte';

	/** A page that couldn't load: its error, and a way back. */
	let { loadError }: { loadError: unknown } = $props();
	const message = $derived(
		loadError instanceof Error ? loadError.message : String(loadError),
	);
</script>

<div class="page">
	<PageTop title="This page couldn't load" />
	<p class="error">{message}</p>
	<p class="empty"><a href="/admin">To the start</a></p>
</div>

<style>
	.page {
		max-width: 640px;
		margin: 0 auto;
	}
</style>
