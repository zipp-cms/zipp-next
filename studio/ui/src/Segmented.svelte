<script lang="ts" generics="T extends string | null">
	import { tick, type Snippet } from 'svelte';

	/**
	 * A few choices side by side, one is on: the tabs of an entry, Write
	 * and Preview, the options of a short dropdown, the tools of the image
	 * editor. `item` renders a choice, its label by default.
	 */
	let {
		options,
		value,
		onchange,
		label,
		tabs = false,
		item,
	}: {
		options: { value: T; label: string }[];
		value: T;
		onchange: (v: T) => void;
		label?: string;
		/** tabs over content, not a value of a field */
		tabs?: boolean;
		item?: Snippet<[{ value: T; label: string }]>;
	} = $props();

	/**
	 * One stop for Tab, the chosen one; the arrow keys choose the one next
	 * to it, like radios and tabs do.
	 */
	function onkeydown(e: KeyboardEvent) {
		const by = { ArrowRight: 1, ArrowDown: 1, ArrowLeft: -1, ArrowUp: -1 }[
			e.key
		];
		if (!by) return;
		e.preventDefault();
		const i = options.findIndex(o => o.value === value);
		const next = options[(i + by + options.length) % options.length];
		onchange(next.value);
		const group = (e.currentTarget as HTMLElement).parentElement;
		tick().then(() =>
			group?.querySelector<HTMLElement>('[tabindex="0"]')?.focus(),
		);
	}
</script>

<div
	class="segmented"
	role={tabs ? 'tablist' : 'radiogroup'}
	aria-label={label}
>
	{#each options as o (o.value)}
		<button
			type="button"
			role={tabs ? 'tab' : 'radio'}
			aria-selected={tabs ? o.value === value : undefined}
			aria-checked={tabs ? undefined : o.value === value}
			class:on={o.value === value}
			tabindex={o.value === value ? 0 : -1}
			onclick={() => onchange(o.value)}
			{onkeydown}
		>
			{#if item}{@render item(o)}{:else}{o.label}{/if}
		</button>
	{/each}
</div>

<style lang="scss">
	@use '../vars' as *;

	.segmented {
		display: inline-flex;
		flex-wrap: wrap;
		gap: 2px;
		padding: 3px;
		background: var(--hover);
		border: 1px solid var(--line);
		border-radius: var(--radius-lg);

		button {
			gap: 0.375rem;
			padding: 0.3125rem 0.75rem;
			border: none;
			border-radius: var(--radius);
			background: none;
			color: var(--muted);

			&.on {
				background: var(--surface);
				color: var(--text);
				box-shadow: var(--shadow);
			}

			@include hover {
				&:hover:not(.on):not(:disabled) {
					color: var(--text);
					background: none;
				}
			}
		}
	}
</style>
