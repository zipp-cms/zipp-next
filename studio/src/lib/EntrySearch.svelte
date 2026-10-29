<script lang="ts">
	import { listEntries, type EntryRef, type Schema } from '../api';
	import { site } from '../route.svelte';
	import Icon from './Icon.svelte';

	/**
	 * Chooses entries for relations and links: a button that opens the
	 * entries of the types, newest or in their order, with a search.
	 * With `multiple` it stays open to choose more.
	 */
	let {
		schema,
		types,
		exclude = [],
		multiple = false,
		onpick,
	}: {
		schema: Schema;
		/** the entry types to choose from */
		types: string[];
		exclude?: number[];
		multiple?: boolean;
		onpick: (e: EntryRef) => void;
	} = $props();

	let open = $state(false);
	let search = $state('');
	let found = $state<EntryRef[]>([]);
	let loading = $state(false);
	let input = $state<HTMLInputElement>();

	const typeName = (t: string) =>
		schema.components.find(c => c.handle === t)?.name ?? t;
	// "Choose Seite" with one type, "Choose entry" with several
	const what = $derived(types.length === 1 ? typeName(types[0]) : 'entry');

	$effect(() => {
		if (!open) return;
		const q = search;
		loading = true;
		const params: Record<string, string> = {
			type: types.join(','),
			limit: '30',
			site: site.current,
		};
		if (q) params.search = q;
		listEntries(params)
			.then(r => {
				if (q === search) found = r.entries;
			})
			.finally(() => (loading = false));
	});

	const shown = $derived(found.filter(e => !exclude.includes(e.id)));

	function toggle() {
		open = !open;
		search = '';
		if (open) setTimeout(() => input?.focus());
	}

	function pick(e: EntryRef) {
		onpick(e);
		if (!multiple) open = false;
	}

	function onkeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') open = false;
		if (e.key === 'Enter' && shown.length) {
			e.preventDefault();
			pick(shown[0]);
		}
	}
</script>

<div class="picker">
	<button type="button" class="choose" onclick={toggle} aria-expanded={open}>
		<Icon name="add" size={14} />
		Choose {what}
	</button>
	{#if open}
		<div class="popup" role="dialog" aria-label="Choose {what}">
			<span class="field">
				<Icon name="search" size={14} />
				<input
					bind:this={input}
					placeholder="Search {types.map(typeName).join(', ')}"
					bind:value={search}
					{onkeydown}
				/>
			</span>
			<div class="results">
				{#each shown as e (e.id)}
					<button type="button" onclick={() => pick(e)}>
						<span class="title">{e.title}</span>
						<span class="muted">{typeName(e.type)}</span>
					</button>
				{:else}
					<p class="muted empty">
						{loading ? 'Loading…' : 'Nothing found'}
					</p>
				{/each}
			</div>
			{#if multiple}
				<div class="foot">
					<button type="button" onclick={() => (open = false)}>
						Done
					</button>
				</div>
			{/if}
		</div>
	{/if}
</div>

<style lang="scss">
	.picker {
		position: relative;
	}

	.choose {
		border-style: dashed;
		color: var(--accent);
	}

	.popup {
		position: absolute;
		z-index: 20;
		top: 100%;
		left: 0;
		width: min(26rem, 90vw);
		margin-top: 4px;
		padding: 6px;
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: 8px;
		box-shadow: 0 8px 24px rgba(16, 24, 40, 0.12);
	}

	.field {
		position: relative;
		display: flex;
		align-items: center;
		color: var(--subtle);

		:global(.icon) {
			position: absolute;
			left: 9px;
		}

		input {
			padding-left: 30px;
		}
	}

	.results {
		display: flex;
		flex-direction: column;
		max-height: 18rem;
		margin-top: 6px;
		overflow-y: auto;

		button {
			display: flex;
			align-items: baseline;
			gap: 10px;
			width: 100%;
			border: none;
			font-weight: 400;
			text-align: left;
		}
	}

	.title {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.muted {
		margin-left: auto;
		flex-shrink: 0;
		color: var(--subtle);
		font-size: 12px;
	}

	.empty {
		margin: 6px 8px;
	}

	.foot {
		display: flex;
		justify-content: flex-end;
		padding-top: 6px;
		border-top: 1px solid var(--line);
	}
</style>
