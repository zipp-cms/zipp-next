<script lang="ts">
	import type { Schema } from '../core/schema';
	import { time } from '../core/format';
	import { dateState, type Dates } from './api';
	import SiteBadges from './SiteBadges.svelte';
	import { t } from '../i18n/index.svelte';

	/**
	 * The badges of an entry in lists: its sites with `published`, Draft
	 * when it has unpublished changes, Pending or Expired with `dates`.
	 */
	let {
		schema,
		published,
		draft,
		dates,
	}: {
		schema: Schema;
		/** published per site handle */
		published?: Record<string, boolean>;
		draft: boolean;
		dates?: Dates;
	} = $props();

	const state = $derived(dates ? dateState(dates) : null);
</script>

{#if published}
	<SiteBadges {schema} {published} />
{/if}
{#if draft}
	<span class="badge changes" title={t('entries.unpublishedChanges')}>
		{t('common.draft')}
	</span>
{/if}
{#if state === 'pending'}
	<span
		class="badge draft"
		title={t('entries.liveFrom', { time: time(dates!.postDate!) })}
	>
		{t('entries.pending')}
	</span>
{:else if state === 'expired'}
	<span
		class="badge draft"
		title={t('entries.expiredAt', { time: time(dates!.expiryDate!) })}
	>
		{t('entries.expired')}
	</span>
{/if}
