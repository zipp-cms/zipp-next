<script lang="ts">
	import type { Snippet } from 'svelte';
	import Icon from './Icon.svelte';

	/**
	 * A panel from the right over the page, Escape or the backdrop close
	 * it. Its bar has Done, or the buttons of `actions`.
	 */
	let {
		title,
		onclose,
		actions,
		children,
	}: {
		title: string;
		onclose: () => void;
		actions?: Snippet;
		children: Snippet;
	} = $props();

	function onkeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') onclose();
	}
</script>

<svelte:window {onkeydown} />

<div class="backdrop" role="presentation" onclick={onclose}></div>
<div class="flyout" role="dialog" aria-modal="true" aria-label={title}>
	<div class="bar">
		<strong>{title}</strong>
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
		z-index: 40;
		background: rgba(16, 24, 40, 0.35);
	}

	.flyout {
		position: fixed;
		top: 0;
		right: 0;
		bottom: 0;
		z-index: 41;
		display: flex;
		flex-direction: column;
		width: min(1040px, 92vw);
		background: var(--bg);
		box-shadow: -8px 0 32px rgba(16, 24, 40, 0.18);
		animation: slide 0.18s ease-out;
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
		padding: 0 24px 24px;
		overflow-y: auto;
	}
</style>
