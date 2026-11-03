<script lang="ts" module>
	/** The section the picker showed last, it opens there again. */
	let lastSection: string | null = null;
</script>

<script lang="ts">
	import { tick, untrack } from 'svelte';
	import { type Schema } from '../core/schema';
	import type { EntryRef } from './api';
	import { site } from '../core/router.svelte';
	import Flyout from 'zipp-ui/Flyout.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import SearchInput from 'zipp-ui/SearchInput.svelte';
	import { componentName } from '../core/schema';
	import { date } from '../core/format';
	import { isFull } from '../core/list';
	import {
		choiceState,
		chooseLabel,
		groupChoices,
		initiallyClosed,
		openRows,
		pickable,
		pickerNav,
		searchRows,
		startSection,
		type Row,
	} from './picker';
	import { listChoices, type Choice } from './api';
	import { t } from '../i18n/index.svelte';

	/**
	 * Chooses entries for relations and links in a flyout, like the asset
	 * picker: the sections of the types on the left as the nav groups
	 * them, the entries of one on the right (a structure as its tree, a
	 * channel newest first), a search over all of them. Other types show
	 * but can't be picked. One pick closes it; with `multiple` a click
	 * picks or takes back, as many as `max`.
	 */
	let {
		schema,
		types,
		sections = [],
		chosen = [],
		multiple = false,
		max = null,
		onpick,
		onclose,
	}: {
		schema: Schema;
		/** the entry types to choose from */
		types: string[];
		/** only entries of these sections, none: all */
		sections?: string[];
		chosen?: number[];
		multiple?: boolean;
		max?: number | null;
		/** with `multiple` also a chosen one, to take it back */
		onpick: (e: EntryRef) => void;
		onclose: () => void;
	} = $props();

	let search = $state('');
	let entries = $state<Choice[]>([]);
	let loaded = $state(false);
	let failed = $state(false);
	let closed = $state(new Set<number>());
	let current = $state<string | null>(null);
	let top = $state<HTMLElement>();
	let list = $state<HTMLElement>();

	const typeName = (ty: string) => componentName(schema, ty);
	const title = $derived(chooseLabel(schema, types));
	const groups = $derived(groupChoices(schema, entries, types));
	const nav = $derived(pickerNav(schema, groups));
	const group = $derived(groups.find(g => g.section.handle === current));
	const searching = $derived(search.trim() !== '');
	const found = $derived(searching ? searchRows(groups, search) : []);
	const full = $derived(isFull(chosen.length, max));
	const tree = $derived(group?.section.kind === 'structure');
	const rows = $derived(
		!group ? [] : tree ? openRows(group.rows, closed) : group.rows,
	);

	// all of them once, the search filters here
	listChoices(
		untrack(() => types),
		site.current,
		untrack(() => sections),
	)
		.then(r => {
			entries = r.entries;
			const all = groupChoices(schema, r.entries, types);
			closed = initiallyClosed(all, chosen);
			current = startSection(all, chosen, lastSection);
		})
		.catch(() => (failed = true))
		.finally(() => (loaded = true));

	// typing goes into the search at once; not on touch screens, where
	// the keyboard would cover the list
	$effect(() => {
		if (!matchMedia('(hover: hover)').matches) return;
		tick().then(() => top?.querySelector('input')?.focus());
	});

	function select(handle: string) {
		lastSection = handle;
		current = handle;
		search = '';
	}

	const isChosen = (r: Row) => chosen.includes(r.e.id);
	const canPick = (r: Row) => r.pickable && (isChosen(r) ? multiple : !full);

	function pick(r: Row) {
		if (!canPick(r)) return;
		lastSection = r.e.section;
		onpick(r.e);
		if (!multiple) onclose();
	}

	/** Why a row can't be picked, as its title. */
	function why(r: Row): string | undefined {
		if (!r.pickable)
			return t('entries.cantChoose', { type: typeName(r.e.type) });
		if (isChosen(r) && !multiple) return t('entries.alreadyChosen');
		if (!isChosen(r) && full) return t('entries.fieldFull');
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
			const first = (searching ? found : rows).find(canPick);
			if (first) pick(first);
		} else if (e.key === 'ArrowDown') {
			e.preventDefault();
			items()[0]?.focus();
		}
	}

	const items = () => [
		...(list?.querySelectorAll<HTMLElement>('.pick') ?? []),
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
		const row = group?.rows.find(r => r.e.id === id);
		const input = top?.querySelector('input') ?? undefined;

		let to: HTMLElement | undefined;
		switch (e.key) {
			case 'ArrowDown':
				to = all[Math.min(i + 1, all.length - 1)];
				break;
			case 'ArrowUp':
				to = i > 0 ? all[i - 1] : input;
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
				if (e.key.length === 1 && !e.ctrlKey && !e.metaKey)
					input?.focus();
				return;
		}
		e.preventDefault();
		to?.focus();
	}
</script>

{#snippet line(r: Row, inTree: boolean)}
	{@const on = isChosen(r)}
	{@const shut = closed.has(r.e.id)}
	{@const status = choiceState(r.e)}
	<div class="line" style:--depth={inTree ? r.depth : 0}>
		{#if inTree && r.parent}
			<button
				type="button"
				class="icon caret"
				tabindex="-1"
				aria-expanded={!shut}
				aria-label={shut
					? t('entries.showBelow')
					: t('entries.hideBelow')}
				title={shut ? t('entries.showBelow') : t('entries.hideBelow')}
				onclick={() => toggle(r.e.id)}
			>
				<Icon
					name={shut ? 'chevron-right' : 'chevron-down'}
					size={14}
				/>
			</button>
		{:else if inTree}
			<span class="caret"></span>
		{/if}
		<button
			type="button"
			class="pick"
			class:chosen={on}
			aria-disabled={!canPick(r)}
			aria-pressed={multiple ? on : undefined}
			data-id={r.e.id}
			title={why(r)}
			onclick={() => pick(r)}
		>
			<span class="dot {status.state}" title={status.label}></span>
			<span class="truncate name">
				{#if !inTree && r.path.length}
					<span class="path">{r.path.join(' › ')} ›</span>
				{/if}
				{r.e.title}
			</span>
			{#if on}<Icon name="check" size={14} />{/if}
			{#if r.e.draft}
				<span
					class="badge changes"
					title={t('entries.unpublishedChanges')}
				>
					{t('common.draft')}
				</span>
			{/if}
			<span class="meta">
				{#if searching && groups.length > 1}
					{groups.find(g => g.section.handle === r.e.section)?.section
						.name}
				{:else if types.length > 1 || !r.pickable}
					{typeName(r.e.type)}
				{/if}
				{#if !inTree && !searching && r.e.postDate}
					<span class="date">{date(r.e.postDate)}</span>
				{/if}
			</span>
		</button>
	</div>
{/snippet}

<Flyout {title} {onclose}>
	{#snippet actions()}
		{#if multiple}
			<span class="note count">
				{max
					? t('entries.chosenOf', { n: chosen.length, max })
					: t('entries.chosen', { n: chosen.length })}
			</span>
		{/if}
		<button type="button" class="primary" onclick={onclose}>
			{t('ui.done')}
		</button>
	{/snippet}
	<div class="picker" class:flat={groups.length < 2}>
		{#if groups.length > 1}
			<nav class="sections" aria-label={t('entries.sections')}>
				{#each nav as part, i (i)}
					{#if part.name && nav.length > 1}
						<div class="head">{part.name}</div>
					{/if}
					{#each part.groups as g (g.section.handle)}
						<button
							type="button"
							class="section"
							class:active={!searching &&
								current === g.section.handle}
							aria-current={!searching &&
							current === g.section.handle
								? 'true'
								: undefined}
							onclick={() => select(g.section.handle)}
						>
							<Icon name={g.section.kind} size={15} />
							<span class="truncate">{g.section.name}</span>
							<span class="n">{pickable(g)}</span>
						</button>
					{/each}
				{/each}
			</nav>
			<!-- a phone has no room for the nav beside the list -->
			<select
				class="section-select"
				aria-label={t('entries.sections')}
				value={current}
				onchange={e => select(e.currentTarget.value)}
			>
				{#each nav as part, i (i)}
					{#if part.name && nav.length > 1}
						<optgroup label={part.name}>
							{#each part.groups as g (g.section.handle)}
								<option value={g.section.handle}>
									{g.section.name}
								</option>
							{/each}
						</optgroup>
					{:else}
						{#each part.groups as g (g.section.handle)}
							<option value={g.section.handle}>
								{g.section.name}
							</option>
						{/each}
					{/if}
				{/each}
			</select>
		{/if}

		<div class="right">
			<div class="search" bind:this={top}>
				<SearchInput
					bind:value={search}
					placeholder={t('entries.searchTypes', {
						types: types.map(typeName).join(', '),
					})}
					onkeydown={onInputKey}
				/>
			</div>
			<!-- the keys of its buttons, see onListKey -->
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div class="card entries" bind:this={list} onkeydown={onListKey}>
				{#if !loaded}
					<p class="empty">{t('common.loading')}</p>
				{:else if failed}
					<p class="error">{t('entries.didntLoad')}</p>
				{:else if searching}
					{#each found as r (r.e.id)}
						{@render line(r, false)}
					{:else}
						<p class="empty">{t('ui.nothingFound')}</p>
					{/each}
				{:else if group}
					<div class="title">{group.section.name}</div>
					{#each rows as r (r.e.id)}
						{@render line(r, tree)}
					{/each}
				{:else}
					<p class="empty">{t('entries.noneYet')}</p>
				{/if}
			</div>
		</div>
	</div>
</Flyout>

<style lang="scss">
	.picker {
		display: grid;
		grid-template-columns: 12.5rem minmax(0, 1fr);
		gap: 1rem;
		align-items: start;
		margin-top: 1rem;

		&.flat {
			grid-template-columns: minmax(0, 1fr);
		}

		@include max-tablet {
			grid-template-columns: minmax(0, 1fr);
		}
	}

	.count {
		align-self: center;
		white-space: nowrap;
	}

	// like the folders of the asset library
	.sections {
		position: sticky;
		top: 1rem;
		display: flex;
		flex-direction: column;
		gap: 0.125rem;

		@include max-tablet {
			display: none;
		}
	}

	.head {
		margin: 0.5rem 0 0.25rem;
		padding-left: 0.5rem;
		color: var(--muted);
		font-size: var(--fs-xs);
		font-weight: 600;
		letter-spacing: 0.04em;
		text-transform: uppercase;

		&:first-child {
			margin-top: 0;
		}
	}

	.section {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		min-height: 2rem;
		padding: 0.375rem 0.5rem;
		border: none;
		border-radius: var(--radius);
		background: none;
		color: var(--text-2);
		font-weight: 500;
		text-align: left;

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

		.n {
			margin-left: auto;
			color: var(--muted);
			font-size: var(--fs-xs);
			font-weight: 400;
		}
	}

	.section-select {
		display: none;

		@include max-tablet {
			display: block;
		}
	}

	.search {
		margin-bottom: 0.75rem;
	}

	.entries {
		display: flex;
		flex-direction: column;
		gap: 2px;
		min-height: 15rem;
		padding: 0.375rem;
	}

	.title {
		padding: 0.25rem 0.625rem 0.375rem;
		color: var(--muted);
		font-size: var(--fs-xs);
		font-weight: 600;
		letter-spacing: 0.04em;
		text-transform: uppercase;
	}

	.line {
		--depth: 0;
		display: flex;
		align-items: center;
		flex-shrink: 0;
		padding-left: calc(var(--depth) * 1.3125rem);
	}

	// the open and close of a tree, room for it where there is nothing below
	.caret {
		display: grid;
		flex-shrink: 0;
		place-items: center;
		width: 1.375rem;
		height: 1.375rem;
	}

	.pick {
		display: flex;
		flex: 1;
		align-items: center;
		gap: 0.5rem;
		min-width: 0;
		min-height: 2.25rem;
		padding: 0.375rem 0.625rem;
		border: none;
		border-radius: var(--radius);
		background: none;
		color: var(--text);
		font-weight: 500;
		text-align: left;

		@include hover {
			&:hover {
				background: var(--hover);
			}
		}

		&.chosen {
			background: var(--accent-soft);

			:global(.icon) {
				color: var(--accent);
			}
		}

		&[aria-disabled='true'] {
			color: var(--muted);
			cursor: default;

			@include hover {
				&:hover {
					background: none;
				}
			}
		}

		// a chosen one stays marked when it can't be taken back
		&.chosen[aria-disabled='true'] {
			color: var(--text);

			@include hover {
				&:hover {
					background: var(--accent-soft);
				}
			}
		}
	}

	.name {
		min-width: 0;
	}

	.path {
		color: var(--muted);
		font-weight: 400;
	}

	// published and live, published but not now, not published
	.dot {
		flex-shrink: 0;
		width: 0.5rem;
		height: 0.5rem;
		border-radius: 50%;
		background: var(--subtle);

		&.live {
			background: var(--ok);
		}

		&.waiting {
			background: var(--changed);
		}
	}

	.meta {
		display: flex;
		gap: 0.75rem;
		flex-shrink: 0;
		margin-left: auto;
		color: var(--subtle);
		font-size: var(--fs-md);
		font-weight: 400;
	}

	@include max-tablet {
		.date {
			display: none;
		}
	}
</style>
