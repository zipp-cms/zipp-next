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
	import { can, isAdmin, sectionCrumbs } from '../../lib/access';
	import { changedPaths, clean } from '../../lib/diff';
	import ConflictBanner from './ConflictBanner.svelte';
	import EditHeader from './EditHeader.svelte';
	import EntryHead from './EntryHead.svelte';
	import EntrySidebar from './EntrySidebar.svelte';
	import { time } from '../../lib/format';
	import { flyout } from '../../lib/flyout.svelte';
	import TabBar from './TabBar.svelte';
	import RevisionView from './RevisionView.svelte';

	let {
		schema,
		id,
		section,
		onclose,
	}: {
		schema: Schema;
		id: number | null;
		section: string | null;
		/** shown in the flyout, which this closes */
		onclose?: () => void;
	} = $props();

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
	// whether the entry is live in each site, as saved
	let perSite = $state<Record<string, boolean>>({});
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
	// the tabs of the entry type, the first one open
	const tabs = $derived(comp?.tabs ?? []);
	let tab = $state('');
	$effect(() => {
		if (tabs.length && !tabs.some(t => t.name === tab)) tab = tabs[0].name;
	});
	// a global: a single without a page, its title is the section's name
	const global = $derived(
		single &&
			secDef?.uri === null &&
			!Object.keys(secDef?.type_uris ?? {}).length,
	);
	// what the role may do here, the server checks it too
	const canEdit = $derived(can(schema, sec, 'edit'));
	const canPublish = $derived(can(schema, sec, 'publish'));
	const url = $derived(entryUrl(schema, site.current, uri));

	$effect(() => {
		// in the flyout the nav stays at the page behind it
		if (!onclose) shown.section = sec;
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
		const [revs, list] = await Promise.all([
			getRevisions(id),
			listEntries({ section: sec, ids: String(id), site: site.current }),
		]);
		revisions = revs;
		perSite = list.published[id] ?? {};
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
		if (!canEdit) return;
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
		if (published !== live.published) set.add('published');
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
		window.scrollTo(0, 0);
	}

	function restore(rev: Revision) {
		return run(async () => {
			await restoreRevision(id!, rev.id);
			viewing = null;
			await load();
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
		if (onclose) onclose();
		else go('/entries/' + sec);
	}

	function onkeydown(e: KeyboardEvent) {
		// with an entry in the flyout, it is the one that saves
		if (!onclose && flyout.open) return;
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
		crumbs={global
			? sectionCrumbs(schema, sec).slice(0, -1)
			: sectionCrumbs(schema, sec)}
		title={global ? secDef.name : title}
		{published}
		hasDraft={!!draft}
		draftBy={draft?.user
			? `by ${draft.user}, ${time(draft.updatedAt)}`
			: null}
		saving={draftState === 'saving'}
		{url}
		{busy}
		isNew={id === null}
		{canEdit}
		{canPublish}
		canDelete={id !== null && !single && can(schema, sec, 'delete')}
		onpreview={preview}
		ondiscard={discard}
		onremove={remove}
		onsave={save}
	/>

	<ConflictBanner
		{conflict}
		onloadtheirs={loadTheirs}
		onkeepmine={keepMine}
	/>

	<div class="layout">
		<div class="main">
			{#if error}<p class="error">{error}</p>{/if}

			{#if viewing}
				<div class="card">
					<RevisionView
						{schema}
						{comp}
						{refs}
						{single}
						rev={viewing.rev}
						doc={viewing.doc}
						changed={viewing.changed}
						onrestore={canEdit ? () => restore(viewing!.rev) : null}
						onback={() => (viewing = null)}
					/>
				</div>
			{:else}
				<!-- read only for a role that may only look -->
				<fieldset disabled={!canEdit}>
					{#if global}
						<h1 class="global">{secDef.name}</h1>
					{:else}
						<EntryHead bind:title {changed} {errors} />
					{/if}
				</fieldset>
				{#if tabs.length || isAdmin(schema)}
					<div class="tabs-row">
						{#if tabs.length}
							<TabBar
								{tabs}
								bind:current={tab}
								{changed}
								{errors}
							/>
						{/if}
						{#if isAdmin(schema)}
							<a
								class="layout-link"
								href="/admin/setup/types/{comp.handle}"
							>
								Field layout «{comp.name}»
							</a>
						{/if}
					</div>
				{/if}
				<fieldset disabled={!canEdit}>
					<Fields
						{schema}
						component={comp}
						values={fields}
						{errors}
						{refs}
						{changed}
						only={tabs.find(t => t.name === tab)?.fields ?? null}
					/>
				</fieldset>
			{/if}
		</div>

		<EntrySidebar
			{schema}
			{secDef}
			{comp}
			{global}
			isNew={id === null}
			{canEdit}
			{canPublish}
			bind:published
			bind:slug
			bind:ty
			bind:parentId
			{uri}
			{perSite}
			{changed}
			{errors}
			{parents}
			{revisions}
			values={fields}
			shownFields={tabs.find(t => t.name === tab)?.fields ?? null}
			oncopy={copyFrom}
			onview={view}
			onrestore={restore}
		/>
	</div>
{/if}

<style lang="scss">
	.layout {
		display: grid;
		grid-template-columns: minmax(0, 1fr) 280px;
		gap: 24px;
		align-items: start;
	}

	// the fields put their labels on the left when this is wide
	.main {
		container-type: inline-size;
	}

	.global {
		margin-bottom: 18px;
		font-size: var(--fs-title);
		line-height: 1.25;
		font-weight: 700;
	}

	.tabs-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 12px;
		margin-bottom: 18px;
	}

	.layout-link {
		margin-left: auto;
		color: var(--muted);
		font-size: var(--fs-sm);
		text-decoration: none;

		&:hover {
			color: var(--accent);
		}
	}

	@media (max-width: 1100px) {
		.layout {
			grid-template-columns: 1fr;
		}
	}
</style>
