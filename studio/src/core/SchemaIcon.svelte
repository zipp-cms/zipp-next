<script lang="ts">
	import { untrack } from 'svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import type { SchemaIcon } from './schema';
	import { knownIcon, lucide } from './lucide';

	/**
	 * The icon the code gave an entry type, a block or a nav group: a
	 * Lucide icon or its own svg; without one, or an unknown name,
	 * `fallback` of the studio's icons (none: nothing).
	 */
	let {
		icon,
		fallback,
		size = 16,
	}: { icon?: SchemaIcon; fallback?: string; size?: number } = $props();

	/** the svg right away: its own, or one the schema brought */
	const now = (i?: SchemaIcon) =>
		!i ? '' : 'svg' in i ? i.svg : (knownIcon(i.lucide) ?? '');
	let svg = $state(untrack(() => now(icon)));

	$effect(() => {
		svg = now(icon);
		if (svg || !icon || 'svg' in icon) return;
		// not in the schema: fetched (dev, or a name it doesn't know)
		const name = icon.lucide;
		lucide(name).then(s => {
			if (icon && 'lucide' in icon && icon.lucide === name) svg = s ?? '';
		});
	});
</script>

{#if svg}
	<span
		class="icon schema-icon"
		style:width="{size / 16}rem"
		style:height="{size / 16}rem"
	>
		<!-- the code's own svg or Lucide's, both from the schema -->
		{@html svg}
	</span>
{:else if fallback}
	<Icon name={fallback} {size} />
{:else if icon}
	<!-- loading: its room, so nothing moves -->
	<span class="icon" style:width="{size / 16}rem"></span>
{/if}

<style lang="scss">
	.icon {
		display: inline-flex;
		flex-shrink: 0;
	}

	.schema-icon :global(svg) {
		width: 100%;
		height: 100%;
		stroke-width: 1.75;
	}
</style>
