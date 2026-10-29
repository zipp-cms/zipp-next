<script lang="ts">
	import {
		type EntryRef,
		type Field,
		type Refs,
		type Schema,
		type Values,
	} from '../api';
	import Icon from '../lib/Icon.svelte';
	import EntrySearch from '../lib/EntrySearch.svelte';
	import { openEntry } from '../lib/flyout.svelte';

	let {
		schema,
		field,
		values,
		refs,
	}: { schema: Schema; field: Field; values: Values; refs: Refs } = $props();

	const types = $derived(field.kind === 'relation' ? field.types : []);
	const max = $derived(field.kind === 'relation' ? field.max : null);
	const ids = $derived<number[]>(values[field.handle] ?? []);
	const full = $derived(max !== null && ids.length >= max);

	function typeName(t: string) {
		return schema.components.find(c => c.handle === t)?.name ?? t;
	}

	function add(e: EntryRef) {
		refs[e.id] = e;
		values[field.handle] = [...ids, e.id];
	}

	function remove(id: number) {
		values[field.handle] = ids.filter(i => i !== id);
	}
</script>

<div class="relation">
	{#each ids as id (id)}
		<div class="chip">
			<button
				type="button"
				class="open"
				title="Open in the flyout"
				onclick={() => openEntry(id)}
			>
				<span class="title">{refs[id]?.title ?? `#${id}`}</span>
				{#if refs[id]}<span class="muted">
						{typeName(refs[id].type)}
					</span>{/if}
			</button>
			<button
				type="button"
				class="icon"
				onclick={() => remove(id)}
				title="Remove from the field"
			>
				<Icon name="close" size={15} />
			</button>
		</div>
	{/each}

	{#if !full}
		<EntrySearch
			{schema}
			{types}
			exclude={ids}
			multiple={max !== 1}
			onpick={add}
		/>
	{/if}
</div>

<style lang="scss">
	.relation {
		display: flex;
		flex-direction: column;
		align-items: stretch;
		gap: 6px;

		> :global(.picker) {
			align-self: flex-start;
		}
	}

	// a row like an input, with the remove action on the right
	.chip {
		display: flex;
		align-items: center;
		gap: 10px;
		min-height: 36px;
		padding: 3px 4px 3px 12px;
		border: 1px solid var(--border);
		border-radius: 5px;
		background: var(--surface);
	}

	// the title opens the entry, like Craft's element chips
	.open {
		display: flex;
		align-items: baseline;
		gap: 10px;
		flex: 1;
		min-width: 0;
		padding: 4px 0;
		border: none;
		background: none;
		font-weight: 500;
		text-align: left;

		&:hover:not(:disabled) {
			background: none;
			color: var(--accent);
		}
	}

	.title {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.muted {
		color: var(--muted);
		font-size: 13px;
	}
</style>
