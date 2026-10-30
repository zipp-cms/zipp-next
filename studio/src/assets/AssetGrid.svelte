<script lang="ts">
	import type { Asset } from './api';
	import Thumb from './Thumb.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import Menu, { type MenuItem } from 'zipp-ui/Menu.svelte';
	import { ASSETS_DRAG } from './uploads.svelte';

	/**
	 * Thumbnails, files show their name. With `draggable` a tile drags
	 * onto a folder of the tree; with `menu` it has a ⋯ menu, where
	 * `rename` turns the caption into an input for `onrename`.
	 */
	let {
		assets,
		selected = [],
		loading = false,
		draggable = false,
		empty = 'No assets yet',
		menu,
		onrename,
		onpick,
	}: {
		assets: Asset[];
		loading?: boolean;
		selected?: number[];
		draggable?: boolean;
		empty?: string;
		menu?: (a: Asset, rename: () => void) => MenuItem[];
		/** whether it worked, the input stays open when not */
		onrename?: (a: Asset, filename: string) => Promise<boolean>;
		onpick: (a: Asset) => void;
	} = $props();

	function ondragstart(e: DragEvent, a: Asset) {
		e.dataTransfer!.setData(ASSETS_DRAG, JSON.stringify([a.id]));
		e.dataTransfer!.effectAllowed = 'move';
	}

	let renaming = $state<number | null>(null);
	let name = $state('');

	function rename(a: Asset) {
		renaming = a.id;
		name = a.filename;
	}

	let saving = false;
	async function save(a: Asset) {
		if (saving || renaming !== a.id) return;
		const n = name.trim();
		if (!n || n === a.filename || !onrename) {
			renaming = null;
			return;
		}
		saving = true;
		if (await onrename(a, n)) renaming = null;
		saving = false;
	}

	function onkeydown(e: KeyboardEvent, a: Asset) {
		if (e.key === 'Enter') {
			e.preventDefault();
			save(a);
		} else if (e.key === 'Escape') {
			e.preventDefault();
			renaming = null;
		}
	}

	/** The name without its extension selected, like a file manager. */
	function focus(el: HTMLInputElement) {
		el.focus();
		const dot = el.value.lastIndexOf('.');
		el.setSelectionRange(0, dot > 0 ? dot : el.value.length);
	}
</script>

<div class="grid">
	{#each assets as a (a.id)}
		<div class="item" class:selected={selected.includes(a.id)}>
			<button
				class="tile"
				onclick={() => onpick(a)}
				title={a.filename}
				draggable={draggable ? 'true' : undefined}
				ondragstart={e => draggable && ondragstart(e, a)}
			>
				<Thumb asset={a} />
				{#if renaming !== a.id}
					<span class="caption">{a.title || a.filename}</span>
				{/if}
				{#if selected.includes(a.id)}
					<span class="check"><Icon name="save" size={14} /></span>
				{/if}
			</button>
			{#if renaming === a.id}
				<input
					class="rename"
					bind:value={name}
					aria-label="Filename"
					use:focus
					onkeydown={e => onkeydown(e, a)}
					onblur={() => save(a)}
				/>
			{/if}
			{#if menu}
				<span class="menu">
					<Menu
						items={menu(a, () => rename(a))}
						label="File actions"
					/>
				</span>
			{/if}
		</div>
	{:else}
		<p class="empty">{loading ? 'Loading…' : empty}</p>
	{/each}
</div>

<style lang="scss">
	.grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(8.75rem, 1fr));
		gap: 1rem;
	}

	.item {
		position: relative;
		display: flex;
		flex-direction: column;
		min-width: 0;
	}

	.tile {
		position: relative;
		display: flex;
		flex-direction: column;
		align-items: stretch;
		gap: 0.375rem;
		padding: 0;
		border: none;
		border-radius: var(--radius-lg);
		background: none;
		font-weight: 400;
		text-align: left;

		@include hover {
			&:hover:not(:disabled) {
				background: none;
			}
		}

		@include hover {
			&:hover :global(.thumb) {
				border-color: var(--border-strong);
			}
		}
	}

	.caption {
		overflow: hidden;
		color: var(--text-2);
		font-size: var(--fs-sm);
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.rename {
		margin-top: 0.25rem;
		padding: 0.125rem 0.375rem;
		font-size: var(--fs-sm);
	}

	.selected :global(.thumb) {
		@include focus-ring(1px);
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

	// over the thumbnail's corner, on hover
	.menu {
		position: absolute;
		top: 0.375rem;
		right: 0.375rem;
		border-radius: var(--radius);
		background: var(--surface);
		box-shadow: var(--shadow);
		opacity: 0;

		:global(button.icon) {
			width: 1.75rem;
			height: 1.75rem;
		}
	}

	.item:hover .menu,
	.item:focus-within .menu,
	.menu:has(:global([aria-expanded='true'])) {
		opacity: 1;
	}

	@media (hover: none) {
		.menu {
			opacity: 1;
		}
	}
</style>
