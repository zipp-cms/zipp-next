<script lang="ts">
	import { siteOf, type Schema } from '../core/schema';
	import { setSite, site } from '../core/router.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import { t } from '../i18n/index.svelte';

	/**
	 * The site the studio edits, and where it runs; the box opens a
	 * choice, the icon next to it the website of that site.
	 */
	let { schema }: { schema: Schema } = $props();

	const current = $derived(siteOf(schema, site.current));
	const url = $derived(
		schema.siteUrls[site.current] ??
			schema.siteUrls[schema.sites[0]?.handle],
	);
	const offline = (handle: string) => schema.offline.includes(handle);
</script>

<div class="site-switch">
	<label class="choice" title={t('shell.site')}>
		<span class="mark">{(current?.name ?? '?')[0]}</span>
		<span class="name">
			<strong class="truncate">{current?.name}</strong>
			<small>
				{schema.dev
					? t('shell.development')
					: t('shell.production')}{offline(site.current)
					? ' · ' + t('shell.offline')
					: ''}
			</small>
		</span>
		{#if schema.sites.length > 1}
			<Icon name="chevrons-up-down" />
			<select
				value={site.current}
				onchange={e => setSite(e.currentTarget.value)}
			>
				{#each schema.sites as s (s.handle)}
					<option value={s.handle}>
						{s.name}{offline(s.handle)
							? ` (${t('shell.offline')})`
							: ''}
					</option>
				{/each}
			</select>
		{/if}
	</label>
	{#if url}
		<a
			class="open"
			href={url}
			target="_blank"
			rel="noopener"
			title={t('shell.openWebsite')}
			aria-label={t('shell.openWebsite')}
		>
			<Icon name="external" />
		</a>
	{/if}
</div>

<style lang="scss">
	.site-switch {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.5rem 0.375rem 0.5rem 0.625rem;
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
		color: var(--muted);
	}

	.choice {
		position: relative;
		display: flex;
		flex: 1;
		align-items: center;
		gap: 0.625rem;
		min-width: 0;
	}

	.open {
		display: grid;
		place-items: center;
		flex-shrink: 0;
		width: 2rem;
		height: 2rem;
		border-radius: var(--radius);
		color: var(--muted);

		@include hover {
			&:hover {
				background: var(--hover);
				color: var(--text);
			}
		}
	}

	.mark {
		display: grid;
		place-items: center;
		width: 1.75rem;
		height: 1.75rem;
		flex-shrink: 0;
		border-radius: var(--radius);
		background: var(--accent);
		color: var(--on-accent);
		font-weight: 700;
		font-size: var(--fs-md);
	}

	.name {
		display: flex;
		flex-direction: column;
		margin-right: auto;
		min-width: 0;

		strong {
			color: var(--text);
			font-size: var(--fs-md);
		}

		small {
			font-size: var(--fs-xs);
		}
	}

	// the box but the icon opens the choice
	select {
		position: absolute;
		inset: 0;
		opacity: 0;
		cursor: pointer;
	}
</style>
