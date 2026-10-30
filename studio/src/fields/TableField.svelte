<script lang="ts">
	import type { FieldOf, Values } from '../core/schema';
	import { valueOf } from '../core/schema';
	import AddButton from 'zipp-ui/AddButton.svelte';
	import ItemActions from 'zipp-ui/ItemActions.svelte';
	import DragHandle from 'zipp-ui/DragHandle.svelte';
	import SortLine from 'zipp-ui/SortLine.svelte';
	import { Sortable, moved } from 'zipp-ui/sortable';
	import { inserted, removed, replaced } from '../core/list';

	/** Rows of the columns of the code, like Craft's table field. */
	let { field, values }: { field: FieldOf<'table'>; values: Values } =
		$props();

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

	function cell(i: number, column: string, text: string) {
		set(replaced(rows, i, { ...rows[i], [column]: text }));
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
			{#each rows as row, i (i)}
				<tr data-sort class:sort-dragging={sort.dragging === i}>
					<td class="handle">
						<DragHandle
							sortable={sort}
							index={i}
							label="Move the row"
						/>
					</td>
					{#each columns as c (c.handle)}
						<td>
							<input
								type={c.kind === 'email' ? 'email' : 'text'}
								placeholder={c.kind === 'url'
									? 'https://…'
									: ''}
								value={row[c.handle] ?? ''}
								oninput={e =>
									cell(i, c.handle, e.currentTarget.value)}
							/>
						</td>
					{/each}
					<td class="actions">
						<ItemActions
							onremove={() => set(removed(rows, i))}
							removeLabel="Remove the row"
						/>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
	<SortLine sortable={sort} />
</div>
<AddButton onclick={() => set(inserted(rows, {}))}>Add a row</AddButton>

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

	.actions {
		width: 1%;
	}
</style>
