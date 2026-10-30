<script module lang="ts">
	import { listAssets as list } from './api';

	/** The newest assets. */
	export async function loadProps() {
		return { first: await list({ search: '' }) };
	}
</script>

<script lang="ts">
	import { untrack } from 'svelte';
	import PageTop from '../ui/PageTop.svelte';
	import { listAssets, uploadAssets, type Asset } from './api';
	import { type Schema } from '../core/schema';
	import { canAssets } from '../core/access';
	import AssetGrid from './AssetGrid.svelte';
	import AssetDetail from './AssetDetail.svelte';
	import SearchInput, { SEARCH_DELAY_MS } from '../ui/SearchInput.svelte';
	import { Busy } from '../core/busy.svelte';
	import UploadButton from '../ui/UploadButton.svelte';

	let { schema, first }: { schema: Schema; first: Asset[] } = $props();
	const canEdit = $derived(canAssets(schema, 'edit'));

	let assets = $state<Asset[]>(untrack(() => first));
	let search = $state('');
	let error = $state('');
	const uploading = new Busy();
	let current = $state<Asset | null>(null);

	function load() {
		const q = search;
		listAssets({ search: q })
			.then(a => {
				if (q === search) assets = a;
			})
			.catch(e => (error = e.message));
	}

	// the first assets came with the page
	let searched = '';
	// a moment after the last key, not with every one
	$effect(() => {
		if (search === searched) return;
		const wait = setTimeout(() => {
			searched = search;
			load();
		}, SEARCH_DELAY_MS);
		return () => clearTimeout(wait);
	});

	async function upload(files: FileList | null) {
		if (!files?.length) return;
		await uploading.run(() => uploadAssets(files));
		load();
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
					{uploading.busy ? 'Uploading…' : 'Upload'}
				</UploadButton>
			{/if}
		{/snippet}
	</PageTop>

	{#if error || uploading.error}
		<p class="error">{error || uploading.error}</p>
	{/if}

	<div class="card">
		<div class="search">
			<SearchInput bind:value={search} placeholder="Search files" />
		</div>
		<AssetGrid {assets} onpick={open} />
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
		margin-bottom: 1rem;

		:global(.search) {
			width: 21.25rem;
		}
	}
</style>
