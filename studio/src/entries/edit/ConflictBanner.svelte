<script lang="ts">
	/**
	 * Someone else changed the draft: load theirs or keep mine.
	 */
	let {
		conflict,
		onloadtheirs,
		onkeepmine,
	}: {
		conflict: string;
		onloadtheirs: () => void;
		onkeepmine: () => void;
	} = $props();
</script>

{#if conflict}
	<div class="conflict">
		<span>
			<b>{conflict}</b>
			while you were editing. Your last changes are not saved yet.
		</span>
		<button class="primary" onclick={onloadtheirs}>
			Load their changes
		</button>
		<button onclick={onkeepmine}>Keep mine</button>
	</div>
{/if}

<style lang="scss">
	.conflict {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 1rem;
		padding: 0.625rem 0.875rem;
		color: var(--warn);
		background: var(--warn-soft);
		border: 1px solid var(--warn-line);
		border-radius: var(--radius-lg);

		span {
			margin-right: auto;
		}
	}
</style>
