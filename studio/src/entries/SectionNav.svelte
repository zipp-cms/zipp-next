<script lang="ts">
	import Icon from 'zipp-ui/Icon.svelte';
	import type { NavPart } from './picker';
	import { t } from '../i18n/index.svelte';

	/**
	 * The sections of the entry picker as the nav groups them, with how
	 * many each has to pick; on a phone a row above the list. A click
	 * also ends a search, in the section shown too.
	 */
	let {
		nav,
		current,
		onselect,
	}: {
		nav: NavPart[];
		/** the section shown, null while searching */
		current: string | null;
		onselect: (handle: string) => void;
	} = $props();
</script>

<nav class="sections" aria-label={t('entries.sections')}>
	{#each nav as part, i (i)}
		{#if part.name && nav.length > 1}
			<div class="head">{part.name}</div>
		{/if}
		{#each part.groups as g (g.section.handle)}
			{@const active = current === g.section.handle}
			<button
				type="button"
				class="section"
				class:active
				aria-current={active ? 'true' : undefined}
				onclick={() => onselect(g.section.handle)}
			>
				<Icon name={g.section.kind} size={15} />
				<span class="truncate">{g.section.name}</span>
				<span class="n">{g.total.pickable}</span>
			</button>
		{/each}
	{/each}
</nav>

<style lang="scss">
	// like the folders of the asset library
	.sections {
		position: sticky;
		top: 1rem;
		display: flex;
		flex-direction: column;
		gap: 0.125rem;

		// a phone has no room beside the list
		@include max-tablet {
			position: static;
			flex-flow: row wrap;
		}
	}

	.head {
		margin: 0.5rem 0 0.25rem;
		padding-left: 0.5rem;
		color: var(--muted);
		font-size: var(--fs-xs);
		font-weight: 600;
		letter-spacing: 0.04em;
		text-transform: uppercase;

		&:first-child {
			margin-top: 0;
		}

		@include max-tablet {
			display: none;
		}
	}

	.section {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		min-height: 2rem;
		padding: 0.375rem 0.5rem;
		border: none;
		border-radius: var(--radius);
		background: none;
		color: var(--text-2);
		font-weight: 500;
		text-align: left;

		@include hover {
			&:hover {
				color: var(--text);
				background: var(--hover);
			}
		}

		&.active {
			color: var(--accent);
			background: var(--accent-soft);
		}

		.n {
			margin-left: auto;
			color: var(--muted);
			font-size: var(--fs-xs);
			font-weight: 400;
		}
	}
</style>
