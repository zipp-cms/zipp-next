<script lang="ts">
	import Icon from 'zipp-ui/Icon.svelte';
	import { fileSize } from '../core/format';
	import { t } from '../i18n/index.svelte';
	import { uploads } from './uploads.svelte';

	/** The running uploads at the bottom right: a bar, the failed ones. */
	const percent = $derived(
		uploads.total ? Math.round((uploads.sent / uploads.total) * 100) : 0,
	);
</script>

{#if uploads.files || uploads.errors.length}
	<div class="progress card" role="status" aria-live="polite">
		<div class="head">
			<strong>
				{#if uploads.active}
					{t('assets.uploadingOf', {
						n:
							uploads.done + 1 > uploads.files
								? uploads.files
								: uploads.done + 1,
						total: uploads.files,
					})}
				{:else}
					{t('assets.uploadedFiles', {
						n: uploads.files - uploads.errors.length,
					})}
				{/if}
			</strong>
			<span class="note">
				{t('assets.sentOf', {
					sent: fileSize(uploads.sent),
					total: fileSize(uploads.total),
				})}
			</span>
			{#if !uploads.active}
				<button
					type="button"
					class="icon"
					aria-label={t('common.close')}
					onclick={() => uploads.close()}
				>
					<Icon name="close" size={14} />
				</button>
			{/if}
		</div>
		<div
			class="bar"
			role="progressbar"
			aria-valuenow={percent}
			aria-valuemin={0}
			aria-valuemax={100}
		>
			<span style:width="{percent}%"></span>
		</div>
		{#if uploads.errors.length}
			<ul class="plain errors">
				{#each uploads.errors as e, i (i)}
					<li class="error">{e}</li>
				{/each}
			</ul>
		{/if}
	</div>
{/if}

<style lang="scss">
	.progress {
		position: fixed;
		right: 1.5rem;
		bottom: 1.5rem;
		// over the flyouts, the picker uploads too
		z-index: calc(var(--z-overlay) + 1);
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		width: 20rem;
		box-shadow: var(--shadow-pop);
	}

	.head {
		display: flex;
		align-items: center;
		gap: 0.5rem;

		strong {
			font-size: var(--fs-sm);
		}

		.note {
			margin-left: auto;
		}
	}

	.bar {
		overflow: hidden;
		height: 0.375rem;
		border-radius: 1rem;
		background: var(--surface-2);

		span {
			display: block;
			height: 100%;
			border-radius: inherit;
			background: var(--accent);
			transition: width 0.2s;
		}
	}

	.errors {
		overflow: auto;
		max-height: 8rem;
		font-size: var(--fs-sm);
	}
</style>
