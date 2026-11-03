<script lang="ts">
	import type { AssetUsage } from './api';
	import { sectionName, type Schema } from '../core/schema';
	import { entryPath } from '../core/paths';
	import { closeFlyout } from '../core/flyout.svelte';
	import { mayLeave } from '../core/leave';
	import { t } from '../i18n/index.svelte';

	/** The entries that use a file, with their sites; links out of the flyout. */
	let {
		schema,
		usage,
	}: {
		schema: Schema;
		/** null while it loads */
		usage: AssetUsage[] | null;
	} = $props();
</script>

<div class="card">
	<h3>{t('common.usedIn')}</h3>
	{#if usage === null}
		<p class="empty">{t('common.loading')}</p>
	{:else}
		<ul class="plain usage">
			{#each usage as u (u.id)}
				<li>
					<a
						href={entryPath(u.section, u.id)}
						onclick={e => {
							// asks once, the page then doesn't
							if (mayLeave()) closeFlyout();
							else e.preventDefault();
						}}
					>
						{u.title}
					</a>
					<span class="note">
						{sectionName(schema, u.section)} · {u.sites
							.map(s => s.toUpperCase())
							.join(' ')}
					</span>
				</li>
			{:else}
				<li class="note">{t('assets.notUsed')}</li>
			{/each}
		</ul>
	{/if}
</div>

<style lang="scss">
	.card {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		padding: 0.75rem;
	}

	.usage {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
		font-size: var(--fs-md);

		li {
			display: flex;
			flex-direction: column;
		}
	}
</style>
