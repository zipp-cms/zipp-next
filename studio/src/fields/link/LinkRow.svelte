<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { Link } from '../../core/schema';
	import Icon from 'zipp-ui/Icon.svelte';
	import { LINK_ICONS } from './link';

	/**
	 * A link as a row: the icon of its kind, title, where it goes, whether
	 * it opens in a new tab, and the actions on the right. The link field
	 * and the items of a navigation.
	 */
	let {
		link,
		summary,
		newTab,
		handle,
		actions,
	}: {
		link: Link;
		summary: { title: string; sub: string; mono: boolean };
		newTab: boolean;
		/** the DragHandle, on the left */
		handle?: Snippet;
		actions: Snippet;
	} = $props();
</script>

<div class="link-row">
	{@render handle?.()}
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
	{#if newTab}
		<span class="tab">
			<Icon name="external" size={12} />
			New tab
		</span>
	{/if}
	<span class="actions">{@render actions()}</span>
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

	.actions {
		display: flex;
		align-items: center;
		gap: 0.125rem;
		flex-shrink: 0;
	}
</style>
