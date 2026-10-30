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

	/** Room kept to the edges of the window, and to the button. */
	const EDGE = 8;
	const GAP = 4;

	/**
	 * Where the box goes, fixed in the window so no scrolling container
	 * around it cuts it or gets a scrollbar: at the point of a right
	 * click, else under the button as `align` says, above it when there is
	 * no room below; moved in where it would leave the window. Again when
	 * the page scrolls or the window changes.
	 */
	function place(node: HTMLElement, p: { x: number; y: number } | null) {
		let point = p;
		const set = () => {
			const r = node.getBoundingClientRect();
			let x: number, y: number;
			if (point) {
				x = point.x;
				y =
					point.y + r.height > innerHeight - EDGE
						? point.y - r.height
						: point.y;
			} else {
				const b = root!.getBoundingClientRect();
				x =
					align === 'end'
						? b.right - r.width
						: align === 'center'
							? b.left + b.width / 2 - r.width / 2
							: b.left;
				const below = b.bottom + GAP;
				const above = b.top - GAP - r.height;
				y =
					below + r.height > innerHeight - EDGE && above >= EDGE
						? above
						: below;
			}
			x = Math.min(x, innerWidth - r.width - EDGE);
			node.style.left = `${Math.max(x, EDGE)}px`;
			node.style.top = `${Math.max(y, EDGE)}px`;
		};
		set();
		addEventListener('scroll', set, true);
		addEventListener('resize', set);
		return {
			// another right click while it is open
			update(next: { x: number; y: number } | null) {
				point = next;
				set();
			},
			destroy() {
				removeEventListener('scroll', set, true);
				removeEventListener('resize', set);
			},
		};
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
			class="popup"
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

	// fixed in the window, see `place`; over the flyouts too
	.popup {
		position: fixed;
		z-index: calc(var(--z-overlay) + 1);
		display: flex;
		flex-direction: column;
		gap: 2px;
		max-width: calc(100vw - 1rem);
	}
</style>
