<script lang="ts">
	import PageTop from '../lib/PageTop.svelte';
	import { listAssets, uploadAssets, type Asset, type Schema } from '../api';
	import { canAssets } from '../lib/access';
	import AssetGrid from '../lib/AssetGrid.svelte';
	import AssetDetail from '../lib/AssetDetail.svelte';
	import SearchInput from '../lib/SearchInput.svelte';
	import UploadButton from '../lib/UploadButton.svelte';

	let { schema }: { schema: Schema } = $props();
	const canEdit = $derived(canAssets(schema, 'edit'));

	let assets = $state<Asset[]>([]);
	let search = $state('');
	let loaded = $state(false);
	let error = $state('');
	let uploading = $state(false);
	let current = $state<Asset | null>(null);

	function load() {
		const q = search;
		listAssets({ search: q })
			.then(a => {
				if (q !== search) return;
				assets = a;
				loaded = true;
			})
			.catch(e => (error = e.message));
	}

	$effect(load);

	async function upload(files: FileList | null) {
		if (!files?.length) return;
		uploading = true;
		try {
			await uploadAssets(files);
			load();
		} catch (e: any) {
			error = e.message;
		} finally {
			uploading = false;
		}
	}

	function open(a: Asset) {
		current = a;
	}

	function ondrop(e: DragEvent) {
		e.preventDefault();
		if (canEdit) upload(e.dataTransfer?.files ?? null);
	}
</script>

<div
	class="page"
	role="region"
	aria-label="Assets"
	ondragover={e => e.preventDefault()}
	{ondrop}
>
	<PageTop title="Assets">
		{#snippet actions()}
			{#if canEdit}
				<UploadButton onfiles={upload} multiple primary>
					{uploading ? 'Uploading…' : 'Upload'}
				</UploadButton>
			{/if}
		{/snippet}
	</PageTop>

	{#if error}<p class="error">{error}</p>{/if}

	<div class="card">
		<div class="search">
			<SearchInput bind:value={search} placeholder="Search files" />
		</div>
		<AssetGrid {assets} loading={!loaded} onpick={open} />
	</div>
</div>

{#if current}
	{#key current.id}
		<AssetDetail
			{schema}
			asset={current}
			onsaved={a => {
				assets = assets.map(x => (x.id === a.id ? a : x));
				current = null;
			}}
			ondeleted={() => {
				current = null;
				load();
			}}
			onclose={() => {
				// a replaced file has a new thumbnail
				current = null;
				load();
			}}
		/>
	{/key}
{/if}

<style lang="scss">
	.page {
		min-height: 80vh;
	}

	.search {
		display: flex;
		justify-content: flex-end;
		margin-bottom: 16px;

		:global(.search) {
			width: 340px;
		}
	}
</style>
