<script lang="ts">
	import type { Snippet } from 'svelte';
	import { untrack } from 'svelte';
	import { type Schema } from '../core/schema';
	import type { EntryRef } from './api';
	import { site } from '../core/router.svelte';
	import AddButton from 'zipp-ui/AddButton.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import Popover from 'zipp-ui/Popover.svelte';
	import SearchInput from 'zipp-ui/SearchInput.svelte';
	import { componentName } from '../core/schema';
	import {
		groupChoices,
		initiallyClosed,
		listChoices,
		openRows,
		searchRows,
		type Choice,
		type Row,
	} from './picker';

	/**
	 * Chooses entries for relations and links: a button that opens the
	 * sections of the types, a structure as its tree, with a search over
	 * all of them. Chosen entries and other types show but can't be
	 * picked. With `multiple` it stays open to choose more.
	 */
	let {
		schema,
		types,
		chosen = [],
		multiple = false,
		onpick,
		trigger,
	}: {
		schema: Schema;
		/** the entry types to choose from */
		types: string[];
		chosen?: number[];
		multiple?: boolean;
		onpick: (e: EntryRef) => void;
		/** what opens it instead of "Choose …", like the chosen entry */
		trigger?: Snippet<[{ toggle: () => void; open: boolean }]>;
	} = $props();

	// the prop's name is the Popover's snippet's too
	const own = $derived(trigger);
	let open = $state(false);
	let search = $state('');
	let entries = $state<Choice[]>([]);
	let loading = $state(false);
	let failed = $state(false);
	let closed = $state(new Set<number>());
	let results = $state<HTMLElement>();

	const typeName = (t: string) => componentName(schema, t);
	// "Choose Seite" with one type, "Choose entry" with several
	const what = $derived(types.length === 1 ? typeName(types[0]) : 'entry');

	const groups = $derived(groupChoices(schema, entries, types));
	const searching = $derived(search.trim() !== '');
	const found = $derived(searching ? searchRows(groups, search) : []);

	// all of them once per open, the search filters here
	$effect(() => {
		if (!open) return;
		loading = true;
		failed = false;
		listChoices(types, site.current)
			.then(r => {
				entries = r.entries;
				closed = initiallyClosed(
					groupChoices(schema, r.entries, types),
					untrack(() => chosen),
				);
			})
			.catch(() => (failed = true))
			.finally(() => (loading = false));
	});

	const canPick = (r: Row) => r.pickable && !chosen.includes(r.e.id);

	function pick(r: Row) {
		if (!canPick(r)) return;
		onpick(r.e);
		if (!multiple) open = false;
	}

	function toggle(id: number) {
		const next = new Set(closed);
		if (!next.delete(id)) next.add(id);
		closed = next;
	}

	/** Enter in the search picks the first one that can be. */
	function onInputKey(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			e.preventDefault();
			const rows = searching
				? found
				: groups.flatMap(g => openRows(g.rows, closed));
			const first = rows.find(canPick);
			if (first) pick(first);
		} else if (e.key === 'ArrowDown') {
			e.preventDefault();
			items()[0]?.focus();
		}
	}

	const items = () => [
		...(results?.querySelectorAll<HTMLElement>('.popup-item') ?? []),
	];

	/**
	 * The arrow keys move between the entries, up from the first back to
	 * the search; right and left open and close a tree, left goes up to
	 * the parent. Typing goes on in the search.
	 */
	function onListKey(e: KeyboardEvent) {
		const all = items();
		const at = e.target as HTMLElement;
		const i = all.indexOf(at);
		if (i < 0) return;
		const id = Number(at.dataset.id);
		const row = groups.flatMap(g => g.rows).find(r => r.e.id === id);

		let to: HTMLElement | undefined;
		switch (e.key) {
			case 'ArrowDown':
				to = all[Math.min(i + 1, all.length - 1)];
				break;
			case 'ArrowUp':
				to =
					i > 0
						? all[i - 1]
						: ((e.currentTarget as HTMLElement)
								.closest('.popup')
								?.querySelector('input') ?? undefined);
				break;
			case 'Home':
				to = all[0];
				break;
			case 'End':
				to = all[all.length - 1];
				break;
			case 'ArrowRight':
				if (!searching && row?.parent && closed.has(id)) toggle(id);
				break;
			case 'ArrowLeft':
				if (searching || !row) break;
				if (row.parent && !closed.has(id)) toggle(id);
				else
					to = all.find(b => b.dataset.id === String(row.e.parentId));
				break;
			default:
				if (e.key.length === 1 && !e.ctrlKey && !e.metaKey) {
					(e.currentTarget as HTMLElement)
						.closest('.popup')
						?.querySelector('input')
						?.focus();
				}
				return;
		}
		e.preventDefault();
		to?.focus();
	}
</script>

{#snippet line(r: Row, tree: boolean)}
	{@const isChosen = chosen.includes(r.e.id)}
	{@const shut = closed.has(r.e.id)}
	<div class="line" style:padding-left="{tree ? r.depth * 1.125 : 0}rem">
		{#if tree && r.parent}
			<button
				type="button"
				class="icon caret"
				class:shut
				tabindex="-1"
				aria-expanded={!shut}
				title={shut
					? 'Show the entries below'
					: 'Hide the entries below'}
				onclick={() => toggle(r.e.id)}
			>
				<Icon name="chevron-down" size={14} />
			</button>
		{:else if tree}
			<span class="caret"></span>
		{/if}
		<button
			type="button"
			class="popup-item"
			class:chosen={isChosen}
			aria-disabled={!canPick(r)}
			data-id={r.e.id}
			title={!r.pickable
				? `A ${typeName(r.e.type)} can't be chosen here`
				: isChosen
					? 'Already chosen'
					: undefined}
			onclick={() => pick(r)}
		>
			<span class="truncate">
				{#if !tree && r.path.length}
					<span class="path">{r.path.join(' › ')} ›</span>
				{/if}
				{r.e.title}
			</span>
			{#if isChosen}
				<Icon name="check" size={14} />
			{/if}
			{#if types.length > 1 || !r.pickable}
				<span class="note type">{typeName(r.e.type)}</span>
			{/if}
		</button>
	</div>
{/snippet}

<Popover bind:open width="24rem" label="Choose {what}">
	{#snippet trigger({ toggle, open })}
		{@const start = () => ((search = ''), toggle())}
		{#if own}
			{@render own({ toggle: start, open })}
		{:else}
			<AddButton onclick={start} expanded={open}>Choose {what}</AddButton>
		{/if}
	{/snippet}
	<SearchInput
		bind:value={search}
		placeholder="Search {types.map(typeName).join(', ')}"
		onkeydown={onInputKey}
	/>
	<!-- the keys of its buttons, see onListKey -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="results"
		class:tall={entries.length > 8}
		bind:this={results}
		onkeydown={onListKey}
	>
		{#if loading && !entries.length}
			<p class="empty">Loading…</p>
		{:else if failed}
			<p class="error">The entries didn't load.</p>
		{:else if searching}
			{#each found as r (r.e.id)}
				{@render line(r, false)}
			{:else}
				<p class="empty">Nothing found</p>
			{/each}
		{:else}
			{#each groups as g (g.section.handle)}
				{@const tree = g.section.kind === 'structure'}
				{#if groups.length > 1}
					<p class="section note">{g.section.name}</p>
				{/if}
				{#each tree ? openRows(g.rows, closed) : g.rows as r (r.e.id)}
					{@render line(r, tree)}
				{/each}
			{:else}
				<p class="empty">No entries yet</p>
			{/each}
		{/if}
	</div>
	{#if multiple}
		<div class="foot">
			<button type="button" onclick={() => (open = false)}>Done</button>
		</div>
	{/if}
</Popover>

<style lang="scss">
	.results {
		display: flex;
		flex-direction: column;
		gap: 2px;
		max-height: 20rem;
		margin-top: 0.25rem;
		overflow-y: auto;

		// as high while searching, so the box doesn't jump
		&.tall {
			height: 20rem;
		}
	}

	.line {
		display: flex;
		align-items: center;
		flex-shrink: 0;

		.popup-item {
			flex: 1;
			min-width: 0;
		}
	}

	// the open and close of a tree, room for it where there is nothing below
	.caret {
		flex-shrink: 0;
		width: 1.375rem;

		:global(.icon) {
			transition: transform 0.15s;
		}

		&.shut :global(.icon) {
			transform: rotate(-90deg);
		}
	}

	.popup-item[aria-disabled='true'] {
		color: var(--muted);
		cursor: default;

		@include hover {
			&:hover {
				background: none;
			}
		}
	}

	.popup-item.chosen {
		color: var(--text);
	}

	.path {
		color: var(--muted);
		font-weight: 400;
	}

	.type {
		margin-left: auto;
		flex-shrink: 0;
	}

	.section {
		flex-shrink: 0;
		margin: 0.5rem 0 0.125rem;
		padding: 0 0.625rem;
		font-weight: 600;

		&:first-child {
			margin-top: 0.25rem;
		}
	}

	.foot {
		display: flex;
		justify-content: flex-end;
		padding-top: 0.375rem;
		border-top: 1px solid var(--line);
	}
</style>
