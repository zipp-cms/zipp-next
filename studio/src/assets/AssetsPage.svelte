<script module lang="ts">
	import type { LoadProps } from '../core/router.svelte';
	import { listAssets as list, listFolders } from './api';

	/** `?folder=12` for a folder, none for the top. */
	function folderOf(param: string | null): number | null {
		const id = Number(param);
		return param && Number.isInteger(id) ? id : null;
	}

	/** The folders and the newest assets of the one in the url. */
	export async function loadProps(_: unknown, lp: LoadProps) {
		const asked = folderOf(lp.req.search.get('folder'));
		const [{ folders, top }, first] = await Promise.all([
			listFolders(),
			list({ folder: asked }),
		]);
		// a deleted folder
		const known = asked === null || folders.some(f => f.id === asked);
		return { folders, top, first, folder: known ? asked : null };
	}
</script>

<script lang="ts">
	import { untrack } from 'svelte';
	import { t } from '../i18n/index.svelte';
	import PageTop from 'zipp-ui/PageTop.svelte';
	import { kinds, type Asset, type Folder } from './api';
	import { root } from './folders';
	import FolderTree from './FolderTree.svelte';
	import { type Schema } from '../core/schema';
	import { canAssets } from '../core/access';
	import AssetGrid from './AssetGrid.svelte';
	import AssetDetail from './AssetDetail.svelte';
	import SearchInput from 'zipp-ui/SearchInput.svelte';
	import Segmented from 'zipp-ui/Segmented.svelte';
	import DropOverlay from './DropOverlay.svelte';
	import UploadButton from 'zipp-ui/UploadButton.svelte';
	import type { MenuItem } from 'zipp-ui/Menu.svelte';
	import { AssetsPageState } from './assetsPage.svelte';
	import { linkItems } from './actions';
	import { FileDrag, uploads } from './uploads.svelte';
	import { fromInput } from './files';

	let {
		schema,
		first,
		folders,
		folder,
		top,
	}: {
		schema: Schema;
		first: Asset[];
		folders: Folder[];
		folder: number | null;
		top: number;
	} = $props();
	const canEdit = $derived(canAssets(schema, 'edit'));
	const canDelete = $derived(canAssets(schema, 'delete'));

	const page = untrack(
		() => new AssetsPageState({ folders, top, folder, assets: first }),
	);
	const { lib, action } = page;

	// files dropped anywhere on the page but on a folder go into the open one
	const drag = new FileDrag(
		items => page.upload(items),
		() => canEdit,
	);

	function menu(a: Asset, startRename: () => void): MenuItem[] {
		const items: MenuItem[] = [
			{
				label: t('common.open'),
				icon: 'view',
				onclick: () => (page.current = a),
			},
		];
		if (canEdit)
			items.push({
				label: t('common.rename'),
				icon: 'edit',
				onclick: startRename,
			});
		items.push(...linkItems(a));
		if (canEdit && a.folderId !== null)
			items.push({
				label: t('assets.moveTo', { folder: root() }),
				icon: 'file',
				onclick: () => page.move([a.id], null),
			});
		if (canDelete)
			items.push({
				label: t('common.delete'),
				icon: 'delete',
				danger: true,
				onclick: () => page.remove(a),
			});
		return items;
	}
</script>

<div
	class="page"
	role="region"
	aria-label={t('assets.title')}
	ondragenter={drag.dragenter}
	ondragleave={drag.dragleave}
	ondragover={drag.dragover}
	ondrop={drag.drop}
>
	<PageTop
		title={t('assets.title')}
		note={page.zipping
			? t('assets.zipping', { name: page.zipping })
			: undefined}
	>
		{#snippet actions()}
			{#if canEdit}
				<UploadButton
					onfiles={f => page.upload(fromInput(f))}
					directory
					icon="folder-plus"
				>
					{t('assets.uploadFolder')}
				</UploadButton>
				<UploadButton
					onfiles={f => page.upload(fromInput(f))}
					multiple
					primary
				>
					{uploads.active
						? t('assets.uploading')
						: t('common.upload')}
				</UploadButton>
			{/if}
		{/snippet}
	</PageTop>

	{#if lib.error || action.error}
		<p class="error">{lib.error || action.error}</p>
	{/if}

	<div class="library">
		<aside class="folders">
			<FolderTree
				folders={lib.folders}
				top={lib.top}
				current={lib.folder}
				searching={!!lib.search}
				onselect={id => page.select(id)}
				editable={canEdit}
				deletable={canDelete}
				onchange={() => lib.loadFolders()}
				onmove={(ids, to) => page.move(ids, to)}
				onupload={(items, into) => page.upload(items, into)}
				ondownload={f => page.download(f)}
			/>
		</aside>

		<div class="card files">
			{#if drag.over}
				<DropOverlay label={t('assets.dropInto', { path: lib.path })} />
			{/if}
			<div class="top">
				<h2 class="truncate">
					{lib.search ? t('assets.inAllFolders') : lib.path}
				</h2>
				<Segmented
					options={kinds()}
					value={lib.kind}
					onchange={k => lib.filter(k)}
					label={t('assets.type')}
				/>
				<SearchInput
					bind:value={() => lib.search, v => lib.type(v)}
					placeholder={t('assets.searchFiles')}
				/>
			</div>
			<AssetGrid
				assets={lib.assets}
				onpick={a => (page.current = a)}
				draggable={canEdit}
				{menu}
				onrename={(a, name) => page.rename(a, name)}
				empty={lib.empty}
			/>
		</div>
	</div>
</div>

{#if page.current}
	{#key page.current.id}
		<AssetDetail
			{schema}
			folders={lib.folders}
			asset={page.current}
			onsaved={a => page.saved(a)}
			ondeleted={() => page.deleted()}
			onclose={() => page.closed()}
		/>
	{/key}
{/if}

<style lang="scss">
	.page {
		min-height: 80vh;
	}

	.library {
		display: grid;
		grid-template-columns: 15rem minmax(0, 1fr);
		gap: 1.5rem;
		align-items: start;

		@include max-tablet {
			grid-template-columns: minmax(0, 1fr);
		}
	}

	.files {
		position: relative;
		min-height: 20rem;
	}

	.folders {
		position: sticky;
		top: 1rem;
	}

	.top {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1rem;

		@include max-tablet {
			flex-wrap: wrap;
		}

		h2 {
			flex: 1;
			margin: 0;
			font-size: var(--fs-md);
		}

		:global(.search) {
			flex-shrink: 0;
			width: 21.25rem;
			max-width: 50%;
		}
	}
</style>
