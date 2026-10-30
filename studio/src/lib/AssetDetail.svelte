<script lang="ts">
	import {
		assetUrl,
		assetUsage,
		deleteAsset,
		replaceAsset,
		updateAsset,
		type Asset,
		type AssetUsage,
		type Edits,
		type Schema,
	} from '../api';
	import { canAssets } from './access';
	import { time } from './format';
	import Icon from './Icon.svelte';
	import ImageEditor from './ImageEditor.svelte';
	import Thumb from './Thumb.svelte';
	import { closeFlyout } from './flyout.svelte';
	import { entryPath } from '../route.svelte';

	/**
	 * A file like Craft's asset panel: the image editor, title, alt text
	 * and filename, what it is and who uploaded it, download, copy the
	 * url, replace the file, and where it is used.
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

	const NONE: Edits = {
		rotate: 0,
		flipH: false,
		flipV: false,
		crop: null,
		resize: null,
		canvas: null,
	};

	// svelte-ignore state_referenced_locally
	let current = $state(asset);
	// svelte-ignore state_referenced_locally
	let title = $state(asset.title);
	// svelte-ignore state_referenced_locally
	let alt = $state(asset.alt);
	// svelte-ignore state_referenced_locally
	let filename = $state(asset.filename);
	// svelte-ignore state_referenced_locally
	let focal = $state({ x: asset.focalX, y: asset.focalY });
	// svelte-ignore state_referenced_locally
	let edits = $state<Edits>({ ...NONE, ...(asset.edits ?? {}) });
	let error = $state('');
	let busy = $state(false);
	let copied = $state(false);
	let usage = $state<AssetUsage[] | null>(null);

	$effect(() => {
		assetUsage(current.id)
			.then(u => (usage = u))
			.catch(() => (usage = []));
	});

	const unchanged = (e: Edits) => JSON.stringify(e) === JSON.stringify(NONE);
	const kb = (n: number) =>
		n > 1024 * 1024
			? `${(n / 1024 / 1024).toFixed(1)} MB`
			: `${Math.round(n / 1024)} KB`;

	async function save() {
		busy = true;
		error = '';
		try {
			onsaved(
				await updateAsset(current.id, {
					title,
					alt,
					filename,
					focalX: focal.x,
					focalY: focal.y,
					edits: unchanged(edits) ? null : $state.snapshot(edits),
				}),
			);
		} catch (e: any) {
			error = e.message;
		} finally {
			busy = false;
		}
	}

	async function remove() {
		const used = usage?.length
			? ` It is used in ${usage.length} ${usage.length === 1 ? 'entry' : 'entries'}.`
			: '';
		if (!confirm(`Delete ${current.filename}?${used}`)) return;
		await deleteAsset(current.id);
		ondeleted();
	}

	async function replace(files: FileList | null) {
		const file = files?.[0];
		if (!file) return;
		busy = true;
		error = '';
		try {
			current = await replaceAsset(current.id, file);
			filename = current.filename;
			focal = { x: current.focalX, y: current.focalY };
			edits = { ...NONE };
		} catch (e: any) {
			error = e.message;
		} finally {
			busy = false;
		}
	}

	async function copyUrl() {
		await navigator.clipboard.writeText(
			location.origin + assetUrl(current),
		);
		copied = true;
		setTimeout(() => (copied = false), 1500);
	}

	const sectionName = (h: string) =>
		schema.sections.find(s => s.handle === h)?.name ?? h;
</script>

<div class="detail">
	<div class="main">
		{#if current.kind === 'image'}
			{#key current.filename + current.size}
				<ImageEditor
					src={assetUrl(current, false, true)}
					bind:edits
					bind:focal
					editable={canEdit}
				/>
			{/key}
		{:else}
			<div class="file"><Thumb asset={current} /></div>
		{/if}
	</div>

	<aside>
		<div class="card fields">
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
			<label>
				Filename
				<input bind:value={filename} disabled={!canEdit} />
			</label>
			{#if error}<p class="error">{error}</p>{/if}
			{#if canEdit}
				<button
					type="button"
					class="primary"
					disabled={busy}
					onclick={save}
				>
					Save
				</button>
			{/if}
		</div>

		<div class="card">
			<dl>
				<dt>Size</dt>
				<dd>{kb(current.size)}</dd>
				{#if current.width}
					<dt>Dimensions</dt>
					<dd>
						{current.width} × {current.height}
						{#if current.edits}
							<span class="muted">
								uploaded {current.originalWidth} × {current.originalHeight}
							</span>
						{/if}
					</dd>
				{/if}
				<dt>Type</dt>
				<dd>
					{current.mime}
					{#if current.edits?.canvas?.background === 'transparent' && current.mime !== 'image/png'}
						<span class="muted">delivered as PNG</span>
					{/if}
				</dd>
				<dt>Uploaded</dt>
				<dd>
					{time(current.createdAt)}
					{#if current.uploadedBy}
						<span class="muted">by {current.uploadedBy}</span>
					{/if}
				</dd>
			</dl>
			<div class="actions">
				<a
					class="light"
					href={assetUrl(current, false, true)}
					download={current.filename}
				>
					<Icon name="download" size={14} /> Download
				</a>
				<button type="button" onclick={copyUrl}>
					<Icon name="copy-url" size={14} />
					{copied ? 'Copied' : 'Copy URL'}
				</button>
				{#if canEdit}
					<label class="light replace">
						<Icon name="replace" size={14} /> Replace file
						<input
							type="file"
							onchange={e => replace(e.currentTarget.files)}
						/>
					</label>
				{/if}
			</div>
		</div>

		<div class="card">
			<h3>Used in</h3>
			{#if usage === null}
				<p class="muted">Loading…</p>
			{:else}
				<ul>
					{#each usage as u (u.id)}
						<li>
							<a
								href="/admin{entryPath(u.section, u.id)}"
								onclick={closeFlyout}
							>
								{u.title}
							</a>
							<span class="muted">
								{sectionName(u.section)} · {u.sites
									.map(s => s.toUpperCase())
									.join(' ')}
							</span>
						</li>
					{:else}
						<li class="muted">Not used in any entry</li>
					{/each}
				</ul>
			{/if}
		</div>

		{#if canDelete}
			<button type="button" class="danger delete" onclick={remove}>
				Delete the file
			</button>
		{/if}
	</aside>
</div>

<style lang="scss">
	.detail {
		display: grid;
		grid-template-columns: minmax(0, 1fr) 280px;
		gap: 20px;
		align-items: start;
		padding-top: 16px;
	}

	@media (max-width: 900px) {
		.detail {
			grid-template-columns: 1fr;
		}
	}

	.file {
		width: 240px;
	}

	aside {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.card {
		display: flex;
		flex-direction: column;
		gap: 12px;
		padding: 12px;
	}

	label {
		display: flex;
		flex-direction: column;
		gap: 5px;
		font-size: 12px;
		font-weight: 600;
	}

	dl {
		display: grid;
		grid-template-columns: auto 1fr;
		gap: 4px 12px;
		margin: 0;
		font-size: 12px;
	}

	dt {
		color: var(--muted);
	}

	dd {
		margin: 0;
		word-break: break-word;
	}

	.muted {
		color: var(--subtle);
		font-size: 12px;
	}

	.actions {
		display: flex;
		flex-wrap: wrap;
		gap: 6px;
	}

	.replace {
		flex-direction: row;
		align-items: center;
		cursor: pointer;

		input {
			display: none;
		}
	}

	h3 {
		margin: 0;
		font-size: 12px;
		font-weight: 600;
	}

	ul {
		display: flex;
		flex-direction: column;
		gap: 6px;
		margin: 0;
		padding: 0;
		list-style: none;
		font-size: 13px;

		li {
			display: flex;
			flex-direction: column;
		}
	}

	.delete {
		align-self: flex-start;
		border-color: transparent;
		background: none;
	}
</style>
