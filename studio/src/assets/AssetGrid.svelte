<script lang="ts">
	import type { Asset } from './api';
	import AssetCard from './AssetCard.svelte';
	import type { MenuItem } from 'zipp-ui/Menu.svelte';
	import { ASSETS_DRAG } from './uploads.svelte';

	/**
	 * The files as cards. With `draggable` a card drags onto a folder of
	 * the tree; with `menu` it has a ⋯ menu, where `rename` turns the
	 * title into an input for `onrename`.
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
		<AssetCard
			asset={a}
			onopen={() => onpick(a)}
			selected={selected.includes(a.id)}
			{draggable}
			ondragstart={e => draggable && ondragstart(e, a)}
			menu={menu?.(a, () => rename(a))}
			caption={renaming === a.id ? renameInput : undefined}
		/>
		{#snippet renameInput()}
			<input
				class="rename"
				bind:value={name}
				aria-label="Filename"
				use:focus
				onkeydown={e => onkeydown(e, a)}
				onblur={() => save(a)}
			/>
		{/snippet}
	{:else}
		<p class="empty">{loading ? 'Loading…' : empty}</p>
	{/each}
</div>

<style lang="scss">
	.grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(10rem, 1fr));
		gap: 1rem;
	}

	// the whole row, not one card's column
	.empty {
		grid-column: 1 / -1;
	}

	.rename {
		flex: 1;
		min-width: 0;
		padding: 0.125rem 0.375rem;
		font-size: var(--fs-sm);
	}
</style>
