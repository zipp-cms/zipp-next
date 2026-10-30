<script lang="ts" generics="T">
	import type { Snippet } from 'svelte';

	/**
	 * A table of the studio: users, the sections and types of Setup. The
	 * columns are the headings (none: no head), `row` the cells of a row.
	 * With `href` a click anywhere on a row opens it like its link to that
	 * href (the studio's router takes it), links and buttons in it still
	 * do their own.
	 */
	let {
		rows,
		key,
		columns = [],
		row,
		href,
		id,
		empty = 'Nothing yet',
	}: {
		rows: T[];
		key: (row: T) => string | number;
		columns?: string[];
		/** the `<td>`s of a row */
		row: Snippet<[T, number]>;
		href?: (row: T) => string;
		/** the id of a row, to link to it */
		id?: (row: T) => string;
		empty?: string;
	} = $props();

	function open(e: MouseEvent, r: T) {
		if (!href || (e.target as Element).closest('a, button, input, select'))
			return;
		const url = href(r);
		if (e.metaKey || e.ctrlKey) return void window.open(url, '_blank');
		const row = e.currentTarget as HTMLElement;
		const link = row.querySelector<HTMLAnchorElement>(`a[href="${url}"]`);
		if (link) link.click();
		else location.assign(url);
	}
</script>

<table class:links={!!href}>
	{#if columns.length}
		<thead>
			<tr>
				{#each columns as c, i (i)}<th>{c}</th>{/each}
			</tr>
		</thead>
	{/if}
	<tbody>
		{#each rows as r, i (key(r))}
			<tr id={id?.(r)} onclick={e => open(e, r)}>
				{@render row(r, i)}
			</tr>
		{:else}
			<tr>
				<td class="empty" colspan={columns.length || 1}>{empty}</td>
			</tr>
		{/each}
	</tbody>
</table>

<style lang="scss">
	@use '../vars' as *;

	table {
		width: 100%;
		border-collapse: collapse;
	}

	th,
	tbody :global(td) {
		padding: 0.375rem 0.5rem;
		text-align: left;
		vertical-align: top;
	}

	th {
		font-weight: 600;
	}

	tbody :global(td) {
		border-top: 1px solid var(--line);
	}

	tbody :global(code) {
		font-size: var(--fs-md);
		color: var(--muted);
	}

	// the row an anchor of the url points to, see router.svelte.ts
	tbody tr:global(.target) {
		background: var(--hover);
		transition: background 0.3s;
	}

	.empty {
		color: var(--muted);
	}

	// a row that opens what it shows
	.links tbody tr {
		cursor: pointer;
	}

	@include hover {
		.links tbody tr:hover {
			background: var(--hover);
		}
	}
</style>
