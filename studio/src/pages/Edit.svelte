<script lang="ts">
	import {
		ApiError,
		deleteEntry,
		discardDraft,
		entryUrl,
		getDraft,
		getEntry,
		getRevision,
		getRevisions,
		listEntries,
		previewToken,
		publishDraft,
		putDraft,
		restoreRevision,
		saveEntry,
		type Draft,
		type Entry,
		type Refs,
		type Revision,
		type Schema,
		type SiteDoc,
		type Values,
	} from '../api';
	import { untrack } from 'svelte';
	import { entryPath, go, shown, site } from '../route.svelte';
	import Fields from '../fields/Fields.svelte';
	import Icon from '../lib/Icon.svelte';
	import { changedPaths } from '../lib/diff';

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
	let busy = $state(false);
	let error = $state('');
	let errors = $state<Record<string, string>>({});
	let tab = $state<'content' | 'options'>('content');
	// possible parents of a new entry in a structure
	let parents = $state<Entry[]>([]);
	// unpublished changes, saved while typing
	let draft = $state<Draft | null>(null);
	let draftState = $state<'' | 'saving' | 'saved'>('');
	let revisions = $state<Revision[]>([]);
	// the live content of the site, to mark what the draft changes
	let live = $state<SiteDoc | null>(null);
	// an old revision, shown read only
	let viewing = $state<{
		rev: Revision;
		doc: SiteDoc | null;
		changed: Set<string>;
	} | null>(null);
	// someone else changed the draft, nothing is saved until resolved
	let conflict = $state('');

	const secDef = $derived(schema.sections.find(s => s.handle === sec));
	const comp = $derived(schema.components.find(c => c.handle === ty));
	const single = $derived(secDef?.kind === 'single');
	const url = $derived(entryUrl(schema, site.current, uri));

	$effect(() => {
		shown.section = sec;
	});

	async function load() {
		if (id === null) return;
		const [{ entry, relations }, d] = await Promise.all([
			getEntry(id, site.current),
			getDraft(id),
		]);
		sec = entry.section;
		ty = entry.type;
		uri = entry.uri;
		parentId = entry.parentId;
		refs = relations;
		draft = d;
		live = {
			title: entry.title,
			slug: entry.slug,
			published: entry.published,
			fields: entry.fields,
		};
		const content = d?.doc[site.current] ?? entry;
		title = content.title;
		slug = content.slug;
		published = content.published;
		fields = content.fields;
		lastSaved = snapshot();
		loaded = true;
		revisions = await getRevisions(id);
	}

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
		load().catch(e => (error = e.message));
	});

	function snapshot() {
		return JSON.stringify([
			title,
			slug,
			published,
			$state.snapshot(fields),
		]);
	}

	// every change goes into the draft, a moment after the last key
	let lastSaved = '';
	let timer: ReturnType<typeof setTimeout>;
	$effect(() => {
		const snap = snapshot();
		if (!loaded || id === null || snap === lastSaved || conflict) return;
		clearTimeout(timer);
		draftState = 'saving';
		timer = setTimeout(() => saveDraft(snap), 700);
	});

	async function saveDraft(snap: string) {
		if (id === null) return;
		const content = {
			title,
			slug,
			published,
			fields: $state.snapshot(fields),
		};
		try {
			draft = await putDraft(
				id,
				site.current,
				content,
				draft?.updatedAt ?? null,
			);
			lastSaved = snap;
			draftState = 'saved';
		} catch (e: any) {
			if (e instanceof ApiError && e.status === 409) {
				conflict = e.message;
				draftState = '';
			} else {
				error = e.message;
			}
		}
	}

	async function run(f: () => Promise<unknown>) {
		busy = true;
		error = '';
		errors = {};
		try {
			await f();
		} catch (e: any) {
			error = e.message;
			if (e instanceof ApiError) {
				for (const f of e.fields) errors[f.path] = f.message;
			}
		} finally {
			busy = false;
		}
	}

	/** A new entry is created directly, a draft is published. */
	function save() {
		return run(async () => {
			if (id === null) {
				const entry = await saveEntry(null, {
					section: sec,
					type: ty,
					site: site.current,
					parentId,
					title,
					slug,
					published,
					fields: $state.snapshot(fields),
				});
				return go(entryPath(sec, entry.id), true);
			}
			// the last keys first
			clearTimeout(timer);
			const snap = snapshot();
			if (snap !== lastSaved) await saveDraft(snap);
			if (!draft) return;
			await publishDraft(id);
			draftState = '';
			await load();
		});
	}

	function discard() {
		if (id === null || !confirm('Throw away the unpublished changes?'))
			return;
		return run(async () => {
			await discardDraft(id!);
			draftState = '';
			await load();
		});
	}

	/**
	 * Takes title and fields of another site into the draft of this one,
	 * e.g. to start a translation from the German text. The slug stays.
	 */
	function copyFrom(other: string) {
		const name = schema.sites.find(s => s.handle === other)?.name;
		if (
			id === null ||
			!confirm(`Replace the content with the one of ${name}?`)
		)
			return;
		return run(async () => {
			// the other site as the editor sees it: its draft, or live
			const d = await getDraft(id!);
			const theirs = d?.doc[other] ?? (await getEntry(id!, other)).entry;
			title = theirs.title;
			fields = theirs.fields;
		});
	}

	/** Throws away the own changes and shows the ones of the other. */
	async function loadTheirs() {
		clearTimeout(timer);
		draftState = '';
		await run(load);
		// only now, else the own state would be saved again
		conflict = '';
	}

	/** Saves the own changes over the ones of the other. */
	async function keepMine() {
		conflict = '';
		if (id === null) return;
		draft = await getDraft(id);
		await saveDraft(snapshot());
	}

	/** What the draft changes against the live entry, also inside blocks. */
	const changed = $derived.by(() => {
		if (!draft || !live) return new Set<string>();
		const set = changedPaths(live.fields, $state.snapshot(fields));
		if (title !== live.title) set.add('title');
		if (slug !== live.slug) set.add('slug');
		return set;
	});

	/** Shows a revision with what it changed against the one before. */
	async function view(rev: Revision) {
		const i = revisions.indexOf(rev);
		const before = revisions[i + 1];
		const [doc, prev] = await Promise.all([
			getRevision(id!, rev.id),
			before ? getRevision(id!, before.id) : null,
		]);
		const now = doc[site.current] ?? null;
		const old = prev?.[site.current] ?? null;
		const set = new Set<string>();
		if (now && old) {
			changedPaths(old.fields, now.fields, '', set);
			if (now.title !== old.title) set.add('title');
			if (now.slug !== old.slug) set.add('slug');
		}
		viewing = { rev, doc: now, changed: set };
		tab = 'content';
		window.scrollTo(0, 0);
	}

	/** `Title (de), Intro (de, en)` */
	function changeText(rev: Revision) {
		if (!rev.changes.length) return 'Created';
		const label = (f: string) =>
			comp?.fields.find(x => x.handle === f)?.label ??
			f.charAt(0).toUpperCase() + f.slice(1);
		const multi = schema.sites.length > 1;
		return rev.changes
			.map(
				c => label(c.field) + (multi ? ` (${c.sites.join(', ')})` : ''),
			)
			.join(', ');
	}

	function restore(rev: Revision) {
		return run(async () => {
			await restoreRevision(id!, rev.id);
			viewing = null;
			await load();
			tab = 'content';
		});
	}

	async function preview() {
		if (id === null || !url) return;
		const token = await previewToken(id);
		window.open(`${url}?token=${token}`, '_blank');
	}

	async function remove() {
		if (id === null || !confirm(`Delete ${title} in all sites?`)) return;
		await deleteEntry(id);
		go('/entries/' + sec);
	}

	function onkeydown(e: KeyboardEvent) {
		if ((e.metaKey || e.ctrlKey) && e.key === 's') {
			e.preventDefault();
			save();
		}
	}

	function time(ms: number) {
		return new Date(ms).toLocaleString();
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
				{published ? 'Published' : 'Not published'}
			</span>
			{#if draft}
				<span
					class="badge changes"
					title="Saved as draft, not live yet"
				>
					{draftState === 'saving'
						? 'Saving…'
						: 'Unpublished changes'}
				</span>
			{/if}
		</h1>
		{#if url}
			{#if draft}
				<button class="light" onclick={preview}>
					<Icon name="view" />
					Preview
				</button>
			{:else}
				<a class="light" href={url} target="_blank">
					<Icon name="view" />
					View
				</a>
			{/if}
		{/if}
		{#if draft}
			<button class="light" onclick={discard} disabled={busy}>
				Discard
			</button>
		{/if}
		{#if id !== null && !single}
			<button class="dark" onclick={remove}>
				<Icon name="delete" />
				Delete
			</button>
		{/if}
		<button
			class="dark"
			onclick={save}
			disabled={busy || (id !== null && !draft)}
		>
			<Icon name="save" />
			{id === null ? 'Create' : 'Publish'}
		</button>
	</div>

	{#if conflict}
		<div class="conflict">
			<span>
				<b>{conflict}</b>
				while you were editing. Your last changes are not saved yet.
			</span>
			<button class="dark" onclick={loadTheirs}>
				Load their changes
			</button>
			<button class="light" onclick={keepMine}>Keep mine</button>
		</div>
	{:else if draft?.user}
		<p class="last-change">
			Draft last changed by {draft.user}, {time(draft.updatedAt)}
		</p>
	{/if}

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
			{#if viewing}
				{@const rc = viewing.changed}
				<div class="viewing">
					<span>
						Revision of {time(viewing.rev.createdAt)}{viewing.rev
							.user
							? ` by ${viewing.rev.user}`
							: ''}, read only
						<br />
						<small>Changed: {changeText(viewing.rev)}</small>
					</span>
					<button class="dark" onclick={() => restore(viewing!.rev)}>
						Restore
					</button>
					<button class="light" onclick={() => (viewing = null)}>
						Back to editing
					</button>
				</div>
				{#if viewing.doc}
					<!-- a disabled fieldset disables every input in it -->
					<fieldset disabled>
						<div class="head">
							<label>
								<span>
									{#if rc.has('title')}<span
											class="dot"
										></span>{/if}Title
								</span>
								<input value={viewing.doc.title} />
							</label>
							{#if !single}
								<label>
									<span>
										{#if rc.has('slug')}<span
												class="dot"
											></span>{/if}Slug
									</span>
									<input value={viewing.doc.slug} />
								</label>
							{/if}
						</div>
						<Fields
							{schema}
							component={comp}
							values={viewing.doc.fields}
							errors={{}}
							{refs}
							changed={rc}
						/>
					</fieldset>
				{:else}
					<p class="muted">
						This revision has no content for this site.
					</p>
				{/if}
			{:else}
				<div class="head">
					<label>
						<span>
							{#if changed.has('title')}<span
									class="dot"
								></span>{/if}Title
						</span>
						<input placeholder="Title" bind:value={title} />
						{#if errors.title}<span class="error">
								{errors.title}
							</span>{/if}
					</label>
					{#if !single}
						<label>
							<span>
								{#if changed.has('slug')}<span
										class="dot"
									></span>{/if}Slug
							</span>
							<input
								placeholder="from the title"
								bind:value={slug}
							/>
							{#if uri !== null}<span class="uri">
									/{uri}
								</span>{/if}
							{#if errors.slug}<span class="error">
									{errors.slug}
								</span>{/if}
						</label>
					{/if}
				</div>

				<Fields
					{schema}
					component={comp}
					values={fields}
					{errors}
					{refs}
					{changed}
				/>
			{/if}
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
				{#if id !== null && schema.sites.length > 1}
					<span>Content</span>
					<select
						value=""
						onchange={e => {
							const other = e.currentTarget.value;
							e.currentTarget.value = '';
							copyFrom(other);
						}}
					>
						<option value="" disabled>
							Copy from another site…
						</option>
						{#each schema.sites.filter(s => s.handle !== site.current) as s (s.handle)}
							<option value={s.handle}>{s.name}</option>
						{/each}
					</select>
				{/if}
				{#if revisions.length}
					<span>Revisions</span>
					<ul class="revisions">
						{#each revisions as rev, i (rev.id)}
							<li>
								<button
									class="link rev"
									onclick={() => view(rev)}
								>
									<span>
										{time(rev.createdAt)}
										{#if rev.user}<span class="muted">
												{rev.user}
											</span>{/if}
										{#if i === 0}<span class="muted">
												live
											</span>{/if}
									</span>
									<span class="what">{changeText(rev)}</span>
								</button>
								{#if i > 0}
									<button
										class="link"
										onclick={() => restore(rev)}
									>
										Restore
									</button>
								{/if}
							</li>
						{/each}
					</ul>
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
	.light {
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

	.conflict {
		display: flex;
		align-items: center;
		gap: 12px;
		margin-bottom: 16px;
		padding: 12px 16px;
		background: #ffcd70;

		span {
			margin-right: auto;
		}
	}

	.last-change {
		margin: -10px 0 12px;
		font-size: 13px;
		color: var(--muted);
	}

	.viewing {
		display: flex;
		align-items: center;
		gap: 12px;
		margin-bottom: 24px;
		padding: 12px 16px;
		background: var(--input);

		span {
			margin-right: auto;
		}
	}

	fieldset {
		margin: 0;
		padding: 0;
		border: none;
	}

	.dot {
		display: inline-block;
		width: 8px;
		height: 8px;
		margin-right: 6px;
		border-radius: 50%;
		background: #d98e04;
	}

	.rev {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		gap: 2px;
		padding: 0;
		color: var(--text);
		text-align: left;
	}

	.what {
		font-size: 13px;
		color: var(--muted);
	}

	.changes {
		background: #d98e04 !important;
	}

	.revisions {
		margin: 0;
		padding: 0;
		list-style: none;

		li {
			display: flex;
			justify-content: space-between;
			align-items: center;
			padding: 6px 0;
			border-bottom: 1px solid var(--line);
		}
	}

	.muted {
		margin-left: 8px;
		color: var(--muted);
		font-size: 14px;
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
