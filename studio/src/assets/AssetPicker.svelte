<script lang="ts">
	import { listAssets, uploadAssets, type Asset } from './api';
	import AssetGrid from './AssetGrid.svelte';
	import { Busy } from '../core/busy.svelte';
	import Flyout from '../ui/Flyout.svelte';
	import SearchInput from '../ui/SearchInput.svelte';
	import UploadButton from '../ui/UploadButton.svelte';

	/** The asset library in a flyout, to pick assets or upload new ones. */
	let {
		images,
		multiple,
		selected,
		onpick,
		onclose,
	}: {
		/** only images */
		images: boolean;
		multiple: boolean;
		selected: number[];
		onpick: (a: Asset) => void;
		onclose: () => void;
	} = $props();

	let library = $state<Asset[]>([]);
	let search = $state('');
	let loaded = $state(false);

	$effect(() => {
		const q = search;
		const params: Record<string, string> = { search: q };
		if (images) params.kind = 'image';
		listAssets(params)
			.then(a => {
				if (q !== search) return;
				library = a;
				loaded = true;
			})
			.catch(e => (action.error = e.message));
	});

	const action = new Busy();

	async function upload(files: FileList) {
		const added = await action.run(() => uploadAssets(files));
		if (!added) return;
		library = [...added, ...library];
		for (const a of added) onpick(a);
	}

	const what = $derived(
		images
			? multiple
				? 'images'
				: 'an image'
			: multiple
				? 'files'
				: 'a file',
	);
</script>

<Flyout title="Choose {what}" {onclose} narrow>
	{#snippet actions()}
		<UploadButton
			onfiles={upload}
			{multiple}
			accept={images ? 'image/*' : undefined}
		>
			{action.busy ? 'Uploading…' : 'Upload'}
		</UploadButton>
		<button type="button" class="primary" onclick={onclose}>Done</button>
	{/snippet}
	<div class="search">
		<SearchInput bind:value={search} placeholder="Search files" />
	</div>
	{#if action.error}<p class="error">{action.error}</p>{/if}
	<div class="card">
		<AssetGrid assets={library} {selected} loading={!loaded} {onpick} />
	</div>
</Flyout>

<style>
	.search {
		margin: 16px 0 12px;
	}
</style>
