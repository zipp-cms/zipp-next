<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { Link, Schema } from '../../core/schema';
	import type { Refs } from '../../entries/api';
	import Icon from 'zipp-ui/Icon.svelte';
	import Menu, { type MenuItem } from 'zipp-ui/Menu.svelte';
	import { chosenAssets } from '../../assets/chosen.svelte';
	import { LINK_ICONS, linkSummary, opensInNewTab } from './link';
	import { t } from '../../i18n/index.svelte';

	/**
	 * A link as a row: the icon of its kind, title, where it goes, whether
	 * it opens in a new tab; a click on it edits it (`onedit`), the ⋯ menu
	 * has the rest. The link field, a navigation, the links and buttons of
	 * a rich text. The click area covers the box around the row: the
	 * nearest positioned parent (`position: relative`).
	 */
	let {
		schema,
		refs,
		link,
		externalInNewTab = true,
		handle,
		onedit,
		editing = false,
		menu = [],
	}: {
		schema: Schema;
		refs: Refs;
		link: Link;
		/** external links open in a new tab, unless it's set */
		externalInNewTab?: boolean;
		/** the DragHandle, on the left */
		handle?: Snippet;
		/** a click on the link edits it */
		onedit?: () => void;
		/** its settings are open */
		editing?: boolean;
		menu?: MenuItem[];
	} = $props();

	const newTab = $derived(opensInNewTab(schema, link, externalInNewTab));
	// a linked file shows its filename
	const asset = chosenAssets(() =>
		link.type === 'asset' && typeof link.value === 'number'
			? [link.value]
			: [],
	);
	const summary = $derived(
		linkSummary(
			schema,
			link,
			refs,
			link.type === 'asset' && typeof link.value === 'number'
				? asset.get(link.value)?.filename
				: undefined,
		),
	);
</script>

{#snippet content()}
	<span class="kind" class:entry={link.type === 'entry'}>
		<Icon name={LINK_ICONS[link.type]} size={16} />
	</span>
	<span class="text">
		<strong class="truncate">{summary.title}</strong>
		{#if summary.sub}
			<span class="sub truncate" class:mono={summary.mono}>
				{summary.sub}
			</span>
		{/if}
	</span>
{/snippet}

<!-- the handle comes after the button so it paints above its click area,
	`order` puts it back on the left -->
<div class="link-row" data-menu-area>
	{#if onedit}
		<button
			type="button"
			class="main plain"
			title={editing ? t('fields.closeSettings') : t('fields.editLink')}
			aria-expanded={editing}
			onclick={onedit}
		>
			{@render content()}
		</button>
	{:else}
		<span class="main">{@render content()}</span>
	{/if}
	{#if newTab}
		<span class="tab">
			<Icon name="external" size={12} />
			{t('fields.newTab')}
		</span>
	{/if}
	{#if handle}
		<span class="handle">{@render handle()}</span>
	{/if}
	<Menu items={menu} label={t('fields.linkActions')} />
</div>

<style lang="scss">
	.link-row {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		min-width: 0;
	}

	// the kind of the link, an entry in the accent
	.kind {
		display: grid;
		place-items: center;
		width: 1.25rem;
		flex-shrink: 0;
		color: var(--muted);

		&.entry {
			color: var(--accent);
		}
	}

	// the link itself, a click edits it
	.main {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		flex: 1;
		min-width: 0;
		text-align: left;
	}

	// its click area covers the whole box, the handle and the menu (both
	// positioned, after it) stay above it
	button.main {
		&::after {
			content: '';
			position: absolute;
			inset: 0;
			border-radius: var(--radius-lg);
		}

		&:focus-visible {
			outline: none;

			&::after {
				@include focus-ring(-2px);
			}
		}
	}

	@include hover {
		button.main:hover strong {
			color: var(--accent);
		}
	}

	.handle {
		display: flex;
		order: -1;
	}

	.text {
		display: flex;
		flex-direction: column;
		flex: 1;
		min-width: 0;
		line-height: 1.3;
	}

	strong {
		font-weight: 600;
	}

	.sub {
		color: var(--muted);
		font-size: var(--fs-md);

		&.mono {
			font-family: var(--mono);
			font-size: var(--fs-sm);
		}
	}

	.tab {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		flex-shrink: 0;
		padding: 0.125rem 0.375rem;
		border: 1px solid var(--line);
		border-radius: var(--radius-sm);
		background: var(--surface-2);
		color: var(--muted);
		font-size: var(--fs-sm);
	}
</style>
