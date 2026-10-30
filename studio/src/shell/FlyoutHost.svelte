<script lang="ts">
	import { listAssets, type Asset } from '../assets/api';
	import { type Schema } from '../core/schema';
	import Edit from '../entries/edit/Edit.svelte';
	import AssetDetail from '../assets/AssetDetail.svelte';
	import Flyout from '../ui/Flyout.svelte';
	import { closeFlyout, flyout } from '../core/flyout.svelte';

	/** The entry or asset a field opened, over the page. */
	let { schema }: { schema: Schema } = $props();

	let asset = $state<Asset | null>(null);
	$effect(() => {
		const open = flyout.open;
		asset = null;
		if (open?.kind !== 'asset') return;
		listAssets({ ids: String(open.id) }).then(a => (asset = a[0] ?? null));
	});
</script>

{#if flyout.open?.kind === 'entry'}
	<Flyout title="Entry" onclose={closeFlyout}>
		{#key flyout.open.id}
			<Edit
				{schema}
				id={flyout.open.id}
				section={null}
				onclose={closeFlyout}
			/>
		{/key}
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
