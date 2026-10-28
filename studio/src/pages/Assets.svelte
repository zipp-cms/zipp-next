<script lang="ts">
	import {
		assetUrl,
		deleteAsset,
		listAssets,
		updateAsset,
		uploadAssets,
		type Asset,
	} from '../api';
	import AssetGrid from '../fields/AssetGrid.svelte';

	let assets = $state<Asset[]>([]);
	let search = $state('');
	let error = $state('');
	let uploading = $state(false);
	let current = $state<Asset | null>(null);
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
		upload(e.dataTransfer?.files ?? null);
	}
</script>

<div
	class="page"
	role="region"
	aria-label="Assets"
	ondragover={e => e.preventDefault()}
	{ondrop}
>
	<header>
		<h2>Assets</h2>
		<label class="button primary">
			{uploading ? 'Uploading…' : 'Upload'}
			<input
				type="file"
				multiple
				onchange={e => upload(e.currentTarget.files)}
			/>
		</label>
	</header>

	<input placeholder="Search files" bind:value={search} />
	{#if error}<p class="error">{error}</p>{/if}

	<div class="grid">
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
				<p class="muted">Click on the image to set the focal point.</p>
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<div
					class="focal"
					role="button"
					tabindex="0"
					onclick={setFocal}
				>
					<img src={assetUrl(current)} {alt} />
					<span
						class="dot"
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
				Alt text
				<input bind:value={alt} placeholder="What the image shows" />
			</label>
			<div class="actions">
				<button class="danger" onclick={remove}>Delete</button>
				<button class="primary" onclick={save}>Save</button>
			</div>
		</div>
	</div>
{/if}

<style lang="scss">
	.page {
		min-height: 80vh;
	}

	header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.button {
		cursor: pointer;
		padding: 0.45rem 0.9rem;
		border-radius: 0.3rem;
		background: var(--accent);
		color: white;

		input {
			display: none;
		}
	}

	.grid {
		margin-top: 1rem;
	}

	.overlay {
		position: fixed;
		inset: 0;
		z-index: 10;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.35);
	}

	.panel {
		width: min(40rem, 92vw);
		max-height: 92vh;
		overflow: auto;
		padding: 1.25rem;
		background: white;
		border-radius: 0.5rem;
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

	.dot {
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
