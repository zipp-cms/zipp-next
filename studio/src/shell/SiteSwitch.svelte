<script lang="ts">
	import { siteOf, type Schema } from '../core/schema';
	import { setSite, site } from '../core/router.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import { t } from '../i18n/index.svelte';
	import { Latest } from '../core/latest.svelte';
	import { favicon } from './favicon';

	/**
	 * The site the studio edits, and where it runs; the box opens a
	 * choice, the icon next to it the website of that site. The mark is
	 * the website's favicon, the name's first letter without one.
	 */
	let { schema }: { schema: Schema } = $props();

	const current = $derived(siteOf(schema, site.current));
	// the site of the website, the first one's when this has no url
	const urlSite = $derived(
		schema.siteUrls[site.current] ? site.current : schema.sites[0]?.handle,
	);
	const url = $derived(urlSite && schema.siteUrls[urlSite]);
	const offline = (handle: string) => schema.offline.includes(handle);

	let icon: string | null = $state(null);
	const loads = new Latest();
	$effect(() => {
		const handle = urlSite;
		icon = null;
		if (handle)
			loads.run(
				() => favicon(handle),
				i => (icon = i),
			);
	});
</script>

<div class="site-switch">
	<label
		class="choice"
		class:multi={schema.sites.length > 1}
		title={t('shell.site')}
	>
		{#if icon}
			<img
				class="mark icon"
				src={icon}
				alt=""
				onerror={() => (icon = null)}
			/>
		{:else}
			<span class="mark">{(current?.name ?? '?')[0]}</span>
		{/if}
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
		padding: 0.25rem 0.375rem 0.25rem 0.25rem;
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
		padding: 0.25rem 0.375rem;
		border-radius: var(--radius);
		transition: background-color 0.2s ease;

		// with several sites it is a choice: it shows it under the mouse
		// and with the keyboard
		&.multi {
			@include hover {
				&:hover {
					background: var(--hover);
					color: var(--text);
				}
			}

			&:has(select:focus-visible) {
				@include focus-ring;
			}
		}
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

		// the favicon on the box's background, its own colors
		&.icon {
			background: none;
			object-fit: contain;
		}
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
