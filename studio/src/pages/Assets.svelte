<script lang="ts">
	import {
		assetUrl,
		deleteAsset,
		listAssets,
		updateAsset,
		uploadAssets,
		type Asset,
		type Schema,
	} from '../api';
	import { canAssets } from '../lib/access';
	import AssetGrid from '../lib/AssetGrid.svelte';
	import Icon from '../lib/Icon.svelte';

	let { schema }: { schema: Schema } = $props();
	const canEdit = $derived(canAssets(schema, 'edit'));
	const canDelete = $derived(canAssets(schema, 'delete'));

	let assets = $state<Asset[]>([]);
	let search = $state('');
	let error = $state('');
	let uploading = $state(false);
	let current = $state<Asset | null>(null);
	let title = $state('');
	let alt = $state('');
	let focal = $state({ x: 0.5, y: 0.5 });

	function load() {
		listAssets({ search })
			.then(a => (assets = a))
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
		title = a.title;
		alt = a.alt;
		focal = { x: a.focalX, y: a.focalY };
	}

	/** A click on the image sets the focal point. */
	function setFocal(e: MouseEvent) {
		const r = (e.currentTarget as HTMLElement).getBoundingClientRect();
		focal = {
			x: (e.clientX - r.left) / r.width,
			y: (e.clientY - r.top) / r.height,
		};
	}

	async function save() {
		if (!current) return;
		const a = await updateAsset(current.id, {
			title,
			alt,
			focalX: focal.x,
			focalY: focal.y,
		});
		assets = assets.map(x => (x.id === a.id ? a : x));
		current = null;
	}

	async function remove() {
		if (!current || !confirm(`Delete ${current.filename}?`)) return;
		await deleteAsset(current.id);
		current = null;
		load();
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
	<div class="page-top">
		<h1>Assets</h1>
		{#if canEdit}
			<label class="upload">
				<Icon name="add" />
				{uploading ? 'Uploading…' : 'Upload'}
				<input
					type="file"
					multiple
					onchange={e => upload(e.currentTarget.files)}
				/>
			</label>
		{/if}
	</div>

	{#if error}<p class="error">{error}</p>{/if}

	<div class="card">
		<div class="search">
			<input placeholder="Search files" bind:value={search} />
		</div>
		<AssetGrid {assets} onpick={open} />
	</div>
</div>

{#if current}
	<div class="overlay" role="presentation" onclick={() => (current = null)}>
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<div
			class="panel"
			role="dialog"
			tabindex="-1"
			onclick={e => e.stopPropagation()}
		>
			{#if current.kind === 'image'}
				{#if canEdit}
					<p class="muted">
						Click on the image to set the focal point.
					</p>
				{/if}
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<div
					class="focal"
					role="button"
					tabindex="0"
					onclick={e => canEdit && setFocal(e)}
				>
					<img src={assetUrl(current)} {alt} />
					<span
						class="point"
						style:left="{focal.x * 100}%"
						style:top="{focal.y * 100}%"
					></span>
				</div>
			{/if}
			<p>
				<a href={assetUrl(current)} target="_blank">
					{current.filename}
				</a>
				<span class="muted">
					{Math.round(current.size / 1024)} KB
					{#if current.width}· {current.width} × {current.height}{/if}
				</span>
			</p>
			<label>
				Title
				<input
					bind:value={title}
					placeholder="From the filename"
					disabled={!canEdit}
				/>
			</label>
			<label>
				Alt text
				<input
					bind:value={alt}
					placeholder="What the image shows"
					disabled={!canEdit}
				/>
			</label>
			<div class="actions">
				{#if canDelete}
					<button class="danger" onclick={remove}>Delete</button>
				{/if}
				{#if canEdit}
					<button class="primary" onclick={save}>Save</button>
				{/if}
			</div>
		</div>
	</div>
{/if}

<style lang="scss">
	.page {
		min-height: 80vh;
	}

	// looks like the main button, it is a label for the file input
	.upload {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 6px 12px;
		font-size: 13px;
		font-weight: 500;
		cursor: pointer;
		border-radius: 5px;
		background: var(--accent);
		color: white;

		&:hover {
			background: var(--accent-hover);
		}

		input {
			display: none;
		}
	}

	.search {
		display: flex;
		justify-content: flex-end;
		margin-bottom: 16px;

		input {
			width: 340px;
		}
	}

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
		width: min(40rem, 92vw);
		max-height: 92vh;
		overflow: auto;
		padding: 1.25rem;
		background: white;
		box-shadow: var(--shadow);
		display: flex;
		flex-direction: column;
		gap: 0.75rem;

		label {
			display: flex;
			flex-direction: column;
			gap: 0.3rem;
		}
	}

	// sized by the image, so a click maps to the image
	.focal {
		position: relative;
		align-self: center;
		cursor: crosshair;
		line-height: 0;

		img {
			display: block;
			max-width: 100%;
			max-height: 60vh;
		}
	}

	.point {
		position: absolute;
		width: 1.4rem;
		height: 1.4rem;
		margin: -0.7rem 0 0 -0.7rem;
		border: 3px solid white;
		border-radius: 50%;
		box-shadow: 0 0 0 2px rgba(0, 0, 0, 0.5);
		pointer-events: none;
	}

	.actions {
		display: flex;
		justify-content: space-between;
	}

	.muted {
		color: var(--muted);
		margin: 0;
	}
</style>
