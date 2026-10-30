<script lang="ts">
	import type { Snippet } from 'svelte';
	import { layer } from './layers';

	/**
	 * A box under the button that opens it: menus, pickers, the block
	 * menu. A click outside or Escape closes it, its first input gets the
	 * focus. `trigger` renders the button with what toggles it.
	 */
	let {
		open = $bindable(false),
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
	} = $props();

	let root = $state<HTMLElement>();
	let box = $state<HTMLElement>();

	const toggle = () => (open = !open);
	const close = () => (open = false);

	function onpointerdown(e: PointerEvent) {
		if (open && !root?.contains(e.target as Node)) close();
	}

	$effect(() => {
		if (open) box?.querySelector<HTMLElement>('input')?.focus();
	});
</script>

<svelte:window {onpointerdown} />

<div class="popover" bind:this={root}>
	{@render trigger({ toggle, open })}
	{#if open}
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
