<script lang="ts">
	import type { Field, Values } from '../core/schema';
	import AddButton from '../ui/AddButton.svelte';
	import ItemActions from '../ui/ItemActions.svelte';
	import { inserted, moved, removed, replaced } from '../core/list';

	/** Rows of the columns of the code, like Craft's table field. */
	let { field, values }: { field: Field; values: Values } = $props();

	const columns = $derived(field.kind === 'table' ? field.columns : []);
	const rows = $derived<Values[]>(values[field.handle] ?? []);

	function set(next: Values[]) {
		values[field.handle] = next.length ? next : null;
	}

	function cell(i: number, column: string, text: string) {
		set(replaced(rows, i, { ...rows[i], [column]: text }));
	}
</script>

<table>
	<thead>
		<tr>
			{#each columns as c (c.handle)}<th>{c.label}</th>{/each}
			<th></th>
		</tr>
	</thead>
	<tbody>
		{#each rows as row, i (i)}
			<tr>
				{#each columns as c (c.handle)}
					<td>
						<input
							type={c.kind === 'email' ? 'email' : 'text'}
							placeholder={c.kind === 'url' ? 'https://…' : ''}
							value={row[c.handle] ?? ''}
							oninput={e =>
								cell(i, c.handle, e.currentTarget.value)}
						/>
					</td>
				{/each}
				<td class="actions">
					<ItemActions
						index={i}
						count={rows.length}
						onmove={by => set(moved(rows, i, by))}
						onremove={() => set(removed(rows, i))}
						removeTitle="Remove the row"
					/>
				</td>
			</tr>
		{/each}
	</tbody>
</table>
<AddButton onclick={() => set(inserted(rows, {}))}>Add a row</AddButton>

<style>
	table {
		width: 100%;
		margin-bottom: 6px;
		border-collapse: collapse;
	}

	th {
		padding: 0 4px 4px;
		text-align: left;
		font-size: var(--fs-md);
		font-weight: normal;
		color: var(--muted);
	}

	td {
		padding: 2px 4px;
	}

	.actions {
		width: 1%;
	}
</style>
