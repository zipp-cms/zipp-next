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
	import { onDestroy, untrack } from 'svelte';
	import { entryUrl, type Schema } from '../../core/schema';
	import { entryPath, go } from '../../core/router.svelte';
	import Fields from '../../fields/Fields.svelte';
	import { pathShown, tabShown } from '../../fields/conditions';
	import { can, isAdmin } from '../../core/access';
	import { sectionCrumbs, sectionPath, setupPath } from '../../core/paths';
	import { EntryEditor } from './entryEditor.svelte';
	import { setEditing } from '../../core/editing';
	import { flyout } from '../../core/flyout.svelte';
	import { session } from '../../core/session.svelte';
	import { componentOf, sectionOf } from '../../core/schema';
	import ConflictBanner from './ConflictBanner.svelte';
	import EditHeader from './EditHeader.svelte';
	import EntryHead from './EntryHead.svelte';
	import EntrySidebar from './EntrySidebar.svelte';
	import RevisionView from './RevisionView.svelte';
	import TabBar from './TabBar.svelte';
	import Preview from '../Preview.svelte';
	import SplitHandle from 'zipp-ui/SplitHandle.svelte';
	import { SitePreview } from '../preview.svelte';
	import { previewToken } from '../drafts';

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
	// the tabs of the entry type whose conditions hold, the first one open
	const tabs = $derived(
		(comp?.tabs ?? []).filter(t => tabShown(comp!, t, ed.fields)),
	);
	// a hidden field's errors don't show, also not in the tab's count
	const errors = $derived(
		Object.fromEntries(
			Object.entries(ed.errors).filter(
				([p]) => !comp || pathShown(schema, comp, ed.fields, p),
			),
		),
	);
	let tab = $state('');
	$effect(() => {
		if (tabs.length && !tabs.some(t => t.name === tab)) tab = tabs[0].name;
	});
	const shownTab = $derived(tabs.find(t => t.name === tab));
	const shownFields = $derived(shownTab?.fields ?? null);
	const card = $derived(!!(comp?.card || shownTab?.card));
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

	// the website next to the editor, on or off in this browser; an entry
	// in the flyout or without a page (or an id) has none
	const LIVE_KEY = 'zipp-live-preview';
	let liveOn = $state(localStorage.getItem(LIVE_KEY) === '1');
	const canLive = $derived(!onclose && id !== null && !!url);
	const live = $derived(
		canLive && liveOn
			? untrack(
					() =>
						new SitePreview(schema, ed.site, url, () =>
							previewToken(id!),
						),
				)
			: null,
	);
	let liveWidth = $state(0);
	let resizing = $state(false);

	function toggleLive() {
		liveOn = !liveOn;
		localStorage.setItem(LIVE_KEY, liveOn ? '1' : '');
	}

	// the draft is new after every save, publish and discard; a published
	// slug moves the page
	let shownUrl = untrack(() => url);
	$effect(() => {
		void ed.draft;
		const u = url;
		untrack(() => {
			if (u && u !== shownUrl) live?.open(u);
			else live?.update();
			shownUrl = u;
		});
	});

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

	// what was typed last is saved, the next editor waits for it
	onDestroy(() => ed.flush());
	// the session ended while typing: saved after the new login
	onDestroy(session.onRenewed(() => ed.retry()));

	async function remove() {
		if (!confirm(`Move ${ed.title} to the trash?`)) return;
		if (!(await ed.remove())) return;
		if (onclose) onclose();
		else go(sectionPath(ed.sec));
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
		live={canLive ? !!live : null}
		onlive={toggleLive}
		onpreview={() => url && ed.preview(url)}
		ondiscard={discard}
		onremove={remove}
		onsave={save}
	/>

	<div class:live style:--frame="{liveWidth}px">
		<div>
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
								comp={componentOf(
									schema,
									ed.viewing.doc?.type ?? '',
								) ?? comp}
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
						<!-- a global has no title, the bar names it -->
						{#if !global}
							<fieldset disabled={!canEdit}>
								<EntryHead
									bind:title={ed.title}
									changed={ed.changed}
									errors={ed.errors}
								/>
							</fieldset>
						{/if}
						{#if tabs.length || isAdmin(schema)}
							<div class="tabs-row">
								{#if tabs.length}
									<TabBar
										{tabs}
										bind:current={tab}
										changed={ed.changed}
										{errors}
									/>
								{/if}
								{#if isAdmin(schema)}
									<a
										class="layout-link"
										href={setupPath('types', comp.handle)}
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
								{errors}
								refs={ed.refs}
								changed={ed.changed}
								only={shownFields}
								{card}
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
		</div>

		{#if live}
			<div class="frame">
				<SplitHandle
					bind:width={liveWidth}
					bind:resizing
					key="zipp-live-preview-width"
					initial={560}
					min={320}
					side="end"
					label="Width of the website"
				/>
				<Preview preview={live} {resizing} />
			</div>
		{/if}
	</div>
{/if}

<style lang="scss">
	// the website on the right, as high as the window under the bar
	.live {
		display: grid;
		grid-template-columns: minmax(0, 1fr) var(--frame);
		gap: 1.25rem;

		.with-aside {
			grid-template-columns: 1fr;
		}
	}

	.frame {
		position: sticky;
		top: 3rem;
		align-self: start;
		display: grid;
		height: calc(100vh - 3rem);
		margin: -1.25rem calc(-1 * var(--page-pad)) -2rem 0;
		border-left: 1px solid var(--line);
	}

	@include max-tablet {
		.live {
			grid-template-columns: 1fr;
		}

		.frame {
			display: none;
		}
	}

	// the fields put their labels on the left when this is wide
	.main {
		container-type: inline-size;
	}

	.tabs-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.75rem;
		margin-bottom: 1.125rem;
	}

	.layout-link {
		margin-left: auto;
		color: var(--muted);
		font-size: var(--fs-sm);
		text-decoration: none;

		@include hover {
			&:hover {
				color: var(--accent);
			}
		}
	}
</style>
