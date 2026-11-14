<script lang="ts" module>
	export type MenuItem = {
		label: string;
		icon: string;
		/** a link, or what a click does */
		href?: string;
		/** the link downloads, with this filename */
		download?: string;
		onclick?: () => void;
		danger?: boolean;
	};
</script>

<script lang="ts">
	import { ui } from './texts.svelte';
	import Icon from './Icon.svelte';
	import MenuItems from './MenuItems.svelte';
	import Popover from './Popover.svelte';

	/**
	 * The ⋯ button of a row with a few actions, like Craft's. A right
	 * click on the row or card around it (`data-menu-area`) opens the same
	 * menu at the pointer; the innermost area's menu wins.
	 */
	let { items, label: given }: { items: MenuItem[]; label?: string } =
		$props();
	const label = $derived(given ?? ui.texts.actions);

	let open = $state(false);
	let at = $state<{ x: number; y: number } | null>(null);
	let button = $state<HTMLElement>();

	$effect(() => {
		const area = button?.closest<HTMLElement>('[data-menu-area]');
		if (!area) return;
		const oncontextmenu = (e: MouseEvent) => {
			// another menu of the same area took it, the first one wins
			if (e.defaultPrevented) return;
			// a text field keeps the browser's menu (paste, spelling)
			if (
				(e.target as Element).closest(
					'input, textarea, [contenteditable]',
				)
			)
				return;
			e.preventDefault();
			e.stopPropagation();
			// the menu key has no pointer, the menu opens under the ⋯
			at =
				e.button === 2 || e.clientX
					? { x: e.clientX, y: e.clientY }
					: null;
			open = true;
		};
		area.addEventListener('contextmenu', oncontextmenu);
		return () => area.removeEventListener('contextmenu', oncontextmenu);
	});
</script>

{#if items.length}
	<Popover bind:open bind:at align="end" {label} menu>
		{#snippet trigger({ toggle, open })}
			<button
				type="button"
				class="icon"
				title={label}
				aria-label={label}
				aria-expanded={open}
				onclick={toggle}
				bind:this={button}
			>
				<Icon name="more" />
			</button>
		{/snippet}
		<MenuItems {items} close={() => (open = false)} />
	</Popover>
{/if}
