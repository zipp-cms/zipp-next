<script lang="ts">
	import PageTop from 'zipp-ui/PageTop.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import Menu, { type MenuItem } from 'zipp-ui/Menu.svelte';
	import { time } from '../../core/format';
	import type { EntryEditor } from './entryEditor.svelte';
	import { t } from '../../i18n/index.svelte';

	/** Where the entry is, its state and what can be done with it. */
	let {
		ed,
		crumbs,
		title,
		url,
		live,
		onlive,
		onpreview,
		ondiscard,
		onremove,
		onsave,
		narrow = false,
		copySites = [],
		oncopy,
	}: {
		ed: EntryEditor;
		crumbs: { label: string; href?: string }[];
		title: string;
		url: string | null;
		/** the live preview is on; null: this entry has none */
		live: boolean | null;
		onlive: () => void;
		onpreview: () => void;
		ondiscard: () => void;
		onremove: () => void;
		onsave: () => void;
		/** the editor is narrow: the rarer actions go into a ⋯ menu */
		narrow?: boolean;
		/** the other sites whose content can be copied, by handle */
		copySites?: { handle: string; name: string }[];
		oncopy?: (site: string) => void;
	} = $props();

	const may = $derived(ed.may);
	const isNew = $derived(ed.id === null);
	const published = $derived(ed.live?.published ?? false);
	const hasDraft = $derived(!!ed.draft);
	// narrow: copying from another site and the trash under ⋯, the
	// sidebar with the copy is at the end of the page there
	const more: MenuItem[] = $derived([
		...copySites.map(s => ({
			label: t('entries.copyFrom', { site: s.name }),
			icon: 'copy',
			onclick: () => oncopy?.(s.handle),
		})),
		...(may.delete
			? [
					{
						label: t('entries.moveToTrash'),
						icon: 'delete',
						onclick: onremove,
						danger: true,
					},
				]
			: []),
	]);

	// who changed the draft last, and when
	const draftBy = $derived(
		ed.draft?.user
			? t('entries.draftBy', {
					user: ed.draft.user,
					time: time(ed.draft.updatedAt),
				})
			: null,
	);
</script>

<PageTop crumbs={[...crumbs, { label: title || t('entries.new') }]}>
	{#snippet state()}
		{#if !isNew}
			<span class="badge" class:live={published} class:draft={!published}>
				{published ? t('entries.live') : t('entries.notPublished')}
			</span>
		{/if}
		{#if ed.saver.state === 'failed'}
			<span class="badge failed" title={ed.error}>
				{t('entries.notSaved')}
			</span>
		{:else if hasDraft}
			<span class="badge changes" title={t('entries.savedAsDraft')}>
				{ed.saver.state === 'saving'
					? t('common.saving')
					: t('common.draft')}
			</span>
			{#if draftBy}<span class="by">{draftBy}</span>{/if}
		{/if}
	{/snippet}
	{#snippet actions()}
		{#if narrow}
			<Menu items={more} />
		{:else if may.delete}
			<button
				class="icon"
				onclick={onremove}
				title={t('entries.moveToTrash')}
				aria-label={t('entries.moveToTrash')}
			>
				<Icon name="delete" />
			</button>
		{/if}
		{#if hasDraft}
			<button
				onclick={ondiscard}
				disabled={ed.busy || !may.edit}
				title={t('entries.discardTitle')}
			>
				<Icon name="reset" />
				<span class="text">{t('common.discard')}</span>
			</button>
		{/if}
		{#if live !== null}
			<button
				class="toggle"
				class:on={live}
				aria-pressed={live}
				onclick={onlive}
				title={t('entries.liveTitle')}
			>
				<Icon name="live-preview" />
				<span class="text">{t('entries.livePreview')}</span>
			</button>
		{/if}
		<!-- the live preview has its own link to a new tab -->
		{#if url && !live}
			{#if hasDraft}
				<button onclick={onpreview} title={t('entries.previewTitle')}>
					<Icon name="view" />
					<span class="text">{t('common.preview')}</span>
				</button>
			{:else}
				<a
					class="light"
					href={url}
					target="_blank"
					title={t('entries.viewTitle')}
				>
					<Icon name="external" />
					<span class="text">{t('common.view')}</span>
				</a>
			{/if}
		{/if}
		<!-- a role that may only edit leaves its draft for someone to publish -->
		<button
			class="primary"
			onclick={onsave}
			disabled={ed.busy ||
				(isNew ? !may.edit : !hasDraft || !may.publish)}
			title={!isNew && !may.publish
				? t('entries.mayNotPublish')
				: undefined}
		>
			{isNew ? t('common.create') : t('common.publish')}
		</button>
	{/snippet}
</PageTop>

<style lang="scss">
	.by {
		color: var(--subtle);
		font-size: var(--fs-sm);
	}

	.on {
		border-color: var(--accent);
		color: var(--accent);
	}

	@include max-desktop {
		.by {
			display: none;
		}
	}

	// a narrow editor (live preview, a phone): icons with their title
	@mixin compact {
		.text {
			display: none;
		}
	}

	@include container-max(48rem, page) {
		@include compact;
	}

	// in a flyout the bar is the flyout's
	@include max-tablet {
		@include compact;
	}

	// too narrow for the website next to the editor
	@include max-tablet {
		.toggle {
			display: none;
		}
	}
</style>
