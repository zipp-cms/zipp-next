<script lang="ts">
	import {
		assetUrl,
		deleteAsset,
		updateAsset,
		type Asset,
		type Schema,
	} from '../api';
	import { canAssets } from './access';

	/**
	 * A file with what editors set on it: title, alt text and the focal
	 * point an image is cropped around, a click on the image sets it.
	 */
	let {
		schema,
		asset,
		onsaved,
		ondeleted,
	}: {
		schema: Schema;
		asset: Asset;
		onsaved: (a: Asset) => void;
		ondeleted: () => void;
	} = $props();

	const canEdit = $derived(canAssets(schema, 'edit'));
	const canDelete = $derived(canAssets(schema, 'delete'));

	// svelte-ignore state_referenced_locally
	let title = $state(asset.title);
	// svelte-ignore state_referenced_locally
	let alt = $state(asset.alt);
	// svelte-ignore state_referenced_locally
	let focal = $state({ x: asset.focalX, y: asset.focalY });
	let error = $state('');

	function setFocal(e: MouseEvent) {
		const r = (e.currentTarget as HTMLElement).getBoundingClientRect();
		focal = {
			x: (e.clientX - r.left) / r.width,
			y: (e.clientY - r.top) / r.height,
		};
	}

	async function save() {
		try {
			onsaved(
				await updateAsset(asset.id, {
					title,
					alt,
					focalX: focal.x,
					focalY: focal.y,
				}),
			);
		} catch (e: any) {
			error = e.message;
		}
	}

	async function remove() {
		if (!confirm(`Delete ${asset.filename}?`)) return;
		await deleteAsset(asset.id);
		ondeleted();
	}
</script>

<div class="detail">
	{#if asset.kind === 'image'}
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<div
			class="focal"
			role="button"
			tabindex="0"
			onclick={e => canEdit && setFocal(e)}
		>
			<img src={assetUrl(asset)} {alt} />
			<span
				class="point"
				style:left="{focal.x * 100}%"
				style:top="{focal.y * 100}%"
			></span>
		</div>
		{#if canEdit}
			<p class="muted">Click on the image to set the focal point.</p>
		{/if}
	{/if}
	<div class="card fields">
		<p class="file">
			<a href={assetUrl(asset)} target="_blank">{asset.filename}</a>
			<span class="muted">
				{Math.round(asset.size / 1024)} KB
				{#if asset.width}· {asset.width} × {asset.height}{/if}
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
		{#if error}<p class="error">{error}</p>{/if}
		<div class="actions">
			{#if canDelete}
				<button type="button" class="danger" onclick={remove}>
					Delete
				</button>
			{/if}
			{#if canEdit}
				<button type="button" class="primary" onclick={save}>
					Save
				</button>
			{/if}
		</div>
	</div>
</div>

<style lang="scss">
	.detail {
		display: flex;
		flex-direction: column;
		gap: 12px;
		padding-top: 20px;
	}

	.focal {
		position: relative;
		align-self: center;
		cursor: crosshair;
		line-height: 0;

		img {
			display: block;
			max-width: 100%;
			max-height: 60vh;
			border-radius: 6px;
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

	.fields {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	label {
		display: flex;
		flex-direction: column;
		gap: 5px;
		font-size: 12px;
		font-weight: 600;
	}

	.file {
		margin: 0;
	}

	.muted {
		margin: 0;
		color: var(--muted);
		font-size: 12px;
		text-align: center;
	}

	.file .muted {
		margin-left: 6px;
	}

	.actions {
		display: flex;
		justify-content: space-between;
	}
</style>
