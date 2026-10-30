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

	/** The ⋯ button of a row with a few actions, like Craft's. */
	let { items, label = 'Actions' }: { items: MenuItem[]; label?: string } =
		$props();

	let open = $state(false);
	let root = $state<HTMLElement>();

	function onpointerdown(e: PointerEvent) {
		if (open && !root?.contains(e.target as Node)) open = false;
	}

	function onkeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') open = false;
	}
</script>

<svelte:window {onpointerdown} {onkeydown} />

{#if items.length}
	<div class="menu" bind:this={root}>
		<button
			type="button"
			class="icon"
			title={label}
			aria-label={label}
			aria-expanded={open}
			onclick={() => (open = !open)}
		>
			<Icon name="more" size={16} />
		</button>
		{#if open}
			<div class="popup" role="menu">
				{#each items as item (item.label)}
					{#if item.href}
						<a
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
							role="menuitem"
							class:danger={item.danger}
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
			</div>
		{/if}
	</div>
{/if}

<style lang="scss">
	.menu {
		position: relative;
	}

	.icon {
		border-color: transparent;
		background: none;
		color: var(--muted);

		&:hover,
		&[aria-expanded='true'] {
			color: var(--text);
			background: var(--surface-2);
		}
	}

	.popup {
		position: absolute;
		top: calc(100% + 4px);
		right: 0;
		z-index: 20;
		display: flex;
		flex-direction: column;
		min-width: 11rem;
		padding: 4px;
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: 8px;
		box-shadow: 0 8px 24px rgba(16, 24, 40, 0.14);

		a,
		button {
			display: flex;
			align-items: center;
			gap: 8px;
			width: 100%;
			padding: 6px 10px;
			border: none;
			border-radius: 5px;
			background: none;
			color: var(--text);
			font-size: 13px;
			font-weight: 500;
			text-align: left;
			text-decoration: none;

			&:hover {
				background: var(--surface-2);
			}
		}

		.danger {
			color: var(--danger);
		}
	}
</style>
