<script lang="ts">
	import { ui } from './texts.svelte';
	/**
	 * Where a page is: links up to it, the page itself last. In a narrow
	 * page only the link one up stays, both shortened.
	 */
	let { items }: { items: { label: string; href?: string }[] } = $props();
</script>

<nav class="crumbs" aria-label={ui.texts.breadcrumbs}>
	{#each items as item, i (i)}
		<!-- far: left out in a narrow page, with the / after it -->
		{@const far = i < items.length - 2}
		{#if i}<span class="sep" class:far={i < items.length - 1}>/</span>{/if}
		{#if item.href && i < items.length - 1}
			<a href={item.href} class:far>{item.label}</a>
		{:else}
			<span class="here truncate">{item.label}</span>
		{/if}
	{/each}
</nav>

<style lang="scss">
	@use '../vars' as *;

	.crumbs {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		min-width: 0;
		font-size: var(--fs-md);
	}

	a {
		text-decoration: none;
		white-space: nowrap;
	}

	.sep {
		color: var(--subtle);
	}

	.here {
		color: var(--text);
		font-weight: 500;
	}

	@mixin short {
		.far {
			display: none;
		}

		// shorter first, the page's name stays as long as it can
		a {
			flex-shrink: 4;
			min-width: 2.5rem;
			overflow: hidden;
			text-overflow: ellipsis;
		}
	}

	@include container-max(36rem, page) {
		@include short;
	}

	// in a flyout the bar is the flyout's
	@include max-tablet {
		@include short;
	}
</style>
