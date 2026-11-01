<script lang="ts">
	import { t } from '../../i18n/index.svelte';

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
			{t('entries.conflict')}
		</span>
		<button class="primary" onclick={onloadtheirs}>
			{t('entries.loadTheirs')}
		</button>
		<button onclick={onkeepmine}>{t('entries.keepMine')}</button>
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
