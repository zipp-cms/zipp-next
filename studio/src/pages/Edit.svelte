<script lang="ts">
	import {
		ApiError,
		deleteEntry,
		getEntry,
		listEntries,
		saveEntry,
		type Entry,
		type Refs,
		type Schema,
		type Values,
	} from '../api';
	import { untrack } from 'svelte';
	import { go, site } from '../route.svelte';
	import Fields from '../fields/Fields.svelte';

	let {
		schema,
		id,
		section,
	}: { schema: Schema; id: number | null; section: string | null } = $props();

	// the page is keyed by id, section and site, they don't change
	let sec = $state(untrack(() => section ?? ''));
	let ty = $state('');
	let title = $state('');
	let slug = $state('');
	let uri = $state<string | null>(null);
	let published = $state(false);
	let parentId = $state<number | null>(null);
	let fields = $state<Values>({});
	let refs = $state<Refs>({});
	let loaded = $state(untrack(() => id === null));
	let saving = $state(false);
	let saved = $state(false);
	let error = $state('');
	let errors = $state<Record<string, string>>({});
	// possible parents of a new entry in a structure
	let parents = $state<Entry[]>([]);

	const secDef = $derived(schema.sections.find(s => s.handle === sec));
	const comp = $derived(schema.components.find(c => c.handle === ty));
	const single = $derived(secDef?.kind === 'single');

	$effect(() => {
		if (id === null) {
			ty = secDef?.types[0] ?? '';
			if (secDef?.kind === 'structure') {
				const params = {
					section: sec,
					site: site.current,
					order: 'structure',
				};
				listEntries(params).then(r => (parents = r.entries));
			}
			return;
		}
		getEntry(id, site.current)
			.then(({ entry, relations }) => {
				sec = entry.section;
				ty = entry.type;
				title = entry.title;
				slug = entry.slug;
				uri = entry.uri;
				published = entry.published;
				parentId = entry.parentId;
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
				section: sec,
				type: ty,
				site: site.current,
				parentId,
				title,
				slug,
				published,
				fields: $state.snapshot(fields),
			});
			if (id === null) return go('#/entry/' + entry.id);
			slug = entry.slug;
			uri = entry.uri;
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
		if (id === null || !confirm(`Delete ${title} in all sites?`)) return;
		await deleteEntry(id);
		go('#/list/' + sec);
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
{:else if loaded && secDef && comp}
	<header>
		<a href="#/list/{sec}">{secDef.name}</a>
		<div class="actions">
			<label class="toggle">
				<input type="checkbox" bind:checked={published} />
				Published
			</label>
			{#if id !== null && !single}
				<button class="danger" onclick={remove}>Delete</button>
			{/if}
			<button class="primary" onclick={save} disabled={saving}>
				{saved ? 'Saved' : 'Save'}
			</button>
		</div>
	</header>

	{#if id === null && (secDef.types.length > 1 || parents.length)}
		<div class="meta">
			{#if secDef.types.length > 1}
				<label>
					Type
					<select bind:value={ty}>
						{#each secDef.types as t (t)}
							<option value={t}>
								{schema.components.find(c => c.handle === t)
									?.name}
							</option>
						{/each}
					</select>
				</label>
			{/if}
			{#if parents.length}
				<label>
					Parent
					<select bind:value={parentId}>
						<option value={null}>None, on the top</option>
						{#each parents as p (p.id)}
							<option value={p.id}>{p.title}</option>
						{/each}
					</select>
				</label>
			{/if}
		</div>
	{/if}

	<input class="title" placeholder="Title" bind:value={title} />
	{#if errors.title}<p class="error">{errors.title}</p>{/if}
	{#if !single}
		<div class="slug">
			<input placeholder="slug from title" bind:value={slug} />
			{#if uri !== null}<span>/{uri}</span>{/if}
		</div>
	{/if}
	{#if errors.slug}<p class="error">{errors.slug}</p>{/if}

	{#if error}<p class="error">{error}</p>{/if}

	<Fields {schema} component={comp} values={fields} {errors} {refs} />
{/if}

<style lang="scss">
	header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;

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

	.title {
		font-size: 1.6rem;
		font-weight: 600;
		border: none;
		padding: 0.2rem 0;
	}

	.meta {
		display: flex;
		gap: 1rem;
		margin-bottom: 1rem;

		label {
			display: flex;
			flex-direction: column;
			gap: 0.2rem;
			color: var(--muted);
			font-size: 0.9rem;
		}
	}

	.slug {
		display: flex;
		gap: 0.75rem;
		align-items: center;
		color: var(--muted);
		margin-bottom: 1.5rem;

		input {
			width: 14rem;
			border: none;
			padding: 0.2rem;
			color: inherit;
		}
	}
</style>
