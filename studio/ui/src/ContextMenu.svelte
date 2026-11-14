<script lang="ts">
	import type { MenuItem } from './Menu.svelte';
	import { ui } from './texts.svelte';
	import { pointerOf, takeContextMenu } from './contextMenu';
	import MenuItems from './MenuItems.svelte';
	import Popover from './Popover.svelte';

	/**
	 * A menu at the pointer without a ⋯ button: `open(e)` in any
	 * `oncontextmenu` shows `items` there. The items may change with what
	 * was clicked, set them before `open`. A text field keeps the
	 * browser's menu (paste, spelling); the menu key opens it under the
	 * element.
	 */
	let { items, label: given }: { items: MenuItem[]; label?: string } =
		$props();
	const label = $derived(given ?? ui.texts.actions);

	let shown = $state(false);
	let at = $state<{ x: number; y: number } | null>(null);

	export function open(e: MouseEvent) {
		if (!items.length || !takeContextMenu(e)) return;

		const r = (e.target as Element).getBoundingClientRect();
		at = pointerOf(e) ?? { x: r.left, y: Math.min(r.bottom, innerHeight) };
		shown = true;
	}
</script>

{#if items.length}
	<Popover bind:open={shown} bind:at {label} menu>
		<MenuItems {items} close={() => (shown = false)} />
	</Popover>
{/if}
