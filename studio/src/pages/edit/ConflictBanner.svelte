<script lang="ts">
	import type { Draft } from '../../api';
	import { time } from '../../lib/format';

	/**
	 * Someone else changed the draft: load theirs or keep mine. Without a
	 * conflict, who changed the draft last.
	 */
	let {
		conflict,
		draft,
		onloadtheirs,
		onkeepmine,
	}: {
		conflict: string;
		draft: Draft | null;
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
		<button class="dark" onclick={onloadtheirs}>Load their changes</button>
		<button class="light" onclick={onkeepmine}>Keep mine</button>
	</div>
{:else if draft?.user}
	<p class="last-change">
		Draft last changed by {draft.user}, {time(draft.updatedAt)}
	</p>
{/if}

<style lang="scss">
	.conflict {
		display: flex;
		align-items: center;
		gap: 12px;
		margin-bottom: 16px;
		padding: 12px 16px;
		background: #ffcd70;

		span {
			margin-right: auto;
		}
	}

	.last-change {
		margin: -10px 0 12px;
		font-size: 13px;
		color: var(--muted);
	}
</style>
