<script lang="ts">
	import PageTop from 'zipp-ui/PageTop.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import { time } from '../../core/format';
	import type { EntryEditor, May } from './entryEditor.svelte';

	/** Where the entry is, its state and what can be done with it. */
	let {
		ed,
		crumbs,
		title,
		url,
		may,
		live,
		onlive,
		onpreview,
		ondiscard,
		onremove,
		onsave,
	}: {
		ed: EntryEditor;
		crumbs: { label: string; href?: string }[];
		title: string;
		url: string | null;
		may: May;
		/** the live preview is on; null: this entry has none */
		live: boolean | null;
		onlive: () => void;
		onpreview: () => void;
		ondiscard: () => void;
		onremove: () => void;
		onsave: () => void;
	} = $props();

	const isNew = $derived(ed.id === null);
	const published = $derived(ed.live?.published ?? false);
	const hasDraft = $derived(!!ed.draft);
	// who changed the draft last, and when
	const draftBy = $derived(
		ed.draft?.user
			? `by ${ed.draft.user}, ${time(ed.draft.updatedAt)}`
			: null,
	);
</script>

<PageTop crumbs={[...crumbs, { label: title || 'New entry' }]}>
	{#snippet state()}
		{#if !isNew}
			<span class="badge" class:live={published} class:draft={!published}>
				{published ? 'Live' : 'Not published'}
			</span>
		{/if}
		{#if ed.draftState === 'failed'}
			<span class="badge failed" title={ed.error}>Not saved</span>
		{:else if hasDraft}
			<span class="badge changes" title="Saved as draft, not live yet">
				{ed.draftState === 'saving' ? 'Saving…' : 'Draft'}
			</span>
			{#if draftBy}<span class="by">{draftBy}</span>{/if}
		{/if}
	{/snippet}
	{#snippet actions()}
		{#if may.delete}
			<button class="icon" onclick={onremove} title="Move to the trash">
				<Icon name="delete" />
			</button>
		{/if}
		{#if hasDraft}
			<button onclick={ondiscard} disabled={ed.busy || !may.edit}>
				Discard
			</button>
		{/if}
		{#if live !== null}
			<button
				class="live"
				class:on={live}
				aria-pressed={live}
				onclick={onlive}
				title="The website next to the editor, with the draft"
			>
				<Icon name="live-preview" />
				Live preview
			</button>
		{/if}
		<!-- the live preview has its own link to a new tab -->
		{#if url && !live}
			{#if hasDraft}
				<button onclick={onpreview}>
					<Icon name="view" />
					Preview
				</button>
			{:else}
				<a class="light" href={url} target="_blank">
					<Icon name="external" />
					View
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
				? 'Your role can edit drafts, someone else publishes them'
				: undefined}
		>
			{isNew ? 'Create' : 'Publish'}
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

	// too narrow for the website next to the editor
	@include max-tablet {
		.live {
			display: none;
		}
	}
</style>
