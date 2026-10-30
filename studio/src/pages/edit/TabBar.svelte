<script lang="ts">
	import type { Tab } from '../../api';

	/**
	 * The tabs of an entry type, like Craft's: a dot on a tab with changes,
	 * the number of its errors in red, so nothing hides in another tab.
	 */
	let {
		tabs,
		current = $bindable(),
		changed = new Set(),
		errors = {},
	}: {
		tabs: Tab[];
		current: string;
		changed?: Set<string>;
		errors?: Record<string, string>;
	} = $props();

	// a path like `body.0.text` belongs to the tab of `body`
	const count = (tab: Tab, paths: Iterable<string>) =>
		[...paths].filter(p => tab.fields.includes(p.split('.')[0])).length;
</script>

<div class="tabs" role="tablist">
	{#each tabs as tab (tab.name)}
		{@const errs = count(tab, Object.keys(errors))}
		<button
			type="button"
			role="tab"
			aria-selected={current === tab.name}
			class:active={current === tab.name}
			onclick={() => (current = tab.name)}
		>
			{#if count(tab, changed)}<span
					class="dot"
					title="Changed"
				></span>{/if}
			{tab.name}
			{#if errs}
				<span class="errors" title="{errs} errors">{errs}</span>
			{/if}
		</button>
	{/each}
</div>

<style lang="scss">
	.tabs {
		display: inline-flex;
		gap: 2px;
		padding: 3px;
		background: var(--hover);
		border: 1px solid var(--line);
		border-radius: var(--radius-lg);

		button {
			gap: 6px;
			padding: 5px 12px;
			border: none;
			border-radius: var(--radius);
			background: none;
			color: var(--muted);

			&.active {
				background: var(--surface);
				color: var(--text);
				box-shadow: 0 1px 2px rgba(16, 24, 40, 0.1);
			}

			&:hover:not(.active) {
				color: var(--text);
				background: none;
			}
		}
	}

	.dot {
		margin: 0;
	}

	.errors {
		min-width: 18px;
		padding: 0 5px;
		border-radius: 999px;
		background: var(--danger);
		color: white;
		font-size: var(--fs-xs);
		font-weight: 600;
		line-height: 17px;
		text-align: center;
	}
</style>
