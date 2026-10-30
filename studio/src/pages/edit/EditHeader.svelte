<script lang="ts">
	import Crumbs from '../../lib/Crumbs.svelte';
	import Icon from '../../lib/Icon.svelte';

	/** Where the entry is, its state and what can be done with it. */
	let {
		crumbs,
		title,
		published,
		hasDraft,
		draftBy,
		saving,
		url,
		busy,
		isNew,
		canEdit,
		canPublish,
		canDelete,
		onpreview,
		ondiscard,
		onremove,
		onsave,
	}: {
		crumbs: { label: string; href?: string }[];
		title: string;
		published: boolean;
		hasDraft: boolean;
		/** who changed the draft last, and when */
		draftBy: string | null;
		saving: boolean;
		url: string | null;
		busy: boolean;
		isNew: boolean;
		canEdit: boolean;
		canPublish: boolean;
		canDelete: boolean;
		onpreview: () => void;
		ondiscard: () => void;
		onremove: () => void;
		onsave: () => void;
	} = $props();
</script>

<div class="page-top">
	<Crumbs items={[...crumbs, { label: title || 'New entry' }]} />
	<span class="state">
		{#if !isNew}
			<span class="badge" class:live={published} class:draft={!published}>
				{published ? 'Live' : 'Not published'}
			</span>
		{/if}
		{#if hasDraft}
			<span class="badge changes" title="Saved as draft, not live yet">
				{saving ? 'Saving…' : 'Draft'}
			</span>
			{#if draftBy}<span class="by">{draftBy}</span>{/if}
		{/if}
	</span>
	<span class="actions">
		{#if canDelete}
			<button class="icon" onclick={onremove} title="Delete in all sites">
				<Icon name="delete" />
			</button>
		{/if}
		{#if hasDraft}
			<button onclick={ondiscard} disabled={busy || !canEdit}>
				Discard
			</button>
		{/if}
		{#if url}
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
			disabled={busy || (isNew ? !canEdit : !hasDraft || !canPublish)}
			title={!isNew && !canPublish
				? 'Your role can edit drafts, someone else publishes them'
				: undefined}
		>
			{isNew ? 'Create' : 'Publish'}
		</button>
	</span>
</div>

<style>
	.state {
		display: flex;
		align-items: center;
		gap: 6px;
		margin-right: auto;
	}

	.by {
		color: var(--subtle);
		font-size: var(--fs-sm);
	}

	@media (max-width: 1100px) {
		.by {
			display: none;
		}
	}

	.actions {
		display: flex;
		align-items: center;
		gap: 8px;
	}
</style>
