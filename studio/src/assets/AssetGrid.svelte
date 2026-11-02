<script lang="ts">
	import type { Asset } from './api';
	import AssetCard from './AssetCard.svelte';
	import NameInput from './NameInput.svelte';
	import type { MenuItem } from 'zipp-ui/Menu.svelte';
	import { ASSETS_DRAG } from './uploads.svelte';
	import { t } from '../i18n/index.svelte';

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
		empty,
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
</script>

<div class="grid">
	{#each assets as a (a.id)}
		<AssetCard
			asset={a}
			onopen={() => onpick(a)}
			selected={selected.includes(a.id)}
			{draggable}
			ondragstart={e => draggable && ondragstart(e, a)}
			menu={menu?.(a, () => (renaming = a.id))}
			caption={renaming === a.id ? renameInput : undefined}
		/>
		{#snippet renameInput()}
			<NameInput
				value={a.filename}
				label={t('assets.filename')}
				selectStem
				small
				onsave={async n => {
					const ok = !onrename || (await onrename(a, n));
					if (ok) renaming = null;
					return ok;
				}}
				oncancel={() => (renaming = null)}
			/>
		{/snippet}
	{:else}
		<p class="empty">
			{loading ? t('common.loading') : (empty ?? t('assets.noAssets'))}
		</p>
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
</style>
