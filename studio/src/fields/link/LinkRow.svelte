<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { Link } from '../../core/schema';
	import Icon from 'zipp-ui/Icon.svelte';
	import Menu, { type MenuItem } from 'zipp-ui/Menu.svelte';
	import { LINK_ICONS } from './link';

	/**
	 * A link as a row: the icon of its kind, title, where it goes, whether
	 * it opens in a new tab; a click on it edits it (`onedit`), the ⋯ menu
	 * has the rest. The link field, a navigation, the links and buttons of
	 * a rich text.
	 */
	let {
		link,
		summary,
		newTab,
		handle,
		onedit,
		editing = false,
		menu = [],
	}: {
		link: Link;
		summary: { title: string; sub: string; mono: boolean };
		newTab: boolean;
		/** the DragHandle, on the left */
		handle?: Snippet;
		/** a click on the link edits it */
		onedit?: () => void;
		/** its settings are open */
		editing?: boolean;
		menu?: MenuItem[];
	} = $props();
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

<div class="link-row" data-menu-area>
	{@render handle?.()}
	{#if onedit}
		<button
			type="button"
			class="main plain"
			title={editing ? 'Close its settings' : 'Edit the link'}
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
			New tab
		</span>
	{/if}
	<Menu items={menu} label="Actions for the link" />
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
		width: 2rem;
		height: 2rem;
		flex-shrink: 0;
		border: 1px solid var(--line);
		border-radius: var(--radius);
		background: var(--surface-2);
		color: var(--muted);

		&.entry {
			border-color: transparent;
			background: var(--accent-soft);
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

	@include hover {
		button.main:hover strong {
			color: var(--accent);
		}
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
