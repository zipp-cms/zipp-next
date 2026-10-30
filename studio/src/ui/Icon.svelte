<script lang="ts">
	// the Lucide icons of src/icons, colored by `color`
	const icons = import.meta.glob('../icons/*.svg', {
		query: '?raw',
		import: 'default',
		eager: true,
	}) as Record<string, string>;

	let { name, size = 16 }: { name: string; size?: number } = $props();
</script>

<span class="icon" style:width="{size}px" style:height="{size}px">
	{@html icons[`../icons/${name}.svg`]}
</span>

<style>
	.icon {
		display: inline-flex;
		flex-shrink: 0;
	}

	.icon :global(svg) {
		width: 100%;
		height: 100%;
		stroke-width: 1.75;
	}
</style>
