<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { Asset } from './api';
	import Thumb from './Thumb.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import Menu, { type MenuItem } from 'zipp-ui/Menu.svelte';

	/**
	 * A file as a square card: the picture, below it a bar with the title
	 * and the ⋯ menu. A click on the picture or the title opens it; `handle` goes on
	 * the top left (a field's order), `caption` replaces the title (a
	 * rename). The Assets page, the picker and the asset field have it.
	 */
	let {
		asset,
		onopen,
		menu,
		selected = false,
		draggable = false,
		ondragstart,
		handle,
		caption,
		openTitle,
	}: {
		asset: Asset;
		onopen: () => void;
		menu?: MenuItem[];
		selected?: boolean;
		draggable?: boolean;
		ondragstart?: (e: DragEvent) => void;
		handle?: Snippet;
		caption?: Snippet;
		/** the picture's tooltip, the filename without */
		openTitle?: string;
	} = $props();
</script>

<div
	class="item"
	data-menu-area
	class:selected
	class:typed={asset.kind !== 'image' && !handle}
>
	<button
		type="button"
		class="tile plain"
		title={openTitle ?? asset.filename}
		onclick={onopen}
		draggable={draggable ? 'true' : undefined}
		{ondragstart}
	>
		<Thumb {asset} named={false} />
	</button>
	<div class="bar">
		{#if caption}
			{@render caption()}
		{:else}
			<!-- the picture is the stop of the keyboard, the title only for the mouse -->
			<button
				type="button"
				class="caption plain truncate"
				title={asset.filename}
				tabindex="-1"
				onclick={onopen}
			>
				{asset.title || asset.filename}
			</button>
		{/if}
		{#if menu?.length}
			<span class="menu"><Menu items={menu} label="File actions" /></span>
		{/if}
	</div>
	{#if handle}<span class="handle">{@render handle()}</span>{/if}
	{#if selected}
		<span class="check"><Icon name="save" size={14} /></span>
	{/if}
</div>

<style lang="scss">
	.item {
		position: relative;
		display: flex;
		flex-direction: column;
		min-width: 0;
		aspect-ratio: 1;
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
		background: var(--surface);

		@include hover {
			&:hover {
				border-color: var(--border-strong);
			}
		}

		&.selected {
			@include focus-ring(1px);
		}
	}

	// the picture takes what the bar leaves
	.tile {
		display: flex;
		flex: 1;
		align-items: stretch;
		min-height: 0;
		width: 100%;
		padding: 0;
		// the card's corners, its ⋯ menu may reach out of it
		overflow: hidden;
		border-radius: calc(var(--radius-lg) - 1px) calc(var(--radius-lg) - 1px)
			0 0;

		:global(.thumb) {
			flex: 1;
			height: auto;
			aspect-ratio: auto;
			border: none;
			border-radius: 0;
		}
	}

	// an image's type is in its name, a file shows it (but not under a handle)
	.item:not(.typed) :global(.ext) {
		display: none;
	}

	.bar {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		min-height: 2.25rem;
		padding: 0 0.25rem 0 0.625rem;
		border-top: 1px solid var(--border);
	}

	.caption {
		display: block;
		flex: 1;
		min-width: 0;
		align-self: stretch;
		padding: 0;
		font-size: var(--fs-sm);
		font-weight: 500;
		text-align: left;

		@include hover {
			&:hover {
				color: var(--accent);
			}
		}
	}

	.menu {
		margin-left: auto;

		:global(button.icon) {
			width: 1.75rem;
			height: 1.75rem;
		}
	}

	.handle {
		position: absolute;
		top: 0.375rem;
		left: 0.375rem;
		display: grid;
		place-items: center;
		width: 1.75rem;
		height: 1.75rem;
		border-radius: var(--radius);
		background: var(--surface);
		box-shadow: var(--shadow);
	}

	.check {
		position: absolute;
		top: 0.375rem;
		right: 0.375rem;
		display: grid;
		place-items: center;
		width: 1.375rem;
		height: 1.375rem;
		border-radius: 50%;
		background: var(--accent);
		color: var(--on-accent);
	}
</style>
