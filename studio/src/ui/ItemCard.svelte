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
		summary,
		open = $bindable(true),
		collapsible = true,
		invalid = false,
		changed = false,
		actions,
		children,
		id,
	}: {
		kind: string;
		summary?: Snippet;
		open?: boolean;
		collapsible?: boolean;
		invalid?: boolean;
		/** new or changed in the draft */
		changed?: boolean;
		actions?: Snippet;
		children: Snippet;
		id?: string;
	} = $props();

	const shown = $derived(open || !collapsible);
	const toggle = () => collapsible && (open = !open);
</script>

<div class="item-card box" class:open={shown} class:invalid {id}>
	<div class="head">
		<button
			type="button"
			class="toggle plain"
			aria-expanded={shown}
			disabled={!collapsible}
			onclick={toggle}
		>
			{#if changed}<span class="dot" title="Changed"></span>{/if}
			<span class="kind">{kind}</span>
			{#if summary && !shown}
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
		scroll-margin-top: 64px;

		&.invalid {
			border-color: var(--danger);
		}
	}

	.head {
		display: flex;
		align-items: center;
		gap: 2px;
		min-height: 38px;
		padding: 0 6px 0 4px;
	}

	.open > .head {
		border-bottom: 1px solid var(--line);
	}

	.toggle {
		display: flex;
		align-items: center;
		gap: 8px;
		flex: 1;
		min-width: 0;
		padding: 6px;

		&:disabled {
			opacity: 1;
			cursor: default;
		}
	}

	.kind {
		flex-shrink: 0;
		padding: 1px 7px;
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
		padding: 14px 14px 16px;
	}
</style>
