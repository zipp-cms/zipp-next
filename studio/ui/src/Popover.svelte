<script lang="ts">
	import type { Snippet } from 'svelte';
	import { layer } from './layers';

	/**
	 * A box under the button that opens it: menus, pickers, the block
	 * menu. A click outside or Escape closes it, its first input gets the
	 * focus. `trigger` renders the button with what toggles it. With `at`
	 * it opens at that point of the window instead (a right click). A
	 * `menu` is a list of `role="menuitem"`s: the first has the focus, the
	 * arrow keys move it.
	 */
	let {
		open = $bindable(false),
		at = $bindable(null),
		align = 'start',
		width,
		trigger,
		children,
		label,
		menu = false,
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
		menu?: boolean;
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
	function place(node: HTMLElement, p: { x: number; y: number } | null) {
		const set = (p: { x: number; y: number } | null) => {
			if (!p) return;
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
		if (!open) return;
		const first = menu ? '[role=menuitem]' : 'input';
		box?.querySelector<HTMLElement>(first)?.focus();
	});

	/** In a menu the arrow keys, Home and End move between the items. */
	function onkeydown(e: KeyboardEvent) {
		if (!menu || !box) return;
		const items = [...box.querySelectorAll<HTMLElement>('[role=menuitem]')];
		const i = items.indexOf(document.activeElement as HTMLElement);
		const to = {
			ArrowDown: (i + 1) % items.length,
			ArrowUp: (i - 1 + items.length) % items.length,
			Home: 0,
			End: items.length - 1,
		}[e.key];
		if (to === undefined || !items.length) return;
		e.preventDefault();
		items[to].focus();
	}
</script>

<div class="popover" bind:this={root}>
	{@render trigger({ toggle, open })}
	{#if open}
		<div
			class="popup {at ? 'at' : align}"
			role={menu ? 'menu' : 'dialog'}
			aria-label={label}
			tabindex="-1"
			style:width
			bind:this={box}
			use:layer={close}
			use:place={at}
			{onkeydown}
		>
			{@render children()}
		</div>
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
