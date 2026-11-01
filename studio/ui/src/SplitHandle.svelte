<script lang="ts">
	import { ui } from './texts.svelte';
	import { untrack } from 'svelte';

	/**
	 * The border between two panes, dragged or moved with the arrow keys
	 * (a focusable separator is ARIA's window splitter); a double click
	 * gives the default. `width` is the pane's that it borders: `side`
	 * `start` is a pane on the left (the handle on its right border),
	 * `end` one on the right; it sits in the pane, which is `position:
	 * relative`. The width is kept in this browser under `key`.
	 * `resizing` is on while it is dragged, a frame next to it may not
	 * take the pointer.
	 */
	let {
		width = $bindable(),
		resizing = $bindable(false),
		key,
		initial,
		min,
		maxShare = 0.6,
		side = 'start',
		label,
	}: {
		width?: number;
		resizing?: boolean;
		key: string;
		initial: number;
		min: number;
		/** of the window's width */
		maxShare?: number;
		side?: 'start' | 'end';
		label: string;
	} = $props();

	const max = () => Math.max(min, Math.round(innerWidth * maxShare));
	const clamp = (w: number) => Math.round(Math.min(Math.max(w, min), max()));

	width = untrack(() => clamp(Number(localStorage.getItem(key)) || initial));

	// dragging to the right grows a pane on the left
	const dir = $derived(side === 'start' ? 1 : -1);

	function set(w: number) {
		width = clamp(w);
		localStorage.setItem(key, `${width}`);
	}

	function onpointerdown(e: PointerEvent) {
		if (e.button !== 0) return;
		e.preventDefault();
		const el = e.currentTarget as HTMLElement;
		el.setPointerCapture(e.pointerId);
		const from = e.clientX - dir * width!;
		resizing = true;
		const move = (ev: PointerEvent) => set(dir * (ev.clientX - from));
		const end = () => {
			resizing = false;
			el.removeEventListener('pointermove', move);
			el.removeEventListener('pointerup', end);
			el.removeEventListener('pointercancel', end);
		};
		el.addEventListener('pointermove', move);
		el.addEventListener('pointerup', end);
		el.addEventListener('pointercancel', end);
	}

	function onkeydown(e: KeyboardEvent) {
		const step = (e.shiftKey ? 80 : 20) * dir;
		if (e.key === 'ArrowLeft') set(width! - step);
		else if (e.key === 'ArrowRight') set(width! + step);
		else return;
		e.preventDefault();
	}
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex, a11y_no_noninteractive_element_interactions -->
<div
	class="split-handle {side}"
	class:resizing
	role="separator"
	aria-orientation="vertical"
	aria-label={label}
	aria-valuenow={width}
	aria-valuemin={min}
	aria-valuemax={max()}
	tabindex="0"
	title={ui.texts.resize}
	{onpointerdown}
	ondblclick={() => set(initial)}
	{onkeydown}
></div>

<style lang="scss">
	@use '../vars' as *;

	// over the border, a line in the accent on hover
	.split-handle {
		position: absolute;
		top: 0;
		bottom: 0;
		z-index: var(--z-raised);
		width: 7px;
		cursor: col-resize;
		touch-action: none;

		&.start {
			right: -4px;
		}

		&.end {
			left: -4px;
		}

		&::after {
			content: '';
			position: absolute;
			top: 0;
			bottom: 0;
			left: 3px;
			width: 1px;
			transition: background 0.15s;
		}

		@include hover {
			&:hover::after {
				left: 2px;
				width: 3px;
				background: var(--accent);
			}
		}

		&:focus-visible::after,
		&.resizing::after {
			left: 2px;
			width: 3px;
			background: var(--accent);
		}

		&:focus-visible {
			outline: none;
		}
	}
</style>
