<script lang="ts">
	import type { MenuItem } from './Menu.svelte';
	import Icon from './Icon.svelte';

	/** The lines of `Menu` and `ContextMenu`; a pick closes the menu. */
	let { items, close }: { items: MenuItem[]; close: () => void } = $props();
</script>

{#each items as item, i (i)}
	{#if item.href}
		<a
			class="popup-item"
			href={item.href}
			target={item.href.startsWith('/admin') || item.download
				? undefined
				: '_blank'}
			download={item.download}
			role="menuitem"
			onclick={close}
		>
			<Icon name={item.icon} size={14} />
			{item.label}
		</a>
	{:else}
		<button
			type="button"
			class="popup-item"
			class:danger={item.danger}
			role="menuitem"
			onclick={() => {
				close();
				item.onclick?.();
			}}
		>
			<Icon name={item.icon} size={14} />
			{item.label}
		</button>
	{/if}
{/each}

<style lang="scss">
	// as wide as the longest item, a label stays on one line
	.popup-item {
		min-width: 8.625rem;
		white-space: nowrap;
	}
</style>
