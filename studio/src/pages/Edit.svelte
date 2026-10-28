<script lang="ts">
	import {
		ApiError,
		deleteEntry,
		getEntry,
		saveEntry,
		type Refs,
		type Schema,
		type Values,
	} from '../api';
	import { untrack } from 'svelte';
	import { go } from '../route.svelte';
	import AutoText from '../fields/AutoText.svelte';
	import Fields from '../fields/Fields.svelte';

	let {
		schema,
		id,
		type,
	}: { schema: Schema; id: number | null; type: string | null } = $props();

	// the page is keyed by id and type, they don't change
	let ty = $state(untrack(() => type ?? ''));
	let title = $state('');
	let slug = $state('');
	let published = $state(false);
	let fields = $state<Values>({});
	let refs = $state<Refs>({});
	let loaded = $state(untrack(() => id === null));
	let saving = $state(false);
	let saved = $state(false);
	let error = $state('');
	let errors = $state<Record<string, string>>({});

	const comp = $derived(schema.components.find(c => c.handle === ty));

	$effect(() => {
		if (id === null) return;
		getEntry(id)
			.then(({ entry, relations }) => {
				ty = entry.type;
				title = entry.title;
				slug = entry.slug;
				published = entry.published;
				fields = entry.fields;
				refs = relations;
				loaded = true;
			})
			.catch(e => (error = e.message));
	});

	async function save() {
		saving = true;
		error = '';
		errors = {};
		try {
			const entry = await saveEntry(id, {
				type: ty,
				title,
				slug,
				published,
				fields: $state.snapshot(fields),
			});
			if (id === null) return go('#/entry/' + entry.id);
			slug = entry.slug;
			fields = entry.fields;
			saved = true;
			setTimeout(() => (saved = false), 1500);
		} catch (e: any) {
			error = e.message;
			if (e instanceof ApiError) {
				for (const f of e.fields) errors[f.path] = f.message;
			}
		} finally {
			saving = false;
		}
	}

	async function remove() {
		if (id === null || !confirm(`Delete ${title}?`)) return;
		await deleteEntry(id);
		go('#/list/' + ty);
	}

	function onkeydown(e: KeyboardEvent) {
		if ((e.metaKey || e.ctrlKey) && e.key === 's') {
			e.preventDefault();
			save();
		}
	}
</script>

<svelte:window {onkeydown} />

{#if error && !loaded}
	<p class="error">{error}</p>
{:else if loaded && comp}
	<header>
		<a href="#/list/{ty}">{comp.name}</a>
		<div class="actions">
			<label class="toggle">
				<input type="checkbox" bind:checked={published} />
				Published
			</label>
			{#if id !== null}
				<button class="danger" onclick={remove}>Delete</button>
			{/if}
			<button class="primary" onclick={save} disabled={saving}>
				{saved ? 'Saved' : 'Save'}
			</button>
		</div>
	</header>

	<article>
		<AutoText
			multiline={false}
			placeholder="Untitled"
			bind:value={title}
			heading
		/>
		{#if errors.title}<p class="error">Title {errors.title}</p>{/if}
		<div class="slug">
			/
			<input placeholder="slug from title" bind:value={slug} />
		</div>
		{#if errors.slug}<p class="error">Slug {errors.slug}</p>{/if}
		{#if error}<p class="error">{error}</p>{/if}

		<Fields {schema} component={comp} values={fields} {errors} {refs} />
	</article>
{/if}

<style lang="scss">
	header {
		position: sticky;
		top: 0;
		z-index: 6;
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 0;
		background: white;

		a {
			color: var(--muted);
		}
	}

	.actions {
		display: flex;
		flex-wrap: wrap;
		justify-content: flex-end;
		gap: 0.5rem;
		align-items: center;
	}

	.toggle {
		display: flex;
		gap: 0.3rem;
		align-items: center;
		margin-right: 0.5rem;

		input {
			width: auto;
		}
	}

	article {
		max-width: 46rem;
		padding: 1.5rem 0 6rem 3.25rem;

		:global(> textarea.heading) {
			font-size: 2.3rem;
			font-weight: 700;
		}
	}

	.slug {
		display: flex;
		align-items: center;
		color: var(--muted);
		margin-bottom: 1.2rem;

		input {
			border: none;
			outline: none;
			padding: 0.2rem;
			color: inherit;
		}
	}

	@media (max-width: 700px) {
		article {
			padding-left: 0;
		}
	}
</style>
