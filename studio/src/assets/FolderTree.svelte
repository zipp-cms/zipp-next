<script lang="ts" module>
	/** What an asset tile carries when it is dragged: the asset ids. */
	export const ASSETS_DRAG = 'application/x-zipp-assets';
</script>

<script lang="ts">
	import { untrack } from 'svelte';
	import {
		createFolder,
		deleteFolder,
		folderRows,
		updateFolder,
		type Folder,
	} from './api';
	import Icon from 'zipp-ui/Icon.svelte';
	import Menu, { type MenuItem } from 'zipp-ui/Menu.svelte';

	/**
	 * The folders of the asset library as a tree, "Files" on the top for
	 * the ones in none. Opens one with `onselect`; with `editable` it adds,
	 * renames and deletes (only empty ones) folders, and assets dragged
	 * from the grid or files from the computer drop onto a folder.
	 */
	let {
		folders,
		current,
		onselect,
		editable = false,
		deletable = false,
		onchange,
		onmove,
		onupload,
	}: {
		folders: Folder[];
		/** null: the top */
		current: number | null;
		onselect: (id: number | null) => void;
		editable?: boolean;
		deletable?: boolean;
		/** after a folder was added, renamed or deleted */
		onchange?: (folder?: Folder) => void;
		onmove?: (ids: number[], folder: number | null) => void;
		onupload?: (files: FileList, folder: number | null) => void;
	} = $props();

	const parentOf = (id: number) =>
		folders.find(f => f.id === id)?.parentId ?? null;

	// the folders around the current one start open
	let open = $state(
		untrack(() => {
			const ids = new Set<number>();
			let at = current === null ? null : parentOf(current);
			while (at !== null && !ids.has(at)) {
				ids.add(at);
				at = parentOf(at);
			}
			return ids;
		}),
	);

	const hasChildren = (id: number) => folders.some(f => f.parentId === id);

	/** The rows whose parents are all open. */
	const rows = $derived(
		folderRows(folders).filter(r => {
			let at = r.folder.parentId;
			while (at !== null) {
				if (!open.has(at)) return false;
				at = parentOf(at);
			}
			return true;
		}),
	);

	function toggle(id: number) {
		const next = new Set(open);
		if (!next.delete(id)) next.add(id);
		open = next;
	}

	/** A new folder in `parent`, or a folder being renamed. */
	let editing = $state<{ parent: number | null } | { id: number } | null>(
		null,
	);
	let name = $state('');
	let error = $state('');

	function startNew(parent: number | null) {
		if (parent !== null) open = new Set(open).add(parent);
		editing = { parent };
		name = '';
		error = '';
	}

	function startRename(f: Folder) {
		editing = { id: f.id };
		name = f.name;
		error = '';
	}

	let saving = false;
	async function save() {
		const e = editing;
		if (!e || saving) return;
		if (!name.trim()) {
			editing = null;
			return;
		}
		saving = true;
		try {
			let saved: Folder;
			if ('id' in e) {
				const f = folders.find(f => f.id === e.id)!;
				if (name.trim() === f.name) {
					editing = null;
					return;
				}
				saved = await updateFolder(f.id, f.parentId, name);
			} else {
				saved = await createFolder(e.parent, name);
			}
			editing = null;
			onchange?.(saved);
		} catch (err) {
			error = (err as Error).message;
		} finally {
			saving = false;
		}
	}

	function onkeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			e.preventDefault();
			save();
		} else if (e.key === 'Escape') {
			e.preventDefault();
			editing = null;
		}
	}

	async function remove(f: Folder) {
		if (!confirm(`Delete the folder ${f.name}?`)) return;
		try {
			await deleteFolder(f.id);
			if (current === f.id) onselect(f.parentId);
			onchange?.();
		} catch (err) {
			alert((err as Error).message);
		}
	}

	function items(f: Folder): MenuItem[] {
		const list: MenuItem[] = [];
		if (editable) {
			list.push(
				{
					label: 'Rename',
					icon: 'edit',
					onclick: () => startRename(f),
				},
				{
					label: 'New folder',
					icon: 'folder-plus',
					onclick: () => startNew(f.id),
				},
			);
		}
		if (deletable) {
			list.push({
				label: 'Delete',
				icon: 'delete',
				danger: true,
				onclick: () => remove(f),
			});
		}
		return list;
	}

	/** The folder a drag is over, `top` for Files. */
	let over = $state<number | 'top' | null>(null);

	function canDrop(e: DragEvent) {
		const types = e.dataTransfer?.types ?? [];
		if (types.includes(ASSETS_DRAG)) return !!onmove;
		return types.includes('Files') && !!onupload;
	}

	function dragover(e: DragEvent, id: number | null) {
		if (!editable || !canDrop(e)) return;
		e.preventDefault();
		e.stopPropagation();
		over = id ?? 'top';
	}

	function drop(e: DragEvent, id: number | null) {
		over = null;
		if (!editable || !canDrop(e)) return;
		e.preventDefault();
		e.stopPropagation();
		const data = e.dataTransfer!;
		const ids = data.getData(ASSETS_DRAG);
		if (ids) onmove?.(JSON.parse(ids), id);
		else if (data.files.length) onupload?.(data.files, id);
	}

	function focus(el: HTMLInputElement) {
		el.focus();
		el.select();
	}
</script>

{#snippet input(depth: number)}
	<div class="row editing" style:--depth={depth}>
		<span class="chevron"></span>
		<Icon name="folder" size={15} />
		<input
			class="name-input"
			bind:value={name}
			placeholder="Folder name"
			aria-label="Folder name"
			use:focus
			{onkeydown}
			onblur={save}
		/>
	</div>
	{#if error}<p class="error" style:--depth={depth}>{error}</p>{/if}
{/snippet}

<nav class="tree" aria-label="Folders">
	<div class="head">
		<span>Folders</span>
		{#if editable}
			<button
				type="button"
				class="icon"
				title="New folder"
				aria-label="New folder"
				onclick={() => startNew(current)}
			>
				<Icon name="folder-plus" size={16} />
			</button>
		{/if}
	</div>

	<div
		class="row"
		class:active={current === null}
		class:over={over === 'top'}
		role="presentation"
		ondragover={e => dragover(e, null)}
		ondragleave={() => (over = null)}
		ondrop={e => drop(e, null)}
	>
		<span class="chevron"></span>
		<button type="button" class="name" onclick={() => onselect(null)}>
			<Icon name="file" size={15} />
			<span class="label">Files</span>
		</button>
	</div>
	{#if editing && 'parent' in editing && editing.parent === null}
		{@render input(0)}
	{/if}

	{#each rows as { folder: f, depth } (f.id)}
		{#if editing && 'id' in editing && editing.id === f.id}
			{@render input(depth)}
		{:else}
			<div
				class="row"
				class:active={current === f.id}
				class:over={over === f.id}
				style:--depth={depth}
				data-folder={f.name}
				role="presentation"
				ondragover={e => dragover(e, f.id)}
				ondragleave={() => (over = null)}
				ondrop={e => drop(e, f.id)}
			>
				{#if hasChildren(f.id)}
					<button
						type="button"
						class="chevron"
						aria-label={open.has(f.id) ? 'Collapse' : 'Expand'}
						aria-expanded={open.has(f.id)}
						onclick={() => toggle(f.id)}
					>
						<Icon
							name={open.has(f.id)
								? 'chevron-down'
								: 'chevron-right'}
							size={14}
						/>
					</button>
				{:else}
					<span class="chevron"></span>
				{/if}
				<button
					type="button"
					class="name"
					onclick={() => onselect(f.id)}
				>
					<Icon name="folder" size={15} />
					<span class="label">{f.name}</span>
					{#if f.assets}<span class="count">{f.assets}</span>{/if}
				</button>
				<span class="menu"><Menu items={items(f)} /></span>
			</div>
		{/if}
		{#if editing && 'parent' in editing && editing.parent === f.id}
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
		color: var(--muted);
		font-size: var(--fs-xs);
		font-weight: 600;
		letter-spacing: 0.04em;
		text-transform: uppercase;
	}

	.row {
		--depth: 0;
		display: flex;
		align-items: center;
		min-height: 2rem;
		padding-left: calc(var(--depth) * 1rem);
		border-radius: var(--radius);
		color: var(--text-2);

		@include hover {
			&:hover {
				color: var(--text);
				background: var(--hover);
			}
		}

		&.active {
			color: var(--accent);
			background: var(--accent-soft);
		}

		&.over {
			@include focus-ring(-1px);
			background: var(--accent-soft);
		}
	}

	.chevron {
		display: grid;
		flex-shrink: 0;
		place-items: center;
		width: 1.25rem;
		height: 1.25rem;
		padding: 0;
		border: none;
		background: none;
		color: inherit;
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

	.menu {
		opacity: 0;

		:global(button.icon) {
			width: 1.75rem;
			height: 1.75rem;
		}
	}

	.row:hover .menu,
	.row:focus-within .menu,
	.menu:has(:global([aria-expanded='true'])) {
		opacity: 1;
	}

	@media (hover: none) {
		.menu {
			opacity: 1;
		}
	}

	.editing {
		gap: 0.5rem;
		padding-right: 0.25rem;

		.name-input {
			flex: 1;
			min-width: 0;
			padding: 0.25rem 0.375rem;
		}
	}

	.error {
		margin: 0.25rem 0 0.25rem calc(var(--depth) * 1rem + 1.25rem);
		font-size: var(--fs-sm);
	}
</style>
