<script lang="ts">
	import Icon from '../../lib/Icon.svelte';

	/** The title of the entry, its state and what can be done with it. */
	let {
		title,
		published,
		hasDraft,
		saving,
		url,
		busy,
		isNew,
		canDelete,
		onpreview,
		ondiscard,
		onremove,
		onsave,
	}: {
		title: string;
		published: boolean;
		hasDraft: boolean;
		saving: boolean;
		url: string | null;
		busy: boolean;
		isNew: boolean;
		canDelete: boolean;
		onpreview: () => void;
		ondiscard: () => void;
		onremove: () => void;
		onsave: () => void;
	} = $props();
</script>

<div class="page-top">
	<h1>
		{title || 'New entry'}
		<span class="badge" class:draft={!published}>
			{published ? 'Published' : 'Not published'}
		</span>
		{#if hasDraft}
			<span class="badge changes" title="Saved as draft, not live yet">
				{saving ? 'Saving…' : 'Unpublished changes'}
			</span>
		{/if}
	</h1>
	{#if url}
		{#if hasDraft}
			<button class="light" onclick={onpreview}>
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
	{#if hasDraft}
		<button class="light" onclick={ondiscard} disabled={busy}>
			Discard
		</button>
	{/if}
	{#if canDelete}
		<button class="dark" onclick={onremove}>
			<Icon name="delete" />
			Delete
		</button>
	{/if}
	<button
		class="dark"
		onclick={onsave}
		disabled={busy || (!isNew && !hasDraft)}
	>
		<Icon name="save" />
		{isNew ? 'Create' : 'Publish'}
	</button>
</div>

<style>
	h1 {
		display: flex;
		align-items: center;
		gap: 10px;
	}

	.changes {
		background: #d98e04;
	}
</style>
