<script lang="ts">
	import type { Field, Values } from '../api';
	import Icon from '../lib/Icon.svelte';
	import { moved } from '../lib/list';

	/** Rows of the columns of the code, like Craft's table field. */
	let { field, values }: { field: Field; values: Values } = $props();

	const columns = $derived(field.kind === 'table' ? field.columns : []);
	const rows = $derived<Values[]>(values[field.handle] ?? []);

	function set(next: Values[]) {
		values[field.handle] = next.length ? next : null;
	}

	function cell(i: number, column: string, text: string) {
		set(rows.map((r, j) => (j === i ? { ...r, [column]: text } : r)));
	}

	const move = (i: number, by: number) => set(moved(rows, i, by));
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
					<button
						type="button"
						class="link"
						disabled={i === 0}
						onclick={() => move(i, -1)}
					>
						↑
					</button>
					<button
						type="button"
						class="link"
						disabled={i === rows.length - 1}
						onclick={() => move(i, 1)}
					>
						↓
					</button>
					<button
						type="button"
						class="link"
						title="Remove the row"
						onclick={() => set(rows.filter((_, j) => j !== i))}
					>
						<Icon name="delete" />
					</button>
				</td>
			</tr>
		{/each}
	</tbody>
</table>
<button type="button" class="link add" onclick={() => set([...rows, {}])}>
	<Icon name="add" /> Add a row
</button>

<style>
	table {
		width: 100%;
		border-collapse: collapse;
	}

	th {
		padding: 0 4px 4px;
		text-align: left;
		font-size: 13px;
		font-weight: normal;
		color: var(--muted);
	}

	td {
		padding: 2px 4px;
	}

	.actions {
		width: 1%;
		white-space: nowrap;
	}

	.add {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		margin-top: 6px;
	}
</style>
