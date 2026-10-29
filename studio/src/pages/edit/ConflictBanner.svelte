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
		gap: 12px;
		margin-bottom: 16px;
		padding: 10px 14px;
		color: var(--warn);
		background: var(--warn-soft);
		border: 1px solid #f1d58b;
		border-radius: 8px;

		span {
			margin-right: auto;
		}
	}
</style>
