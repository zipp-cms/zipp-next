<script lang="ts">
	/** Where a page is: links up to it, the page itself last. */
	let { items }: { items: { label: string; href?: string }[] } = $props();
</script>

<nav class="crumbs" aria-label="Breadcrumbs">
	{#each items as item, i (i)}
		{#if i}<span class="sep">/</span>{/if}
		{#if item.href && i < items.length - 1}
			<a href={item.href}>{item.label}</a>
		{:else}
			<span class="here">{item.label}</span>
		{/if}
	{/each}
</nav>

<style>
	.crumbs {
		display: flex;
		align-items: center;
		gap: 6px;
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
		overflow: hidden;
		color: var(--text);
		font-weight: 500;
		text-overflow: ellipsis;
		white-space: nowrap;
	}
</style>
