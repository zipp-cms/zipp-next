<script lang="ts">
	import {
		ApiError,
		deleteEntry,
		getEntry,
		entryUrl,
		listEntries,
		saveEntry,
		type Entry,
		type Refs,
		type Schema,
		type Values,
	} from '../api';
	import { untrack } from 'svelte';
	import { go, shown, site } from '../route.svelte';
	import Fields from '../fields/Fields.svelte';
	import Icon from '../lib/Icon.svelte';

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
	let tab = $state<'content' | 'options'>('content');
	// possible parents of a new entry in a structure
	let parents = $state<Entry[]>([]);

	const secDef = $derived(schema.sections.find(s => s.handle === sec));
	const comp = $derived(schema.components.find(c => c.handle === ty));
	const single = $derived(secDef?.kind === 'single');
	const url = $derived(entryUrl(schema, site.current, uri));

	$effect(() => {
		shown.section = sec;
	});

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
	<div class="page-top">
		<h1>
			{title || 'New entry'}
			<span class="badge" class:draft={!published}>
				{published ? 'Published' : 'Draft'}
			</span>
		</h1>
		{#if url}
			<a
				class="view"
				href={url}
				target="_blank"
				title="View on the website"
			>
				<Icon name="view" />
				View
			</a>
		{/if}
		{#if id !== null && !single}
			<button class="dark" onclick={remove}>
				<Icon name="delete" />
				Delete
			</button>
		{/if}
		<button class="dark" onclick={save} disabled={saving}>
			<Icon name="save" />
			{saved ? 'Saved' : 'Save'}
		</button>
	</div>

	<div class="card">
		<div class="tabs">
			<button
				class:active={tab === 'content'}
				onclick={() => (tab = 'content')}
			>
				Content
			</button>
			<button
				class:active={tab === 'options'}
				onclick={() => (tab = 'options')}
			>
				Options
			</button>
		</div>

		{#if error}<p class="error">{error}</p>{/if}

		{#if tab === 'content'}
			<div class="head">
				<label>
					Title
					<input placeholder="Title" bind:value={title} />
					{#if errors.title}<span class="error">
							{errors.title}
						</span>{/if}
				</label>
				{#if !single}
					<label>
						Slug
						<input placeholder="from the title" bind:value={slug} />
						{#if uri !== null}<span class="uri">/{uri}</span>{/if}
						{#if errors.slug}<span class="error">
								{errors.slug}
							</span>{/if}
					</label>
				{/if}
			</div>

			<Fields {schema} component={comp} values={fields} {errors} {refs} />
		{:else}
			<div class="options">
				<span>Published</span>
				<input type="checkbox" bind:checked={published} />
				<span>Section</span>
				<span>{secDef.name}</span>
				<span>Type</span>
				{#if id === null && secDef.types.length > 1}
					<select bind:value={ty}>
						{#each secDef.types as t (t)}
							<option value={t}>
								{schema.components.find(c => c.handle === t)
									?.name}
							</option>
						{/each}
					</select>
				{:else}
					<span>{comp.name}</span>
				{/if}
				{#if id === null && parents.length}
					<span>Parent</span>
					<select bind:value={parentId}>
						<option value={null}>None, on the top</option>
						{#each parents as p (p.id)}
							<option value={p.id}>{p.title}</option>
						{/each}
					</select>
				{/if}
			</div>
		{/if}
	</div>
{/if}

<style lang="scss">
	.view {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 10px 12px;
		font-size: 14px;
		color: var(--text);
		text-decoration: none;
		border: 1px solid var(--dark);
		border-radius: 2px;
		background: white;
	}

	h1 {
		display: flex;
		align-items: center;
		gap: 10px;
	}

	.badge {
		padding: 2px 6px;
		font-size: 13px;
		font-weight: normal;
		color: white;
		background: var(--dark);
		border-radius: 2px;

		&.draft {
			background: #c4c4c8;
		}
	}

	.tabs {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 24px;
		margin: -14px -16px 24px;
		padding: 0 16px;
		border-bottom: 2px solid var(--line);

		button {
			padding: 12px 0;
			font-size: 18px;
			border: none;
			border-bottom: 2px solid transparent;
			border-radius: 0;
			background: none;
			margin-bottom: -2px;

			&.active {
				border-bottom-color: var(--dark);
			}
		}
	}

	.head {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 24px;
		margin-bottom: 30px;

		label {
			display: flex;
			flex-direction: column;
			gap: 10px;
		}
	}

	.uri {
		margin-top: -4px;
		color: var(--muted);
		font-size: 14px;
	}

	.options {
		display: grid;
		grid-template-columns: 200px 1fr;
		gap: 16px 30px;
		align-items: center;

		select {
			max-width: 24rem;
		}
	}

	@media (max-width: 800px) {
		.head,
		.options {
			grid-template-columns: 1fr;
		}
	}
</style>
