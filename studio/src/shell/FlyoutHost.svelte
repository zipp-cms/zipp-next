<script lang="ts">
	import { listAssets, type Asset } from '../assets/api';
	import AssetDetail from '../assets/AssetDetail.svelte';
	import { closeFlyout, flyout } from '../core/flyout.svelte';
	import { site } from '../core/router.svelte';
	import { type Schema } from '../core/schema';
	import Edit from '../entries/edit/Edit.svelte';
	import { EntryEditor } from '../entries/edit/entryEditor.svelte';
	import Flyout from '../ui/Flyout.svelte';

	/**
	 * The entry or asset a field opened, over the page: loaded when it
	 * opens, again in another site.
	 */
	let { schema }: { schema: Schema } = $props();

	type Loaded =
		| { kind: 'entry'; editor: EntryEditor }
		| { kind: 'asset'; asset: Asset }
		| { kind: 'error'; message: string };
	let loaded = $state<Loaded | null>(null);

	$effect(() => {
		const open = flyout.open;
		void site.current;
		loaded = null;
		if (!open) return;
		const load: Promise<Loaded> =
			open.kind === 'entry'
				? EntryEditor.open(schema, open.id, null).then(editor => ({
						kind: 'entry',
						editor,
					}))
				: listAssets({ ids: String(open.id) }).then(([asset]) =>
						asset
							? { kind: 'asset', asset }
							: {
									kind: 'error',
									message: 'This file was deleted.',
								},
					);
		load.catch((e): Loaded => ({ kind: 'error', message: e.message }))
			// only for what is still open
			.then(l => {
				if (flyout.open === open) loaded = l;
			});
	});
</script>

{#if loaded?.kind === 'asset'}
	{#key loaded.asset.id}
		<AssetDetail
			{schema}
			asset={loaded.asset}
			onsaved={closeFlyout}
			ondeleted={closeFlyout}
			onclose={closeFlyout}
		/>
	{/key}
{:else if flyout.open}
	<Flyout
		title={flyout.open.kind === 'entry' ? 'Entry' : 'File'}
		onclose={closeFlyout}
	>
		{#if loaded?.kind === 'entry'}
			{#key loaded.editor}
				<Edit {schema} editor={loaded.editor} onclose={closeFlyout} />
			{/key}
		{:else if loaded?.kind === 'error'}
			<p class="error">{loaded.message}</p>
		{:else}
			<p class="empty">Loading…</p>
		{/if}
	</Flyout>
{/if}
