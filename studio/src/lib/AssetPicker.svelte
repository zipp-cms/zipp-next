<script lang="ts">
	import { listAssets, uploadAssets, type Asset } from '../api';
	import AssetGrid from './AssetGrid.svelte';

	/** The asset library in a popup, to pick assets or upload new ones. */
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

	$effect(() => {
		const params: Record<string, string> = { search };
		if (images) params.kind = 'image';
		listAssets(params).then(a => (library = a));
	});

	async function upload(files: FileList | null) {
		if (!files?.length) return;
		const added = await uploadAssets(files);
		library = [...added, ...library];
		for (const a of added) onpick(a);
	}
</script>

<div class="overlay" role="presentation" onclick={onclose}>
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<div
		class="panel"
		role="dialog"
		tabindex="-1"
		onclick={e => e.stopPropagation()}
	>
		<header>
			<input placeholder="Search files" bind:value={search} />
			<label class="upload">
				Upload
				<input
					type="file"
					{multiple}
					accept={images ? 'image/*' : undefined}
					onchange={e => upload(e.currentTarget.files)}
				/>
			</label>
			<button class="primary" onclick={onclose}>Done</button>
		</header>
		<AssetGrid assets={library} {selected} {onpick} />
	</div>
</div>

<style lang="scss">
	.overlay {
		position: fixed;
		inset: 0;
		z-index: 10;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.6);
	}
	.panel {
		width: min(52rem, 94vw);
		height: min(40rem, 90vh);
		overflow: auto;
		padding: 1rem;
		background: white;
		box-shadow: var(--shadow);

		header {
			display: flex;
			gap: 0.5rem;
			margin-bottom: 1rem;
		}
	}
	.upload {
		cursor: pointer;
		padding: 0.45rem 0.9rem;
		border: 1px solid var(--dark);
		border-radius: 2px;
		font-size: 14px;
		white-space: nowrap;

		input {
			display: none;
		}
	}
</style>
