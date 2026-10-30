<script lang="ts">
	import type { Sortable } from './sortable.svelte';

	/**
	 * Where a dragged row would go: a line in the container of the rows,
	 * which has `position: relative`.
	 */
	let { sortable }: { sortable: Sortable } = $props();
</script>

{#if sortable.line}
	<div
		class="sort-line"
		style:top="{sortable.line.top}px"
		style:left="{sortable.line.left}px"
		style:width="{sortable.line.width}px"
		style:height="{sortable.line.height}px"
	></div>
{/if}

<style>
	.sort-line {
		position: absolute;
		z-index: var(--z-raised);
		border-radius: 2px;
		background: var(--accent);
		pointer-events: none;
	}
</style>
