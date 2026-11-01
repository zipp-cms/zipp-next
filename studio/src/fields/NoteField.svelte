<script lang="ts">
	import type { FieldOf } from '../core/schema';
	import { markdown } from '../core/markdown';

	/** Text for editors from the code, it has no value to edit. */
	let { field }: { field: FieldOf<'note'> } = $props();
</script>

<!-- from the code, markdown without html or images -->
<div class="note-text">{@html markdown(field.text)}</div>

<style lang="scss">
	.note-text {
		padding: 0.625rem 0.875rem;
		background: var(--surface-2);
		border: 1px solid var(--line);
		border-radius: var(--radius);
		color: var(--muted);
		font-size: var(--fs-sm);
		line-height: 1.5;

		> :global(:first-child) {
			margin-top: 0;
		}

		> :global(:last-child) {
			margin-bottom: 0;
		}

		:global(a) {
			color: inherit;
		}
	}
</style>
