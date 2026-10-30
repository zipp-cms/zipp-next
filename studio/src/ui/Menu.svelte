<script lang="ts" module>
	export type MenuItem = {
		label: string;
		icon: string;
		/** a link, or what a click does */
		href?: string;
		onclick?: () => void;
		danger?: boolean;
	};
</script>

<script lang="ts">
	import Icon from './Icon.svelte';
	import Popover from './Popover.svelte';

	/** The ⋯ button of a row with a few actions, like Craft's. */
	let { items, label = 'Actions' }: { items: MenuItem[]; label?: string } =
		$props();

	let open = $state(false);
</script>

{#if items.length}
	<Popover bind:open align="end" width="11rem" {label}>
		{#snippet trigger({ toggle, open })}
			<button
				type="button"
				class="icon"
				title={label}
				aria-label={label}
				aria-expanded={open}
				onclick={toggle}
			>
				<Icon name="more" size={16} />
			</button>
		{/snippet}
		{#each items as item (item.label)}
			{#if item.href}
				<a
					class="popup-item"
					href={item.href}
					target={item.href.startsWith('/admin')
						? undefined
						: '_blank'}
					role="menuitem"
					onclick={() => (open = false)}
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
						open = false;
						item.onclick?.();
					}}
				>
					<Icon name={item.icon} size={14} />
					{item.label}
				</button>
			{/if}
		{/each}
	</Popover>
{/if}
