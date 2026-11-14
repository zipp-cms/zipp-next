<script>
	import DragHandle from 'zipp-ui/DragHandle.svelte';
	import FieldRow from 'zipp-ui/FieldRow.svelte';
	import Flyout from 'zipp-ui/Flyout.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import Menu from 'zipp-ui/Menu.svelte';
	import Popover from 'zipp-ui/Popover.svelte';
	import Segmented from 'zipp-ui/Segmented.svelte';
	import SortLine from 'zipp-ui/SortLine.svelte';
	import { Sortable } from 'zipp-ui/sortable';
	import FieldCard from './FieldCard.svelte';
	import {
		PAGE,
		WIDTHS,
		flatten,
		groupsOf,
		labelOf,
		moved,
		renamed,
		span,
		toPage,
		withFields,
	} from './formFields.js';
	import { texts } from './texts.js';

	/**
	 * The builder of a form, the editor of `formFields`: its pages as
	 * tabs (drag them into their order, ⋯ to rename, move or delete, + for
	 * a new one), the fields of the page on screen as a grid of their
	 * widths with a preview of their inputs. A click on a field opens its
	 * settings and width in a flyout, the studio's editor of its block;
	 * its ⋯ menu duplicates, moves it to another page, switches it off or
	 * deletes it.
	 */
	let { value, onchange, readonly, blocks } = $props();
	const tx = texts();

	const items = $derived(value ?? []);
	const groups = $derived(groupsOf(items));
	let active = $state(0);
	const at = $derived(Math.min(active, groups.length - 1));
	const group = $derived(groups[at]);
	const fieldTypes = $derived(blocks.types.filter(t => t.handle !== PAGE));
	const typeName = type =>
		blocks.types.find(t => t.handle === type)?.name ?? type;
	const pageName = (g, i) => labelOf(g) || `${tx.page} ${i + 1}`;

	const newPage = () => ({ block: blocks.create(PAGE) });
	function set(next) {
		const out = flatten(next, newPage);
		onchange(out.length ? out : null);
	}

	// pages

	let renaming = $state(null);

	function addPage() {
		const page = newPage();
		page.block.fields.label = `${tx.page} ${groups.length + 1}`;
		set([...groups, { page, fields: [] }]);
		active = groups.length;
	}

	function rename(i, label) {
		if (renaming !== i) return;
		renaming = null;
		set(renamed(groups, i, label.trim(), newPage));
	}

	/**
	 * Focuses the name's input once the menu that opened it closed and
	 * gave the focus back to its button; a blur before that isn't the
	 * end of renaming.
	 */
	function focusName(node, i) {
		let ready = false;
		const id = setTimeout(() => {
			node.focus();
			node.select();
			ready = true;
		}, 50);
		const blur = () => ready && rename(i, node.value);
		node.addEventListener('blur', blur);
		return {
			destroy() {
				clearTimeout(id);
				node.removeEventListener('blur', blur);
			},
		};
	}

	function movePage(from, to) {
		set(moved(groups, from, to));
		active = to;
	}

	function deletePage(i) {
		if (groups[i].fields.length && !confirm(tx.confirmDeletePage)) return;
		set(groups.filter((_, k) => k !== i));
		active = Math.max(0, i - 1);
	}

	const pageMenu = i => [
		{ label: tx.renamePage, icon: 'edit', onclick: () => (renaming = i) },
		...(i > 0
			? [
					{
						label: tx.moveLeft,
						icon: 'arrow-left',
						onclick: () => movePage(i, i - 1),
					},
				]
			: []),
		...(i < groups.length - 1
			? [
					{
						label: tx.moveRight,
						icon: 'arrow-right',
						onclick: () => movePage(i, i + 1),
					},
				]
			: []),
		...(groups.length > 1
			? [
					{
						label: tx.deletePage,
						icon: 'delete',
						danger: true,
						onclick: () => deletePage(i),
					},
				]
			: []),
	];

	let tabList = $state();
	const tabSort = new Sortable({
		container: () => tabList,
		depths: () => groups.map(() => 0),
		grid: () => true,
		onmove: (from, drop) => movePage(from, drop.index),
	});

	// fields

	// the id of the block whose settings are open, and the value before
	// them for Cancel
	let editing = $state(null);
	let before = null;

	function edit(id) {
		before = value;
		editing = id;
	}

	function apply() {
		editing = null;
	}

	function cancel() {
		editing = null;
		onchange(before);
	}
	const edited = $derived(
		editing &&
			groups
				.flatMap((g, gi) =>
					g.fields.map((item, fi) => ({ item, gi, fi })),
				)
				.find(x => x.item.block.id === editing),
	);

	const fields = f => set(withFields(groups, at, f));

	// a new field: its first input gets the cursor
	let fresh = null;

	function add(type) {
		const block = blocks.create(type);
		const prev = value;
		fields(fs => [...fs, { width: 100, block }]);
		fresh = block.id;
		edit(block.id);
		// Cancel takes the new field away again
		before = prev;
	}

	function duplicate(fi) {
		const item = group.fields[fi];
		const copy = { ...item, block: blocks.copy(item.block) };
		fields(fs => [...fs.slice(0, fi + 1), copy, ...fs.slice(fi + 1)]);
	}

	function switchOff(fi) {
		fields(fs =>
			fs.map((item, k) => {
				if (k !== fi) return item;
				const { enabled, ...block } = item.block;
				return {
					...item,
					block:
						enabled === false
							? block
							: { ...block, enabled: false },
				};
			}),
		);
	}

	function remove(fi) {
		if (editing === group.fields[fi].block.id) editing = null;
		fields(fs => fs.filter((_, k) => k !== fi));
	}

	function setWidth(gi, fi, width) {
		set(
			withFields(groups, gi, fs =>
				fs.map((item, k) => (k === fi ? { ...item, width } : item)),
			),
		);
	}

	const fieldMenu = (item, fi) => [
		{
			label: tx.edit,
			icon: 'edit',
			onclick: () => (editing = item.block.id),
		},
		{ label: tx.duplicate, icon: 'copy', onclick: () => duplicate(fi) },
		...groups
			.map((g, gi) => ({ g, gi }))
			.filter(({ gi }) => gi !== at)
			.map(({ g, gi }) => ({
				label: tx.toPage.replace('{page}', pageName(g, gi)),
				icon: 'move',
				onclick: () => set(toPage(groups, at, fi, gi)),
			})),
		{
			label: item.block.enabled === false ? tx.switchOn : tx.switchOff,
			icon: item.block.enabled === false ? 'view' : 'hide',
			onclick: () => switchOff(fi),
		},
		{
			label: tx.remove,
			icon: 'delete',
			danger: true,
			onclick: () => remove(fi),
		},
	];

	let grid = $state();
	// where the pointer is while a field is dragged
	let pointer = { x: 0, y: 0 };
	$effect(() => {
		const track = e => (pointer = { x: e.clientX, y: e.clientY });
		window.addEventListener('pointermove', track);
		return () => window.removeEventListener('pointermove', track);
	});

	/**
	 * Beside a field (a line next to it) only in the middle of one that
	 * isn't full width, else between the rows (a line across).
	 */
	function besides() {
		const cell = document
			.elementFromPoint(pointer.x, pointer.y)
			?.closest('.cell[data-sort]');
		if (!cell || cell.classList.contains('sort-dragging')) return false;
		if (getComputedStyle(cell).getPropertyValue('--span') === '12') {
			return false;
		}
		const r = cell.getBoundingClientRect();
		const y = (pointer.y - r.top) / r.height;
		return y > 0.25 && y < 0.75;
	}

	const fieldSort = new Sortable({
		container: () => grid,
		depths: () => group.fields.map(() => 0),
		grid: besides,
		onmove: (from, drop) => fields(fs => moved(fs, from, drop.index)),
	});

	/** The studio's editor of a block. */
	function editor(target, id) {
		const m = blocks.mount(target, id);
		if (fresh === id) {
			fresh = null;
			setTimeout(() => target.querySelector('input')?.focus(), 50);
		}
		return { destroy: () => m.destroy() };
	}

	const widthLabel = w =>
		({ 25: '¼', 33: '⅓', 50: '½', 67: '⅔', 75: '¾', 100: tx.full })[w];
</script>

<div class="builder">
	<div class="tabs" role="tablist" aria-label={tx.pages} bind:this={tabList}>
		{#each groups as g, i (g.page?.block.id ?? 'first')}
			<div
				class="tab"
				class:on={i === at}
				class:sort-dragging={tabSort.moving(i)}
				data-sort
				data-menu-area
			>
				{#if !readonly && groups.length > 1}
					<span class="grip">
						<DragHandle sortable={tabSort} index={i} />
					</span>
				{/if}
				{#if renaming === i}
					<input
						class="rename"
						aria-label={tx.pageName}
						value={labelOf(g)}
						placeholder={pageName(g, i)}
						use:focusName={i}
						onkeydown={e => {
							if (e.key === 'Enter')
								rename(i, e.currentTarget.value);
							if (e.key === 'Escape') renaming = null;
						}}
					/>
				{:else}
					<button
						type="button"
						role="tab"
						class="plain name"
						aria-selected={i === at}
						onclick={() => (active = i)}
						ondblclick={() => !readonly && (renaming = i)}
					>
						{pageName(g, i)}
						<span class="count">{g.fields.length}</span>
					</button>
				{/if}
				{#if !readonly && i === at && renaming !== i}
					<Menu items={pageMenu(i)} label={tx.pageActions} />
				{/if}
			</div>
		{/each}
		{#if !readonly}
			<button
				type="button"
				class="plain add-page"
				title={tx.addPage}
				onclick={addPage}
			>
				<Icon name="add" />
				<span>{tx.addPage}</span>
			</button>
		{/if}
		<SortLine sortable={tabSort} />
	</div>

	<div class="grid" bind:this={grid}>
		{#each group.fields as item, fi (item.block.id)}
			<div
				class="cell"
				style:--span={span(item.width)}
				data-sort
				class:sort-dragging={fieldSort.moving(fi)}
			>
				<FieldCard
					{item}
					typeName={typeName(item.block.type)}
					selected={editing === item.block.id}
					{readonly}
					sortable={fieldSort}
					index={fi}
					menu={fieldMenu(item, fi)}
					onedit={() => edit(item.block.id)}
				/>
			</div>
		{:else}
			{#if readonly}<p class="empty">{tx.noFields}</p>{/if}
		{/each}
		{#if !readonly}
			<div class="cell add">
				<Popover menu label={tx.addField}>
					{#snippet trigger({ toggle, open })}
						<button
							type="button"
							class="add-field"
							aria-expanded={open}
							onclick={toggle}
						>
							<Icon name="add" />
							{tx.addField}
						</button>
					{/snippet}
					{#each fieldTypes as t (t.handle)}
						<button
							type="button"
							class="popup-item"
							role="menuitem"
							onclick={() => add(t.handle)}
						>
							{t.name}
						</button>
					{/each}
				</Popover>
			</div>
		{/if}
		<SortLine sortable={fieldSort} />
	</div>
</div>

{#snippet flyoutActions()}
	<button type="button" onclick={cancel}>{tx.cancel}</button>
	<button type="button" class="primary" onclick={apply}>
		<Icon name="check" size={14} />
		{tx.apply}
	</button>
{/snippet}

{#if edited}
	<Flyout
		title={edited.item.block.fields.label ||
			typeName(edited.item.block.type)}
		onclose={readonly ? apply : cancel}
		actions={readonly ? undefined : flyoutActions}
	>
		<div class="settings">
			{#if !readonly}
				<FieldRow label={tx.width}>
					<Segmented
						label={tx.width}
						options={WIDTHS.map(w => ({
							value: w,
							label: widthLabel(w),
						}))}
						value={edited.item.width ?? 100}
						onchange={w => setWidth(edited.gi, edited.fi, w)}
					/>
				</FieldRow>
			{/if}
			{#key editing}
				<div use:editor={editing}></div>
			{/key}
		</div>
	</Flyout>
{/if}

<style lang="scss">
	@use 'zipp-ui/vars.scss' as *;

	.builder {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.tabs {
		position: relative;
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 0.25rem;
		padding-bottom: 0.5rem;
		border-bottom: 1px solid var(--line);
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.125rem;
		padding: 0.125rem 0.25rem;
		border: 1px solid transparent;
		border-radius: var(--radius);

		&.on {
			background: var(--accent-soft);
			border-color: var(--accent-ring);
		}

		.grip {
			display: flex;
			color: var(--muted);
		}
	}

	.name {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.25rem 0.375rem;
		font-weight: 500;

		.on & {
			color: var(--accent);
		}
	}

	.count {
		min-width: 1.25rem;
		padding: 0 0.3125rem;
		color: var(--muted);
		font-size: var(--fs-xs);
		text-align: center;
		background: var(--hover);
		border-radius: 999px;
	}

	.rename {
		width: 12rem;
		padding: 0.25rem 0.5rem;
	}

	.add-page {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.375rem 0.5rem;
		color: var(--muted);

		@include hover {
			color: var(--accent);
		}
	}

	.grid {
		position: relative;
		display: grid;
		grid-template-columns: repeat(12, minmax(0, 1fr));
		gap: 0.625rem;
	}

	.cell {
		grid-column: span var(--span, 12);
		min-width: 0;

		&.add {
			grid-column: span 12;
		}
	}

	.add-field {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		width: 100%;
		padding: 0.75rem;
		color: var(--muted);
		background: transparent;
		border: 1px dashed var(--border-strong);
		border-radius: var(--radius-lg);

		@include hover {
			color: var(--accent);
			border-color: var(--accent);
		}
	}

	.settings {
		display: flex;
		flex-direction: column;
		gap: 0.625rem;
	}

	@include max-tablet {
		.cell {
			grid-column: span 12;
		}
	}
</style>
