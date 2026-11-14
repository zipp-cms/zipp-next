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
	import { on } from 'svelte/events';
	import { ui } from './texts.svelte';
	import { pointerOf, takeContextMenu } from './contextMenu';
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
		// through svelte's `on`, so a `ContextMenu` inside the area comes first
		return on(area, 'contextmenu', e => {
			if (!takeContextMenu(e)) return;
			// the menu key has no pointer, the menu opens under the ⋯
			at = pointerOf(e);
			open = true;
		});
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
