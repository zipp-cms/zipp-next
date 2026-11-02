<script lang="ts">
	import type { FieldOf, Schema, Values } from '../core/schema';
	import { t } from '../i18n/index.svelte';
	import { valueOf } from '../core/schema';
	import type { Refs } from '../entries/api';
	import AddButton from 'zipp-ui/AddButton.svelte';
	import ItemActions from 'zipp-ui/ItemActions.svelte';
	import DragHandle from 'zipp-ui/DragHandle.svelte';
	import SortLine from 'zipp-ui/SortLine.svelte';
	import { Sortable, moved } from 'zipp-ui/sortable';
	import { inserted, removed, replaced } from '../core/list';
	import TableCell from './TableCell.svelte';

	/**
	 * Rows of the columns of the code, like Craft's table field. A
	 * plugin's module mounts it without schema and refs, links need them.
	 */
	let {
		schema,
		field,
		values,
		refs = {},
	}: {
		schema?: Schema;
		field: FieldOf<'table'>;
		values: Values;
		refs?: Refs;
	} = $props();

	const columns = $derived(field.columns);
	const rows = $derived(valueOf(values, field) ?? []);

	function set(next: Values[]) {
		values[field.handle] = next.length ? next : null;
	}

	let list = $state<HTMLElement>();
	const sort = new Sortable({
		container: () => list,
		depths: () => rows.map(() => 0),
		onmove: (from, drop) => set(moved(rows, from, drop)),
	});

	// rows have no id; a key per row, so a moved or removed row takes
	// its cells' editors along (an edit makes a new row, it keeps the key)
	let keys = new WeakMap<Values, number>();
	let nextKey = 0;
	function key(row: Values) {
		if (!keys.has(row)) keys.set(row, nextKey++);
		return keys.get(row)!;
	}

	// nothing leaves the row, so an empty row is one
	function cell(i: number, column: string, value: unknown) {
		const k = key(rows[i]);
		const { [column]: _, ...row } = rows[i];
		set(
			replaced(
				rows,
				i,
				value === null ? row : { ...row, [column]: value },
			),
		);
		// the one in the state, a proxy
		keys.set(rows[i], k);
	}
</script>

<div class="wrap" bind:this={list}>
	<table>
		<thead>
			<tr>
				<th></th>
				{#each columns as c (c.handle)}<th>{c.label}</th>{/each}
				<th></th>
			</tr>
		</thead>
		<tbody>
			{#each rows as row, i (key(row))}
				<tr data-sort class:sort-dragging={sort.dragging === i}>
					<td class="handle">
						<DragHandle
							sortable={sort}
							index={i}
							label={t('fields.moveRow')}
						/>
					</td>
					{#each columns as c (c.handle)}
						<td class={c.kind}>
							<TableCell
								{schema}
								{refs}
								column={c}
								value={row[c.handle] ?? null}
								onchange={v => cell(i, c.handle, v)}
							/>
						</td>
					{/each}
					<td class="actions">
						<ItemActions
							onremove={() => set(removed(rows, i))}
							removeLabel={t('fields.removeRow')}
						/>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
	<SortLine sortable={sort} />
</div>
<AddButton onclick={() => set(inserted(rows, {}))}>
	{t('fields.addRow')}
</AddButton>

<style lang="scss">
	.wrap {
		position: relative;
	}

	.handle {
		width: 1.75rem;
	}

	table {
		width: 100%;
		margin-bottom: 0.375rem;
		border-collapse: collapse;

		// the button further from the rows than from the column labels
		&:has(tbody tr) {
			margin-bottom: 0.75rem;
		}
	}

	th {
		padding: 0 0.25rem 0.25rem;
		text-align: left;
		font-size: var(--fs-md);
		font-weight: normal;
		color: var(--muted);
	}

	td {
		padding: 2px 0.25rem;
	}

	// a switch only as wide as it is
	.boolean {
		width: 1%;
	}

	.actions {
		width: 1%;
	}
</style>
