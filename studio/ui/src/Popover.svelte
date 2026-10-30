<script lang="ts">
	import type { Snippet } from 'svelte';
	import { layer } from './layers';

	/**
	 * A box under the button that opens it: menus, pickers, the block
	 * menu. A click outside or Escape closes it, its first input gets the
	 * focus. `trigger` renders the button with what toggles it. With `at`
	 * it opens at that point of the window instead (a right click).
	 */
	let {
		open = $bindable(false),
		at = $bindable(null),
		align = 'start',
		width,
		trigger,
		children,
		label,
	}: {
		open?: boolean;
		/** which edge of the button the box starts at */
		align?: 'start' | 'end' | 'center';
		/** a css width, default as wide as its content */
		width?: string;
		trigger: Snippet<[{ toggle: () => void; open: boolean }]>;
		children: Snippet;
		label?: string;
		at?: { x: number; y: number } | null;
	} = $props();

	let root = $state<HTMLElement>();
	let box = $state<HTMLElement>();

	const toggle = () => (open = !open);
	const close = () => (open = false);

	// the next open is under the button again
	$effect(() => {
		if (!open) at = null;
	});

	/** At the point, moved in where it would leave the window. */
	function place(node: HTMLElement, p: { x: number; y: number }) {
		const set = (p: { x: number; y: number }) => {
			const r = node.getBoundingClientRect();
			const x = Math.min(p.x, innerWidth - r.width - 8);
			const y = p.y + r.height > innerHeight - 8 ? p.y - r.height : p.y;
			node.style.left = `${Math.max(x, 8)}px`;
			node.style.top = `${Math.max(y, 8)}px`;
		};
		set(p);
		// another right click while it is open
		return { update: set };
	}

	// a click outside closes it, listened to only while it is open
	$effect(() => {
		if (!open) return;
		const onpointerdown = (e: PointerEvent) => {
			if (!root?.contains(e.target as Node)) close();
		};
		window.addEventListener('pointerdown', onpointerdown);
		return () => window.removeEventListener('pointerdown', onpointerdown);
	});

	$effect(() => {
		if (open) box?.querySelector<HTMLElement>('input')?.focus();
	});
</script>

<div class="popover" bind:this={root}>
	{@render trigger({ toggle, open })}
	{#if open}
		{#if at}
			<div
				class="popup at"
				role="dialog"
				aria-label={label}
				style:width
				bind:this={box}
				use:layer={close}
				use:place={at}
			>
				{@render children()}
			</div>
		{:else}
			<div
				class="popup {align}"
				role="dialog"
				aria-label={label}
				style:width
				bind:this={box}
				use:layer={close}
			>
				{@render children()}
			</div>
		{/if}
	{/if}
</div>

<style lang="scss">
	.popover {
		position: relative;
	}

	.popup {
		top: calc(100% + 0.25rem);
		display: flex;
		flex-direction: column;
		gap: 2px;
		max-width: 90vw;
	}

	// at the pointer, over everything of the page
	.at {
		position: fixed;
		z-index: calc(var(--z-overlay) + 1);
	}

	.start {
		left: 0;
	}

	.end {
		right: 0;
	}

	.center {
		left: 50%;
		transform: translateX(-50%);
	}
</style>
