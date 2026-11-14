<script>
	import AddGroup from 'zipp-ui/AddGroup.svelte';
	import DragHandle from 'zipp-ui/DragHandle.svelte';
	import ItemActions from 'zipp-ui/ItemActions.svelte';
	import ItemCard from 'zipp-ui/ItemCard.svelte';
	import Segmented from 'zipp-ui/Segmented.svelte';
	import SortLine from 'zipp-ui/SortLine.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import { Sortable } from 'zipp-ui/sortable';
	import {
		addTo,
		freeLabel,
		moveTo,
		pagesOf,
		removePage,
		rename,
		reorder,
	} from './formFields.js';
	import { texts } from './texts.js';

	/**
	 * The fields of a form, the editor of `formFields`: its pages as tabs
	 * (a form of one page shows none, only Add page), in a tab the
	 * studio's editors of the field blocks, to drag, duplicate, switch off
	 * or move to another page. A new page has no fields until one is
	 * added, it lives here until then.
	 */
	let { value, onchange, readonly, blocks } = $props();
	const tx = texts();
	const uid = $props.id();

	const items = $derived(value ?? []);
	// pages without fields yet
	let extra = $state([]);
	const pages = $derived(pagesOf(items, extra));
	let active = $state(0);
	const page = $derived(pages[Math.min(active, pages.length - 1)]);
	const nameOf = (p, i) => p.label || `${tx.page} ${i + 1}`;

	const set = next => onchange(next.length ? next : null);

	function add(type) {
		const item = { page: page.label, block: blocks.create(type) };
		extra = extra.filter(l => l !== page.label);
		set(addTo(items, item));
	}

	function addPage() {
		const labels = pages.map(p => p.label);
		const label = freeLabel(labels, `${tx.page} ${pages.length + 1}`);
		extra = [...extra, label];
		active = pages.length;
	}

	function renamePage(e) {
		const labels = pages.filter(p => p !== page).map(p => p.label);
		const label = freeLabel(labels, e.currentTarget.value.trim());
		extra = extra.map(l => (l === page.label ? label : l));
		if (page.rows.length) set(rename(items, page.label, label));
	}

	function deletePage() {
		if (page.rows.length && !confirm(tx.confirmDeletePage)) return;
		extra = extra.filter(l => l !== page.label);
		if (page.rows.length) set(removePage(items, page.label));
		active = Math.max(0, active - 1);
	}

	function duplicate(row) {
		const item = items[row];
		set(addTo(items, { ...item, block: blocks.copy(item.block) }));
	}

	function switchOff(row) {
		const { enabled, ...block } = items[row].block;
		const next = [...items];
		next[row] = {
			...items[row],
			block: enabled === false ? block : { ...block, enabled: false },
		};
		set(next);
	}

	function move(row, label) {
		extra = extra.filter(l => l !== label);
		set(moveTo(items, row, label));
	}

	let list = $state();
	const sort = new Sortable({
		container: () => list,
		depths: () => page.rows.map(() => 0),
		onmove: (from, drop) =>
			set(reorder(items, page.rows, from, drop.index)),
	});

	const typeName = type =>
		blocks.types.find(t => t.handle === type)?.name ?? type;

	/** The studio's editor of a block. */
	function editor(target, id) {
		const m = blocks.mount(target, id);
		return { destroy: () => m.destroy() };
	}
</script>

<div class="form-fields">
	{#if pages.length > 1}
		<div class="tabs">
			<Segmented
				tabs
				label={tx.pages}
				options={pages.map((p, i) => ({
					value: i,
					label: nameOf(p, i),
				}))}
				value={pages.indexOf(page)}
				onchange={i => (active = i)}
			/>
		</div>
		{#if !readonly}
			<div class="page-head">
				<label class="stacked">
					{tx.pageName}
					<input
						id="{uid}-name"
						value={page.label}
						placeholder={nameOf(page, pages.indexOf(page))}
						onchange={renamePage}
					/>
				</label>
				<button type="button" class="light danger" onclick={deletePage}>
					<Icon name="delete" />
					{tx.deletePage}
				</button>
			</div>
			<p class="note">{tx.pageNameNote}</p>
		{/if}
	{/if}

	<div class="items" bind:this={list}>
		{#each page.rows as row, i (items[row].block.id)}
			{@const item = items[row]}
			<div data-sort class:sort-dragging={sort.moving(i)}>
				<ItemCard
					kind={typeName(item.block.type)}
					off={item.block.enabled === false}
				>
					{#snippet handle()}
						{#if !readonly}
							<DragHandle sortable={sort} index={i} />
						{/if}
					{/snippet}
					{#snippet summary()}{item.block.fields.label ??
							''}{/snippet}
					{#snippet actions()}
						{#if !readonly}
							{#if pages.length > 1}
								<select
									class="move"
									aria-label={tx.moveTo}
									title={tx.moveTo}
									value=""
									onchange={e =>
										move(row, e.currentTarget.value)}
								>
									<option value="" disabled>
										{tx.moveTo}
									</option>
									{#each pages as p, k (k)}
										{#if p !== page}
											<option value={p.label}>
												{nameOf(p, k)}
											</option>
										{/if}
									{/each}
								</select>
							{/if}
							<ItemActions
								off={item.block.enabled === false}
								onswitch={() => switchOff(row)}
								onduplicate={() => duplicate(row)}
								onremove={() =>
									set(items.filter((_, k) => k !== row))}
							/>
						{/if}
					{/snippet}
					<div use:editor={item.block.id}></div>
				</ItemCard>
			</div>
		{:else}
			<p class="empty">{tx.noFields}</p>
		{/each}
		<SortLine sortable={sort} />
	</div>

	{#if !readonly}
		<div class="add">
			<AddGroup
				options={blocks.types.map(t => ({
					value: t.handle,
					label: t.name,
				}))}
				onadd={add}
			/>
			<button type="button" class="light" onclick={addPage}>
				<Icon name="add" />
				{tx.addPage}
			</button>
		</div>
	{/if}
</div>

<style lang="scss">
	@use 'zipp-ui/vars.scss' as *;

	.form-fields {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.page-head {
		display: flex;
		align-items: flex-end;
		gap: 0.75rem;

		label {
			flex: 1;
		}
	}

	.note {
		margin: -0.5rem 0 0;
	}

	.items {
		position: relative;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.move {
		width: auto;
		padding: 0.125rem 0.375rem;
		font-size: var(--fs-sm);
	}

	.add {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 0.75rem;
	}

	@include max-tablet {
		.page-head {
			flex-direction: column;
			align-items: stretch;
		}
	}
</style>
