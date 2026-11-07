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
	import Icon from 'zipp-ui/Icon.svelte';
	import {
		componentOf,
		entryUrl,
		siteName,
		uriOf,
		type Schema,
	} from '../../core/schema';
	import { go } from '../../core/router.svelte';
	import Fields from '../../fields/Fields.svelte';
	import { pathShown } from '../../fields/conditions';
	import { Tabs } from '../../fields/tabs.svelte';
	import { isAdmin } from '../../core/access';
	import {
		entryPath,
		sectionCrumbs,
		sectionPath,
		setupPath,
	} from '../../core/paths';
	import { EntryEditor } from './entryEditor.svelte';
	import { setEditing } from '../../core/editing';
	import { flyout } from '../../core/flyout.svelte';
	import { session } from '../../core/session.svelte';
	import { guardLeave } from '../../core/leave';
	import ConflictBanner from './ConflictBanner.svelte';
	import EditHeader from './EditHeader.svelte';
	import EntryHead from './EntryHead.svelte';
	import EntrySidebar from './EntrySidebar.svelte';
	import EntryStatus from './EntryStatus.svelte';
	import LivePane from './LivePane.svelte';
	import { liveSetting, toggleLive } from './live.svelte';
	import RevisionView from './RevisionView.svelte';
	import TabBar from '../../fields/TabBar.svelte';
	import { SitePreview } from '../preview.svelte';
	import { previewToken } from '../drafts';
	import { t } from '../../i18n/index.svelte';

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
	setEditing({
		entryId: id,
		site: ed.site,
		get slug() {
			return ed.slug;
		},
		get section() {
			return ed.sec;
		},
		get type() {
			return ed.ty;
		},
	});

	const tabs = new Tabs(
		() => ed.comp,
		() => ed.fields,
	);
	// a hidden field's errors don't show, also not in the tab's count
	const errors = $derived(
		Object.fromEntries(
			Object.entries(ed.errors).filter(
				([p]) => !ed.comp || pathShown(schema, ed.comp, ed.fields, p),
			),
		),
	);
	const card = $derived(!!(ed.comp?.card || tabs.card));
	const url = $derived(entryUrl(schema, ed.site, ed.uri));

	// the website next to the editor; an entry in the flyout or without
	// a page (or an id) has none
	const canLive = $derived(!onclose && id !== null && !!url);
	// a new entry of a section with pages gets it once it is saved: the
	// button saves it unpublished first, like Craft's new entries are
	// drafts, and opens it with the website
	const canLiveNew = $derived(
		!onclose &&
			id === null &&
			!!schema.siteUrls[ed.site] &&
			!!ed.secDef &&
			uriOf(ed.secDef, ed.ty, ed.site) !== null,
	);
	const live = $derived(
		canLive && liveSetting.on
			? untrack(
					() =>
						new SitePreview(schema, ed.site, url, () =>
							previewToken(id!),
						),
				)
			: null,
	);

	// too narrow for the entry's sidebar next to the content (live
	// preview, tablets, phones): the CSS lays it out (editor-narrow),
	// this moves the status on top, collapsed; the same width as the
	// mixin's, the window's until the editor is measured
	const rem = parseFloat(getComputedStyle(document.documentElement).fontSize);
	let editorWidth = $state(0);
	const narrow = $derived((editorWidth || innerWidth) < 56 * rem);

	// every change goes into the draft, a moment after the last key
	$effect(() => {
		void ed.snapshot();
		untrack(() => ed.saver.changed(ed.may.edit));
	});

	async function save() {
		const created = await ed.save();
		if (created !== null) go(entryPath(ed.sec, created), true);
	}

	async function liveNew() {
		const created = await ed.save(true);
		if (created === null) return;
		if (!liveSetting.on) toggleLive();
		go(entryPath(ed.sec, created), true);
	}

	function discard() {
		if (confirm(t('entries.confirmDiscard'))) ed.discard();
	}

	function copyFrom(other: string) {
		if (
			confirm(t('entries.confirmCopy', { site: siteName(schema, other) }))
		)
			ed.copyFrom(other);
	}

	// the fields are read only while an action runs: publishing,
	// discarding and restoring load them new, what is typed meanwhile
	// would be lost; the field that had the focus gets it back
	let focused: HTMLElement | null = null;
	$effect.pre(() => {
		if (ed.busy) {
			focused = document.activeElement as HTMLElement | null;
			return;
		}
		const el = focused;
		focused = null;
		// after the rich text is editable again, see RichTextField
		if (el) setTimeout(() => el.isConnected && el.focus());
	});

	// what was typed last is saved, the next editor waits for it
	onDestroy(() => ed.saver.flush());
	// a new entry has no draft: leaving asks first
	onDestroy(guardLeave(() => ed.saver.unsaved));
	// the session ended while typing: saved after the new login
	onDestroy(session.onRenewed(() => ed.saver.retry()));

	async function remove() {
		if (!confirm(t('entries.confirmTrash', { title: ed.title }))) return;
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
			if (ed.id === null ? ed.may.edit : ed.may.publish) save();
		}
	}
</script>

<svelte:window {onkeydown} />

{#if ed.error && !ed.loaded}
	<p class="error">{ed.error}</p>
{:else if ed.loaded && ed.secDef && ed.comp}
	{@const comp = ed.comp}
	<EditHeader
		{ed}
		crumbs={ed.global
			? sectionCrumbs(schema, ed.sec).slice(0, -1)
			: sectionCrumbs(schema, ed.sec)}
		title={ed.global ? ed.secDef.name : ed.title}
		{url}
		live={canLive ? !!live : canLiveNew ? false : null}
		onlive={canLive ? toggleLive : liveNew}
		onpreview={() => url && ed.preview(url)}
		ondiscard={discard}
		onremove={remove}
		onsave={save}
		{narrow}
		copySites={ed.copySites}
		oncopy={copyFrom}
	/>

	<LivePane {ed} preview={live} {url} bind:width={editorWidth}>
		<ConflictBanner
			conflict={ed.conflict}
			onloadtheirs={() => ed.loadTheirs()}
			onkeepmine={() => ed.keepMine()}
		/>

		<div class="with-aside">
			<div class="main">
				{#if ed.error}<p class="error">{ed.error}</p>{/if}

				{#if ed.viewing}
					<!-- laid out like the editor, read only -->
					<RevisionView
						{schema}
						comp={componentOf(schema, ed.viewing.doc?.type ?? '') ??
							comp}
						refs={ed.refs}
						single={ed.single}
						global={ed.global}
						rev={ed.viewing.rev}
						doc={ed.viewing.doc}
						changed={ed.viewing.changed}
						tab={tabs.current}
						onrestore={ed.may.edit
							? () => ed.restore(ed.viewing!.rev)
							: null}
						onback={() => (ed.viewing = null)}
					/>
				{:else}
					<!-- read only for a role that may only look, and while
					publishing, discarding or restoring (see focused) -->
					<!-- a global has no title, the bar names it -->
					{#if !ed.global}
						<fieldset disabled={!ed.may.edit || ed.busy}>
							<EntryHead
								bind:title={ed.title}
								changed={ed.changed}
								errors={ed.errors}
							/>
						</fieldset>
						{#if narrow}
							<aside
								class="strip"
								aria-label={t('entries.status')}
							>
								<EntryStatus {schema} {ed} compact />
							</aside>
						{/if}
					{/if}
					{#if tabs.shown.length || isAdmin(schema)}
						<div class="tabs-row">
							{#if tabs.shown.length}
								<TabBar
									tabs={tabs.shown}
									bind:current={tabs.current}
									changed={ed.changed}
									{errors}
								/>
							{/if}
							{#if isAdmin(schema)}
								{@const layout = t('entries.fieldLayout', {
									name: comp.name,
								})}
								<a
									class="layout-link"
									href={setupPath('types', comp.handle)}
									title={layout}
								>
									<Icon name="settings" size={14} />
									<span class="text">{layout}</span>
								</a>
							{/if}
						</div>
					{/if}
					<fieldset disabled={!ed.may.edit || ed.busy}>
						<Fields
							{schema}
							component={comp}
							values={ed.fields}
							{errors}
							refs={ed.refs}
							changed={ed.changed}
							only={tabs.fields}
							{card}
						/>
					</fieldset>
				{/if}
			</div>

			<EntrySidebar
				{schema}
				{ed}
				shownFields={tabs.fields}
				{narrow}
				oncopy={copyFrom}
				onview={view}
			/>
		</div>
	</LivePane>
{/if}

<style lang="scss">
	// the entry's sidebar next to the content when the editor has room,
	// also in a window that has none for other pages' asides
	.with-aside {
		grid-template-columns: minmax(0, 1fr) 18.75rem;
	}

	@include editor-narrow {
		.with-aside {
			grid-template-columns: minmax(0, 1fr);
		}
	}

	.strip {
		margin-bottom: 1.125rem;
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
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		flex-shrink: 0;
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

	// narrow: only its icon, the tabs keep the row
	@include container-max(36rem) {
		.layout-link .text {
			display: none;
		}
	}
</style>
