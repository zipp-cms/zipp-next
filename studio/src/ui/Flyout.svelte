<script lang="ts">
	import type { Snippet } from 'svelte';
	import Icon from './Icon.svelte';
	import { layer } from './layers';

	/**
	 * The panel from the right over the page, for an entry, an asset or
	 * the asset library. Escape or the backdrop close it; its bar has Done,
	 * or the buttons of `actions`.
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

	let panel = $state<HTMLElement>();
	$effect(() => panel?.focus());
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
>
	<div class="bar">
		<strong class="truncate">{title}</strong>
		{#if actions}
			<div class="actions">{@render actions()}</div>
		{:else}
			<button type="button" class="primary" onclick={onclose}>
				<Icon name="close" size={14} />
				Done
			</button>
		{/if}
	</div>
	<div class="body">{@render children()}</div>
</div>

<style>
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
		width: min(1040px, 92vw);
		background: var(--bg);
		box-shadow: var(--shadow-flyout);
		outline: none;
		animation: slide 0.18s ease-out;
	}

	.narrow {
		width: min(760px, 92vw);
	}

	@keyframes slide {
		from {
			transform: translateX(40px);
			opacity: 0;
		}
	}

	.bar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 12px;
		min-height: 48px;
		padding: 8px 16px;
		background: var(--surface);
		border-bottom: 1px solid var(--line);
	}

	.actions {
		display: flex;
		gap: 8px;
	}

	/* the page inside scrolls, its top bar sticks to this */
	.body {
		flex: 1;
		min-height: 0;
		padding: 0 var(--page-pad) var(--page-pad);
		overflow-y: auto;
	}
</style>
