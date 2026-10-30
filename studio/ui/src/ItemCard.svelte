<script lang="ts">
	import type { Snippet } from 'svelte';
	import Icon from './Icon.svelte';

	/**
	 * A card of a list in a field: a block, a block in a rich text, an
	 * item of a navigation. Its head has the kind, a summary when it is
	 * closed and the actions when the pointer is on it; the head opens it.
	 */
	let {
		kind,
		kindIcon,
		summary,
		open = $bindable(true),
		collapsible = true,
		invalid = false,
		changed = false,
		handle,
		actions,
		children,
		id,
	}: {
		kind: string;
		/** before the kind, e.g. the icon of a block type */
		kindIcon?: Snippet;
		summary?: Snippet;
		open?: boolean;
		collapsible?: boolean;
		invalid?: boolean;
		/** new or changed in the draft */
		changed?: boolean;
		/** the DragHandle, at the start of the head */
		handle?: Snippet;
		actions?: Snippet;
		children: Snippet;
		id?: string;
	} = $props();

	const shown = $derived(open || !collapsible);
	const toggle = () => collapsible && (open = !open);
</script>

<div class="item-card box" class:open={shown} class:invalid {id}>
	<div class="head">
		{@render handle?.()}
		<button
			type="button"
			class="toggle plain"
			aria-expanded={shown}
			disabled={!collapsible}
			onclick={toggle}
		>
			{#if changed}<span class="changed-dot" title="Changed"></span>{/if}
			<span class="kind">{@render kindIcon?.()}{kind}</span>
			{#if summary}
				<span class="summary truncate">{@render summary()}</span>
			{/if}
			{#if invalid}
				<span class="flag" title="Has errors">
					<Icon name="alert" size={14} />
				</span>
			{/if}
		</button>
		{#if actions}<span class="reveal">{@render actions()}</span>{/if}
		{#if collapsible}
			<button
				type="button"
				class="icon"
				title={shown ? 'Collapse' : 'Expand'}
				onclick={toggle}
			>
				<Icon name={shown ? 'chevron-down' : 'chevron-right'} />
			</button>
		{/if}
	</div>
	{#if shown}
		<div class="body">{@render children()}</div>
	{/if}
</div>

<style lang="scss">
	.item-card {
		scroll-margin-top: 4rem;

		&.invalid {
			border-color: var(--danger);
		}
	}

	.head {
		display: flex;
		align-items: center;
		gap: 2px;
		min-height: 2.375rem;
		padding: 0 0.375rem 0 0.25rem;
	}

	.open > .head {
		border-bottom: 1px solid var(--line);
	}

	.toggle {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		flex: 1;
		min-width: 0;
		padding: 0.375rem;

		&:disabled {
			opacity: 1;
			cursor: default;
		}
	}

	.kind {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		flex-shrink: 0;
		padding: 1px 0.4375rem;
		border-radius: var(--radius-sm);
		background: var(--accent-soft);
		color: var(--accent);
		font-size: var(--fs-xs);
		font-weight: 600;
	}

	.summary {
		color: var(--text-2);
		font-weight: 400;
	}

	.flag {
		display: inline-flex;
		color: var(--danger);
	}

	.body {
		padding: 0.875rem 0.875rem 1rem;
	}
</style>
