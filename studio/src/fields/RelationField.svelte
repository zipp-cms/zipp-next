<script lang="ts">
	import {
		listEntries,
		type EntryRef,
		type Field,
		type Refs,
		type Schema,
		type Values,
	} from '../api';

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
		listEntries({ search: q, limit: '20' }).then(r => {
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
				×
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
		gap: 0.4rem;
	}

	.chip {
		display: flex;
		gap: 0.5rem;
		align-items: center;
		padding: 0.3rem 0.4rem 0.3rem 0.7rem;
		border: 1px solid var(--line);
		border-radius: 0.3rem;

		button {
			margin-left: auto;
		}
	}

	.search {
		position: relative;
	}

	.results {
		position: absolute;
		z-index: 2;
		left: 0;
		right: 0;
		background: white;
		border: 1px solid var(--line);
		border-radius: 0.3rem;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);

		button {
			display: flex;
			gap: 0.5rem;
			width: 100%;
			border: none;
			text-align: left;
		}
	}

	.muted {
		color: var(--muted);
		font-size: 0.85rem;
	}
</style>
