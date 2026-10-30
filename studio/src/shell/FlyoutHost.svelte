<script lang="ts">
	import { listAssets, type Asset } from '../assets/api';
	import { type Schema } from '../core/schema';
	import Edit from '../entries/edit/Edit.svelte';
	import AssetDetail from '../assets/AssetDetail.svelte';
	import Flyout from '../ui/Flyout.svelte';
	import { closeFlyout, flyout } from '../core/flyout.svelte';
	import { EntryEditor } from '../entries/edit/entryEditor.svelte';

	/** The entry or asset a field opened, over the page. */
	let { schema }: { schema: Schema } = $props();

	let asset = $state<Asset | null>(null);
	$effect(() => {
		const open = flyout.open;
		asset = null;
		if (open?.kind !== 'asset') return;
		listAssets({ ids: String(open.id) }).then(a => (asset = a[0] ?? null));
	});

	let editor = $state<EntryEditor | null>(null);
	$effect(() => {
		const open = flyout.open;
		editor = null;
		if (open?.kind !== 'entry') return;
		EntryEditor.open(schema, open.id, null).then(e => {
			if (flyout.open === open) editor = e;
		});
	});
</script>

{#if flyout.open?.kind === 'entry'}
	<Flyout title="Entry" onclose={closeFlyout}>
		{#if editor}
			{#key editor}
				<Edit {schema} {editor} onclose={closeFlyout} />
			{/key}
		{:else}
			<p class="empty">Loading…</p>
		{/if}
	</Flyout>
{:else if flyout.open?.kind === 'asset'}
	{#if asset}
		{#key asset.id}
			<AssetDetail
				{schema}
				{asset}
				onsaved={closeFlyout}
				ondeleted={closeFlyout}
				onclose={closeFlyout}
			/>
		{/key}
	{:else}
		<Flyout title="File" onclose={closeFlyout}>
			<p class="empty">Loading…</p>
		</Flyout>
	{/if}
{/if}
