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
		detail,
		isOpen,
		ontoggle,
		widths,
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
		/** what a row shows under it while it is open, a click opens it */
		detail?: Snippet<[T]>;
		isOpen?: (row: T) => boolean;
		ontoggle?: (row: T) => void;
		/** of the columns, so tables one under the other line up */
		widths?: string[];
	} = $props();

	function click(e: MouseEvent, r: T) {
		if ((e.target as Element).closest('a, button, input, select')) return;
		if (ontoggle) return ontoggle(r);
		if (!href) return;
		const url = href(r);
		if (e.metaKey || e.ctrlKey) return void window.open(url, '_blank');
		const row = e.currentTarget as HTMLElement;
		const link = row.querySelector<HTMLAnchorElement>(
			`a[href="${CSS.escape(url)}"]`,
		);
		if (link) link.click();
		else location.assign(url);
	}
</script>

<table class:links={!!href || !!ontoggle} class:fixed={!!widths}>
	{#if widths}
		<colgroup>
			{#each widths as w, i (i)}<col style:width={w} />{/each}
		</colgroup>
	{/if}
	{#if columns.length}
		<thead>
			<tr>
				{#each columns as c, i (i)}<th>{c}</th>{/each}
			</tr>
		</thead>
	{/if}
	<tbody>
		{#each rows as r, i (key(r))}
			<!-- a row that opens takes the focus, Enter or Space opens it -->
			<tr
				id={id?.(r)}
				aria-expanded={ontoggle ? !!isOpen?.(r) : undefined}
				tabindex={ontoggle ? 0 : undefined}
				onclick={e => click(e, r)}
				onkeydown={e => {
					if (!ontoggle || e.target !== e.currentTarget) return;
					if (e.key !== 'Enter' && e.key !== ' ') return;
					e.preventDefault();
					ontoggle(r);
				}}
			>
				{@render row(r, i)}
			</tr>
			{#if detail && isOpen?.(r)}
				<tr class="detail">
					<td colspan={columns.length || 1}>{@render detail(r)}</td>
				</tr>
			{/if}
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

		&.fixed {
			table-layout: fixed;
		}
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

	// the open row and what it shows, as one
	tbody tr[aria-expanded='true'],
	.detail {
		background: var(--surface-2);
	}

	.detail :global(td) {
		border-top: none;
		padding: 0.25rem 0.5rem 0.75rem;
	}

	.empty {
		color: var(--muted);
	}

	// a row that opens what it shows
	.links tbody tr:not(.detail) {
		cursor: pointer;
	}

	@include hover {
		.links tbody tr:not(.detail):hover {
			background: var(--hover);
		}
	}
</style>
