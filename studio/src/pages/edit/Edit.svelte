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
	} from '../../api';
	import { untrack } from 'svelte';
	import { entryPath, go, shown, site } from '../../route.svelte';
	import Fields from '../../fields/Fields.svelte';
	import { changedPaths, clean } from '../../lib/diff';
	import ConflictBanner from './ConflictBanner.svelte';
	import EditHeader from './EditHeader.svelte';
	import EntryHead from './EntryHead.svelte';
	import OptionsTab from './OptionsTab.svelte';
	import RevisionView from './RevisionView.svelte';

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

	// as publishing keeps it: an empty group that a field fills in on
	// load is no change
	function snapshot() {
		return JSON.stringify([
			title,
			slug,
			published,
			clean($state.snapshot(fields)),
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
</script>

<svelte:window {onkeydown} />

{#if error && !loaded}
	<p class="error">{error}</p>
{:else if loaded && secDef && comp}
	<EditHeader
		{title}
		{published}
		hasDraft={!!draft}
		saving={draftState === 'saving'}
		{url}
		{busy}
		isNew={id === null}
		canDelete={id !== null && !single}
		onpreview={preview}
		ondiscard={discard}
		onremove={remove}
		onsave={save}
	/>

	<ConflictBanner
		{conflict}
		{draft}
		onloadtheirs={loadTheirs}
		onkeepmine={keepMine}
	/>

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
				<RevisionView
					{schema}
					{comp}
					{refs}
					{single}
					rev={viewing.rev}
					doc={viewing.doc}
					changed={viewing.changed}
					onrestore={() => restore(viewing!.rev)}
					onback={() => (viewing = null)}
				/>
			{:else}
				<EntryHead
					bind:title
					bind:slug
					{single}
					{changed}
					{uri}
					{errors}
				/>
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
			<OptionsTab
				{schema}
				{secDef}
				{comp}
				isNew={id === null}
				bind:published
				bind:ty
				bind:parentId
				{parents}
				{revisions}
				oncopy={copyFrom}
				onview={view}
				onrestore={restore}
			/>
		{/if}
	</div>
{/if}

<style lang="scss">
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
</style>
