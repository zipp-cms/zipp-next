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
			<span>{refs[id]?.title ?? `#${id}`}</span>
			{#if refs[id]}<span class="muted">
					{typeName(refs[id].type)}
				</span>{/if}
			<button class="icon" onclick={() => remove(id)} title="Remove">
				<Icon name="delete" />
			</button>
		</div>
	{/each}

	{#if !full}
		<EntrySearch {schema} {types} exclude={ids} onpick={add} />
	{/if}
</div>

<style lang="scss">
	.relation {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	// like a filled input, with the remove action on the right
	.chip {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 5px 4px 5px 12px;
		border-radius: 2px;
		background: var(--input);

		button {
			margin-left: auto;
		}
	}

	.muted {
		color: var(--muted);
		font-size: 13px;
	}
</style>
