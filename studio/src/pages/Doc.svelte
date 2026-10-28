<script lang="ts">
	import type { Schema } from '../api';
	import { shown } from '../route.svelte';

	/** A markdown file of the code, read only. */
	let { doc }: { doc: Schema['docs'][number] } = $props();

	$effect(() => {
		shown.section = doc.handle;
	});

	// links out of the studio open in a new tab, many sites can't be framed
	function external(node: HTMLElement) {
		$effect(() => {
			void doc.html;
			for (const a of node.querySelectorAll('a')) {
				if (a.origin !== location.origin) {
					a.target = '_blank';
					a.rel = 'noreferrer';
				}
			}
		});
	}
</script>

<div class="card markdown-body" use:external>
	{@html doc.html}
</div>

<style>
	.card {
		max-width: 900px;
		padding: 24px 32px;
	}
</style>
