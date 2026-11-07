<script lang="ts">
	import { ui } from './texts.svelte';
	import Icon from './Icon.svelte';
	import Menu, { type MenuItem } from './Menu.svelte';

	/**
	 * What can be done with an item of a list besides moving it (that's
	 * the DragHandle): switch it off and on, duplicate it, remove it.
	 * Every list of a field has these, in this order; in a narrow place
	 * more than one go into a ⋯ menu.
	 */
	let {
		onswitch,
		off = false,
		onduplicate,
		onremove,
		removeLabel,
	}: {
		/** switches it off (the website leaves it out) and on again */
		onswitch?: () => void;
		off?: boolean;
		onduplicate?: () => void;
		onremove: () => void;
		removeLabel?: string;
	} = $props();

	const items: MenuItem[] = $derived([
		...(onswitch
			? [
					{
						label: off ? ui.texts.show : ui.texts.hide,
						icon: off ? 'view' : 'hide',
						onclick: onswitch,
					},
				]
			: []),
		...(onduplicate
			? [
					{
						label: ui.texts.duplicate,
						icon: 'copy',
						onclick: onduplicate,
					},
				]
			: []),
		{
			label: removeLabel ?? ui.texts.remove,
			icon: 'delete',
			onclick: onremove,
			danger: true,
		},
	]);
</script>

<span class="item-actions" class:many={items.length > 1}>
	{#if onswitch}
		<button
			type="button"
			class="icon"
			title={off ? ui.texts.show : ui.texts.hide}
			aria-label={off ? ui.texts.show : ui.texts.hide}
			aria-pressed={off}
			onclick={onswitch}
		>
			<Icon name={off ? 'view' : 'hide'} size={15} />
		</button>
	{/if}
	{#if onduplicate}
		<button
			type="button"
			class="icon"
			title={ui.texts.duplicate}
			aria-label={ui.texts.duplicate}
			onclick={onduplicate}
		>
			<Icon name="copy" size={15} />
		</button>
	{/if}
	<button
		type="button"
		class="icon remove"
		title={removeLabel ?? ui.texts.remove}
		aria-label={removeLabel ?? ui.texts.remove}
		onclick={onremove}
	>
		<Icon name="delete" size={15} />
	</button>
	{#if items.length > 1}<span class="menu"><Menu {items} /></span>{/if}
</span>

<style lang="scss">
	@use '../vars' as *;

	.item-actions {
		display: flex;
		flex-shrink: 0;
	}

	.menu {
		display: none;
	}

	@include container-max(30rem) {
		.many > :not(.menu) {
			display: none;
		}

		.many > .menu {
			display: contents;
		}
	}

	@include hover {
		.remove:hover:not(:disabled) {
			color: var(--danger);
		}
	}
</style>
