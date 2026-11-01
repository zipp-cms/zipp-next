<script lang="ts">
	import type { Snippet } from 'svelte';

	/**
	 * The card of fields that belong together, like a group field: its
	 * rows are `FieldRow`s (or the studio's fields inside a block).
	 */
	let { children }: { children: Snippet } = $props();
</script>

<div class="field-group box">
	{@render children()}
</div>

<style lang="scss">
	@use '../vars' as *;

	.field-group {
		container-type: inline-size;
		display: grid;
		gap: 0.625rem;
		padding: 0.875rem;
	}
</style>
