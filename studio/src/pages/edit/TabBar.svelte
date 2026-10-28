<script lang="ts">
	import type { Tab } from '../../api';

	/**
	 * The tabs of an entry type, like Craft's: a dot on a tab with changes,
	 * red with errors, so nothing hides in another tab.
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
	const has = (tab: Tab, paths: Iterable<string>) =>
		[...paths].some(p => tab.fields.includes(p.split('.')[0]));
</script>

<div class="tabs" role="tablist">
	{#each tabs as tab (tab.name)}
		<button
			type="button"
			role="tab"
			aria-selected={current === tab.name}
			class:active={current === tab.name}
			class:error={has(tab, Object.keys(errors))}
			onclick={() => (current = tab.name)}
		>
			{#if has(tab, changed)}<span class="dot"></span>{/if}{tab.name}
		</button>
	{/each}
</div>

<style lang="scss">
	.tabs {
		display: flex;
		gap: 24px;
		margin: -14px -16px 24px;
		padding: 0 16px;
		border-bottom: 2px solid var(--line);

		button {
			padding: 12px 0;
			font-size: 16px;
			border: none;
			border-bottom: 2px solid transparent;
			border-radius: 0;
			background: none;
			margin-bottom: -2px;

			&.active {
				border-bottom-color: var(--dark);
			}

			&.error {
				color: var(--danger);
			}
		}
	}
</style>
