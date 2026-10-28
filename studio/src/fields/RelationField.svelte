<script lang="ts">
	import {
		listEntries,
		type EntryRef,
		type Field,
		type Refs,
		type Schema,
		type Values,
	} from '../api';
	import { site } from '../route.svelte';
	import Icon from '../lib/Icon.svelte';

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

	let search = $state('');
	let found = $state<EntryRef[]>([]);

	$effect(() => {
		if (!search) return void (found = []);
		const q = search;
		listEntries({ search: q, limit: '20', site: site.current }).then(r => {
			if (q !== search) return;
			found = r.entries.filter(
				e => types.includes(e.type) && !ids.includes(e.id),
			);
		});
	});

	function typeName(t: string) {
		return schema.components.find(c => c.handle === t)?.name ?? t;
	}

	function add(e: EntryRef) {
		refs[e.id] = e;
		values[field.handle] = [...ids, e.id];
		search = '';
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
		<div class="search">
			<input
				placeholder="Link {types.map(typeName).join(' or ')}…"
				bind:value={search}
			/>
			{#if found.length}
				<div class="results">
					{#each found as e (e.id)}
						<button onclick={() => add(e)}>
							{e.title}
							<span class="muted">{typeName(e.type)}</span>
						</button>
					{/each}
				</div>
			{/if}
		</div>
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

	.search {
		position: relative;
	}

	.results {
		position: absolute;
		z-index: 5;
		left: 0;
		right: 0;
		padding: 4px;
		background: white;
		box-shadow: var(--shadow);

		button {
			display: flex;
			gap: 10px;
			width: 100%;
			border: none;
			text-align: left;

			&:hover {
				background: var(--hover);
			}
		}
	}

	.muted {
		color: var(--muted);
		font-size: 13px;
	}
</style>
