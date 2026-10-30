<script lang="ts">
	// the Lucide icons of ui/icons, colored by `color`
	const icons = import.meta.glob('./icons/*.svg', {
		query: '?raw',
		import: 'default',
		eager: true,
	}) as Record<string, string>;

	/** `size` in px at the browser's default font size, it grows with it */
	let { name, size = 16 }: { name: string; size?: number } = $props();
</script>

<span class="icon" style:width="{size / 16}rem" style:height="{size / 16}rem">
	{@html icons[`./icons/${name}.svg`]}
</span>

<style lang="scss">
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
