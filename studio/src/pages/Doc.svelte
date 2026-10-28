<script lang="ts">
	import type { Schema } from '../api';
	import { shown } from '../route.svelte';

	/** A markdown file of the code, read only. */
	let { doc }: { doc: Schema['docs'][number] } = $props();

	$effect(() => {
		shown.section = doc.handle;
	});
</script>

<div class="card markdown-body">
	{@html doc.html}
</div>

<style>
	.card {
		max-width: 900px;
		padding: 24px 32px;
	}
</style>
