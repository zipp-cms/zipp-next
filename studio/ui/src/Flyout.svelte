<script lang="ts">
	import type { Snippet } from 'svelte';
	import Icon from './Icon.svelte';
	import { layer } from './layers';
	import { provideFlyoutBar, type FlyoutBar } from './flyoutBar';

	/**
	 * The panel from the right over the page, for an entry, an asset or
	 * the asset library. Escape or the backdrop close it; its bar has Done,
	 * or the buttons of `actions`. A page in it (its PageTop) puts its
	 * crumbs and actions into the bar, with × to close.
	 */
	let {
		title,
		onclose,
		actions,
		children,
		narrow = false,
	}: {
		title: string;
		onclose: () => void;
		actions?: Snippet;
		children: Snippet;
		/** for a list to choose from, not a whole page */
		narrow?: boolean;
	} = $props();

	const bar: FlyoutBar = $state({ content: null });
	provideFlyoutBar(bar);

	let panel = $state<HTMLElement>();
	$effect(() => panel?.focus());

	const FOCUSABLE =
		'a[href], button:not(:disabled), input:not(:disabled), ' +
		'select:not(:disabled), textarea:not(:disabled), ' +
		'[tabindex]:not([tabindex="-1"]), [contenteditable="true"]';

	/** Tab stays in the flyout, it is modal: from the last to the first. */
	function onkeydown(e: KeyboardEvent) {
		if (e.key !== 'Tab' || !panel) return;
		const all = [...panel.querySelectorAll<HTMLElement>(FOCUSABLE)].filter(
			el => el.offsetParent !== null,
		);
		if (!all.length) return;
		const [first, last] = [all[0], all[all.length - 1]];
		const at = document.activeElement;
		if (e.shiftKey && (at === first || at === panel)) {
			e.preventDefault();
			last.focus();
		} else if (!e.shiftKey && at === last) {
			e.preventDefault();
			first.focus();
		}
	}
</script>

<div class="backdrop" role="presentation" onclick={onclose}></div>
<div
	class="flyout"
	class:narrow
	role="dialog"
	aria-modal="true"
	aria-label={title}
	tabindex="-1"
	bind:this={panel}
	use:layer={onclose}
	{onkeydown}
>
	<div class="bar">
		{#if bar.content}
			<div class="page">{@render bar.content()}</div>
			<button
				type="button"
				class="icon"
				title="Close"
				aria-label="Close"
				onclick={onclose}
			>
				<Icon name="close" size={16} />
			</button>
		{:else}
			<strong class="truncate">{title}</strong>
			{#if actions}
				<div class="actions">{@render actions()}</div>
			{:else}
				<button type="button" class="primary" onclick={onclose}>
					<Icon name="close" size={14} />
					Done
				</button>
			{/if}
		{/if}
	</div>
	<div class="body" class:page-in={!!bar.content}>{@render children()}</div>
</div>

<style lang="scss">
	.backdrop {
		position: fixed;
		inset: 0;
		z-index: var(--z-overlay);
		background: var(--backdrop);
	}

	.flyout {
		position: fixed;
		top: 0;
		right: 0;
		bottom: 0;
		z-index: var(--z-overlay);
		display: flex;
		flex-direction: column;
		width: min(65rem, 92vw);
		background: var(--bg);
		box-shadow: var(--shadow-flyout);
		outline: none;
		animation: slide 0.18s ease-out;
	}

	.narrow {
		width: min(47.5rem, 92vw);
	}

	@keyframes slide {
		from {
			transform: translateX(2.5rem);
			opacity: 0;
		}
	}

	.bar {
		position: relative;
		z-index: var(--z-bar);
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.75rem;
		min-height: 3rem;
		padding: 0.5rem 1rem;
		background: var(--surface);
		border-bottom: 1px solid var(--line);
	}

	.actions {
		display: flex;
		gap: 0.5rem;
	}

	/* the page inside scrolls, its top bar sticks to this */
	.body {
		flex: 1;
		min-height: 0;
		padding: 0 var(--page-pad) var(--page-pad);
		overflow-y: auto;
	}

	// a page's PageTop is in the bar: the space under it here
	.page-in {
		padding-top: 1.25rem;
	}

	// the PageTop of the page in it
	.page {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		flex: 1;
		flex-wrap: wrap;
		min-width: 0;
	}
</style>
