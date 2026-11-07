<script lang="ts">
	import type { Folder } from './api';
	import { root } from './folders';
	import { FolderTreeState } from './folderTree.svelte';
	import { ASSETS_DRAG, readDrop } from './uploads.svelte';
	import type { Upload } from './files';
	import Icon from 'zipp-ui/Icon.svelte';
	import Menu, { type MenuItem } from 'zipp-ui/Menu.svelte';
	import TreeCaret from 'zipp-ui/TreeCaret.svelte';
	import NameInput from './NameInput.svelte';
	import { t } from '../i18n/index.svelte';

	/**
	 * The folders of the asset library as a tree below its top (`root()`),
	 * which holds the assets in no folder. Opens one with `onselect`; with
	 * `editable` it adds, renames and deletes (only empty ones) folders
	 * (folderTree.svelte.ts), and assets dragged from the grid or files
	 * from the computer drop onto a folder.
	 */
	let {
		folders,
		current,
		searching = false,
		onselect,
		editable = false,
		deletable = false,
		onchange,
		onmove,
		onupload,
		ondownload,
		top = 0,
	}: {
		folders: Folder[];
		/** null: the top */
		current: number | null;
		/** a search looks in all folders: none is shown as open */
		searching?: boolean;
		onselect: (id: number | null) => void;
		editable?: boolean;
		deletable?: boolean;
		/** after a folder was added, renamed or deleted */
		onchange?: () => void;
		onmove?: (ids: number[], folder: number | null) => void;
		onupload?: (items: Upload[], folder: number | null) => void;
		/** a folder's ⋯ downloads it as a zip */
		ondownload?: (folder: Folder) => void;
		/** the assets in no folder */
		top?: number;
	} = $props();

	const tree = new FolderTreeState({
		folders: () => folders,
		current: () => current,
		onselect: id => onselect(id),
		onchange: () => onchange?.(),
	});
	$effect(() => tree.reveal());

	function items(f: Folder): MenuItem[] {
		const list: MenuItem[] = [];
		if (editable) {
			list.push(
				{
					label: t('common.rename'),
					icon: 'edit',
					onclick: () => tree.startRename(f),
				},
				{
					label: t('assets.newFolder'),
					icon: 'folder-plus',
					onclick: () => tree.startNew(f.id),
				},
			);
		}
		if (ondownload)
			list.push({
				label: t('assets.downloadZip'),
				icon: 'download',
				onclick: () => ondownload(f),
			});
		if (deletable) {
			list.push({
				label: t('common.delete'),
				icon: 'delete',
				danger: true,
				onclick: () => tree.remove(f),
			});
		}
		return list;
	}

	/** The folder a drag is over, `top` for the library. */
	let over = $state<number | 'top' | null>(null);

	function canDrop(e: DragEvent) {
		const types = e.dataTransfer?.types ?? [];
		if (types.includes(ASSETS_DRAG)) return !!onmove;
		return types.includes('Files') && !!onupload;
	}

	function dragover(e: DragEvent, id: number | null) {
		if (!editable || !canDrop(e)) return;
		e.preventDefault();
		over = id ?? 'top';
	}

	function drop(e: DragEvent, id: number | null) {
		over = null;
		if (!editable || !canDrop(e)) return;
		// the page's drop sees it was taken
		e.preventDefault();
		const data = e.dataTransfer!;
		const ids = data.getData(ASSETS_DRAG);
		if (ids) onmove?.(JSON.parse(ids), id);
		else readDrop(data, items => onupload?.(items, id));
	}
</script>

{#snippet input(depth: number)}
	<div class="row editing" style:--depth={depth}>
		<TreeCaret />
		<Icon name="folder" size={15} />
		<NameInput
			value={tree.editedName}
			label={t('assets.folderName')}
			placeholder={t('assets.folderName')}
			onsave={name => tree.save(name)}
			oncancel={() => (tree.editing = null)}
		/>
	</div>
	{#if tree.error}
		<p class="error" style:--depth={depth}>{tree.error}</p>
	{/if}
{/snippet}

<nav class="tree" aria-label={t('assets.folders')}>
	<div class="head overline">
		<span>{t('assets.folders')}</span>
		{#if editable}
			<button
				type="button"
				class="icon"
				title={t('assets.newFolder')}
				aria-label={t('assets.newFolder')}
				onclick={() => tree.startNew(current)}
			>
				<Icon name="folder-plus" />
			</button>
		{/if}
	</div>
	<!-- a failed delete; one of adding or renaming shows at its input -->
	{#if tree.error && !tree.editing}
		<p class="error" style:--depth={0}>{tree.error}</p>
	{/if}

	<div
		class="row nav-item"
		class:active={!searching && current === null}
		class:over={over === 'top'}
		role="presentation"
		ondragover={e => dragover(e, null)}
		ondragleave={() => (over = null)}
		ondrop={e => drop(e, null)}
	>
		<TreeCaret />
		<button type="button" class="name" onclick={() => onselect(null)}>
			<Icon name="dashboard" size={15} />
			<span class="label">{root()}</span>
			{#if top}<span class="count">{top}</span>{/if}
		</button>
		{#if editable || deletable}<span class="menu-space"></span>{/if}
	</div>
	{#if tree.editing && 'parent' in tree.editing && tree.editing.parent === null}
		{@render input(1)}
	{/if}

	<!-- all folders are in the library, one step in -->
	{#each tree.rows as { folder: f, depth: d } (f.id)}
		{@const depth = d + 1}
		{#if tree.editing && 'id' in tree.editing && tree.editing.id === f.id}
			{@render input(depth)}
		{:else}
			<div
				class="row nav-item"
				class:active={!searching && current === f.id}
				class:over={over === f.id}
				style:--depth={depth}
				data-folder={f.name}
				data-menu-area
				role="presentation"
				ondragover={e => dragover(e, f.id)}
				ondragleave={() => (over = null)}
				ondrop={e => drop(e, f.id)}
			>
				<TreeCaret
					open={tree.hasChildren(f.id)
						? tree.open.has(f.id)
						: undefined}
					onclick={() => tree.toggle(f.id)}
				/>
				<button
					type="button"
					class="name"
					onclick={() => tree.select(f.id)}
				>
					<Icon name="folder" size={15} />
					<span class="label">{f.name}</span>
					{#if f.assets}<span class="count">{f.assets}</span>{/if}
				</button>
				<span class="menu"><Menu items={items(f)} /></span>
			</div>
		{/if}
		{#if tree.editing && 'parent' in tree.editing && tree.editing.parent === f.id}
			{@render input(depth + 1)}
		{/if}
	{/each}
</nav>

<style lang="scss">
	.tree {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.head {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 0.25rem;
		padding-left: 0.5rem;
	}

	.row {
		--depth: 0;
		display: flex;
		align-items: center;
		min-height: 2rem;
		padding-left: calc(var(--depth) * 1rem);

		&.over {
			@include focus-ring(-1px);
			background: var(--accent-soft);
		}
	}

	.name {
		display: flex;
		flex: 1;
		align-items: center;
		gap: 0.5rem;
		min-width: 0;
		padding: 0.375rem 0.25rem;
		border: none;
		background: none;
		color: inherit;
		font-weight: 500;
		text-align: left;

		@include hover {
			&:hover:not(:disabled) {
				background: none;
			}
		}
	}

	.label {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.count {
		margin-left: auto;
		color: var(--muted);
		font-size: var(--fs-xs);
		font-weight: 400;
	}

	// where the ⋯ of the folders is, their counts line up
	.menu-space {
		flex-shrink: 0;
		width: 1.75rem;
	}

	.menu :global(button.icon) {
		width: 1.75rem;
		height: 1.75rem;
	}

	// the ⋯ shows on the row of the pointer and the open one, always on
	// touch screens
	@include hover {
		.menu {
			opacity: 0;
		}

		.row:hover .menu,
		.row.active .menu,
		.row:focus-within .menu,
		.menu:has(:global([aria-expanded='true'])) {
			opacity: 1;
		}
	}

	.editing {
		gap: 0.5rem;
		padding-right: 0.25rem;
	}

	.error {
		margin: 0.25rem 0 0.25rem calc(var(--depth) * 1rem + 1.25rem);
		font-size: var(--fs-sm);
	}
</style>
