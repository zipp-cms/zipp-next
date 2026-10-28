<script lang="ts">
	import { listEntries, type EntryRef, type Schema } from '../api';
	import { site } from '../route.svelte';

	/** An input that finds entries by title, for relations and links. */
	let {
		schema,
		types,
		exclude = [],
		onpick,
	}: {
		schema: Schema;
		/** the entry types to find */
		types: string[];
		exclude?: number[];
		onpick: (e: EntryRef) => void;
	} = $props();

	let search = $state('');
	let found = $state<EntryRef[]>([]);

	$effect(() => {
		if (!search) return void (found = []);
		const q = search;
		listEntries({ search: q, limit: '20', site: site.current }).then(r => {
			if (q !== search) return;
			found = r.entries.filter(
				e => types.includes(e.type) && !exclude.includes(e.id),
			);
		});
	});

	function typeName(t: string) {
		return schema.components.find(c => c.handle === t)?.name ?? t;
	}

	function pick(e: EntryRef) {
		search = '';
		onpick(e);
	}
</script>

<div class="search">
	<input
		placeholder="Find {types.map(typeName).join(' or ')}…"
		bind:value={search}
	/>
	{#if found.length}
		<div class="results">
			{#each found as e (e.id)}
				<button onclick={() => pick(e)}>
					{e.title}
					<span class="muted">{typeName(e.type)}</span>
				</button>
			{/each}
		</div>
	{/if}
</div>

<style lang="scss">
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
