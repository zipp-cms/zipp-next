<script lang="ts">
	// the icons of the old Zipp (Material icons), colored by `color`
	const icons = import.meta.glob('../icons/*.svg', {
		query: '?raw',
		import: 'default',
		eager: true,
	}) as Record<string, string>;

	let { name }: { name: string } = $props();
</script>

<span class="icon">{@html icons[`../icons/${name}.svg`]}</span>

<style>
	.icon {
		display: inline-flex;
		width: 16px;
		height: 16px;
		flex-shrink: 0;
	}

	.icon :global(svg) {
		width: 100%;
		height: 100%;
	}
</style>
