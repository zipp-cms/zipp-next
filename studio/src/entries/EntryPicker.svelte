<script lang="ts">
	import { tick, untrack } from 'svelte';
	import { type Schema } from '../core/schema';
	import type { EntryRef } from './api';
	import Flyout from 'zipp-ui/Flyout.svelte';
	import SearchInput from 'zipp-ui/SearchInput.svelte';
	import { listKey } from 'zipp-ui/listKeys';
	import { componentName } from '../core/schema';
	import { chooseLabel } from './picker';
	import { EntryPickerState } from './entryPicker.svelte';
	import SectionNav from './SectionNav.svelte';
	import PickerRow from './PickerRow.svelte';
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

	const picker = new EntryPickerState({
		schema: () => schema,
		types: () => types,
		sections: () => sections,
		chosen: () => chosen,
		multiple: () => multiple,
		max: () => max,
		onpick: e => onpick(e),
		onclose: () => onclose(),
	});
	untrack(() => picker.load());

	let top = $state<HTMLElement>();
	let list = $state<HTMLElement>();
	let more = $state<HTMLElement>();

	const title = $derived(chooseLabel(schema, types));

	// typing goes into the search at once; not on touch screens, where
	// the keyboard would cover the list
	$effect(() => {
		if (!matchMedia('(hover: hover)').matches) return;
		tick().then(() => top?.querySelector('input')?.focus());
	});

	// the next part of a long section when its end scrolls into view
	$effect(() => {
		if (!more) return;
		const seen = new IntersectionObserver(all => {
			if (all.some(e => e.isIntersecting)) picker.loadMore();
		});
		seen.observe(more);
		return () => seen.disconnect();
	});

	/** Enter in the search picks the first match that can be. */
	function onInputKey(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			e.preventDefault();
			// an empty one picks nothing, the input has the focus at once
			if (!picker.searching) return;
			const first = picker.found.find(picker.canPick);
			if (first) picker.pick(first);
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
		const row = picker.group?.rows.find(r => r.e.id === id);
		const input = top?.querySelector('input') ?? undefined;
		const shut = picker.closed.has(id);

		let to: HTMLElement | undefined;
		switch (e.key) {
			case 'ArrowUp':
				to = i > 0 ? all[i - 1] : input;
				break;
			case 'ArrowRight':
				if (!picker.searching && row?.parent && shut) picker.toggle(id);
				break;
			case 'ArrowLeft':
				if (picker.searching || !row) break;
				if (row.parent && !shut) picker.toggle(id);
				else
					to = all.find(b => b.dataset.id === String(row.e.parentId));
				break;
			default: {
				const n = listKey(e.key, i, all.length);
				if (n !== undefined) {
					to = all[n];
					break;
				}
				if (e.key.length === 1 && !e.ctrlKey && !e.metaKey)
					input?.focus();
				return;
			}
		}
		e.preventDefault();
		to?.focus();
	}
</script>

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
	<div class="picker" class:flat={picker.groups.length < 2}>
		{#if picker.groups.length > 1}
			<SectionNav
				nav={picker.nav}
				current={picker.searching ? null : picker.current}
				onselect={h => picker.select(h)}
			/>
		{/if}

		<div class="right">
			<div class="search" bind:this={top}>
				<SearchInput
					bind:value={picker.search}
					placeholder={t('entries.searchTypes', {
						types: types
							.map(ty => componentName(schema, ty))
							.join(', '),
					})}
					onkeydown={onInputKey}
				/>
			</div>
			<!-- the keys of its buttons, see onListKey -->
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div class="card entries" bind:this={list} onkeydown={onListKey}>
				{#if !picker.loaded}
					<p class="empty">{t('common.loading')}</p>
				{:else if picker.error}
					<p class="error">{t('entries.didntLoad')} {picker.error}</p>
				{:else if picker.searching}
					{#each picker.found as r (r.e.id)}
						<PickerRow
							{picker}
							{schema}
							{r}
							inTree={false}
							{multiple}
							{types}
						/>
					{:else}
						<p class="empty">
							{picker.finder.loading
								? t('common.loading')
								: t('ui.nothingFound')}
						</p>
					{/each}
					{#if picker.finder.error}
						<p class="error">{picker.finder.error}</p>
					{:else if picker.foundMore}
						<p class="note">
							{t('entries.firstFound', {
								n: picker.found.length,
							})}
						</p>
					{/if}
				{:else if picker.group}
					<div class="title">{picker.group.section.name}</div>
					{#each picker.rows as r (r.e.id)}
						<PickerRow
							{picker}
							{schema}
							{r}
							inTree={picker.tree}
							{multiple}
							{types}
						/>
					{/each}
					{#if picker.more.error}
						<p class="error">{picker.more.error}</p>
					{:else if picker.hasMore}
						<button
							type="button"
							class="more"
							bind:this={more}
							disabled={picker.more.loading}
							onclick={() => picker.loadMore()}
						>
							{picker.more.loading
								? t('common.loading')
								: t('entries.loadMore')}
						</button>
					{/if}
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

	.search {
		margin-bottom: 0.75rem;
	}

	.entries {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
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

	.more {
		align-self: center;
		margin: 0.5rem 0;
	}

	.note {
		padding: 0.5rem 0.625rem;
	}
</style>
