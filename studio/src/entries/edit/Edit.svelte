<script module lang="ts">
	import type { Schema as SchemaT } from '../../core/schema';
	import { EntryEditor as Editor } from './entryEditor.svelte';

	/** The entry, or a new one of the section, loaded before it shows. */
	export async function loadProps(props: {
		schema: SchemaT;
		section: string;
		id: string;
	}) {
		const id = props.id === 'new' ? null : Number(props.id);
		return { editor: await Editor.open(props.schema, id, props.section) };
	}
</script>

<script lang="ts">
	import { untrack } from 'svelte';
	import { entryUrl, type Schema } from '../../core/schema';
	import { entryPath, go, site } from '../../core/router.svelte';
	import Fields from '../../fields/Fields.svelte';
	import { can, isAdmin, sectionCrumbs } from '../../core/access';
	import { EntryEditor } from './entryEditor.svelte';
	import { setEditing } from '../../core/editing';
	import { flyout } from '../../core/flyout.svelte';
	import { componentOf, sectionOf } from '../../core/schema';
	import ConflictBanner from './ConflictBanner.svelte';
	import EditHeader from './EditHeader.svelte';
	import EntryHead from './EntryHead.svelte';
	import EntrySidebar from './EntrySidebar.svelte';
	import RevisionView from './RevisionView.svelte';
	import TabBar from './TabBar.svelte';

	let {
		schema,
		editor,
		onclose,
	}: {
		schema: Schema;
		/** loaded, see `loadProps` */
		editor: EntryEditor;
		/** shown in the flyout, which this closes */
		onclose?: () => void;
	} = $props();

	// the page is new for another entry or site
	const ed = untrack(() => editor);
	const id = ed.id;
	// the fields in it know their entry, also in the flyout
	setEditing({ entryId: id, site: ed.site });

	const secDef = $derived(sectionOf(schema, ed.sec));
	const comp = $derived(componentOf(schema, ed.ty));
	const single = $derived(secDef?.kind === 'single');
	// the tabs of the entry type, the first one open
	const tabs = $derived(comp?.tabs ?? []);
	let tab = $state('');
	$effect(() => {
		if (tabs.length && !tabs.some(t => t.name === tab)) tab = tabs[0].name;
	});
	const shownFields = $derived(
		tabs.find(t => t.name === tab)?.fields ?? null,
	);
	// a global: a single without a page, its title is the section's name
	const global = $derived(
		single &&
			secDef?.uri === null &&
			!Object.keys(secDef?.type_uris ?? {}).length,
	);
	// what the role may do here, the server checks it too
	const canEdit = $derived(can(schema, ed.sec, 'edit'));
	const canPublish = $derived(can(schema, ed.sec, 'publish'));
	const may = $derived({
		edit: canEdit,
		publish: canPublish,
		delete: id !== null && !single && can(schema, ed.sec, 'delete'),
	});
	const url = $derived(entryUrl(schema, ed.site, ed.uri));

	// every change goes into the draft, a moment after the last key
	$effect(() => {
		void ed.snapshot();
		untrack(() => ed.changedNow(canEdit));
	});

	async function save() {
		const created = await ed.save();
		if (created !== null) go(entryPath(ed.sec, created), true);
	}

	function discard() {
		if (confirm('Throw away the unpublished changes?')) ed.discard();
	}

	function copyFrom(other: string) {
		const name = schema.sites.find(s => s.handle === other)?.name;
		if (confirm(`Replace the content with the one of ${name}?`))
			ed.copyFrom(other);
	}

	async function remove() {
		if (!confirm(`Delete ${ed.title} in all sites?`)) return;
		if (!(await ed.remove())) return;
		if (onclose) onclose();
		else go('/admin/entries/' + ed.sec);
	}

	async function view(rev: (typeof ed.revisions)[number]) {
		await ed.view(rev);
		window.scrollTo(0, 0);
	}

	function onkeydown(e: KeyboardEvent) {
		// with an entry in the flyout, it is the one that saves
		if (!onclose && flyout.open) return;
		if ((e.metaKey || e.ctrlKey) && e.key === 's') {
			e.preventDefault();
			// like the button: a new entry who may edit, else who may publish
			if (ed.id === null ? canEdit : canPublish) save();
		}
	}
</script>

<svelte:window {onkeydown} />

{#if ed.error && !ed.loaded}
	<p class="error">{ed.error}</p>
{:else if ed.loaded && secDef && comp}
	<EditHeader
		{ed}
		crumbs={global
			? sectionCrumbs(schema, ed.sec).slice(0, -1)
			: sectionCrumbs(schema, ed.sec)}
		title={global ? secDef.name : ed.title}
		{url}
		{may}
		onpreview={() => url && ed.preview(url)}
		ondiscard={discard}
		onremove={remove}
		onsave={save}
	/>

	<ConflictBanner
		conflict={ed.conflict}
		onloadtheirs={() => ed.loadTheirs()}
		onkeepmine={() => ed.keepMine()}
	/>

	<div class="with-aside">
		<div class="main">
			{#if ed.error}<p class="error">{ed.error}</p>{/if}

			{#if ed.viewing}
				<div class="card">
					<RevisionView
						{schema}
						{comp}
						refs={ed.refs}
						{single}
						rev={ed.viewing.rev}
						doc={ed.viewing.doc}
						changed={ed.viewing.changed}
						onrestore={canEdit
							? () => ed.restore(ed.viewing!.rev)
							: null}
						onback={() => (ed.viewing = null)}
					/>
				</div>
			{:else}
				<!-- read only for a role that may only look -->
				<fieldset disabled={!canEdit}>
					{#if global}
						<h1 class="global">{secDef.name}</h1>
					{:else}
						<EntryHead
							bind:title={ed.title}
							changed={ed.changed}
							errors={ed.errors}
						/>
					{/if}
				</fieldset>
				{#if tabs.length || isAdmin(schema)}
					<div class="tabs-row">
						{#if tabs.length}
							<TabBar
								{tabs}
								bind:current={tab}
								changed={ed.changed}
								errors={ed.errors}
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
						values={ed.fields}
						errors={ed.errors}
						refs={ed.refs}
						changed={ed.changed}
						only={shownFields}
					/>
				</fieldset>
			{/if}
		</div>

		<EntrySidebar
			{schema}
			{ed}
			{secDef}
			{comp}
			{global}
			{may}
			{shownFields}
			oncopy={copyFrom}
			onview={view}
		/>
	</div>
{/if}

<style lang="scss">
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
</style>
