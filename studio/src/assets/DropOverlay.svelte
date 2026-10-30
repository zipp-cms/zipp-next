<script lang="ts">
	import Icon from 'zipp-ui/Icon.svelte';

	/** Over the files while files from the computer are dragged in. */
	let { label }: { label: string } = $props();
</script>

<div class="drop">
	<Icon name="add" size={22} />
	{label}
</div>

<style lang="scss">
	.drop {
		position: absolute;
		inset: 0;
		z-index: var(--z-raised);
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		border: 2px dashed var(--accent);
		border-radius: var(--radius-lg);
		background: color-mix(in srgb, var(--accent-soft) 85%, transparent);
		color: var(--accent);
		font-weight: 600;
		pointer-events: none;
	}
</style>
