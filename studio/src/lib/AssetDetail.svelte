<script lang="ts">
	import {
		assetUrl,
		assetUsage,
		deleteAsset,
		replaceAsset,
		updateAsset,
		type Asset,
		type AssetUsage,
		NO_EDITS,
		noEdits,
		transparentEdges,
		type Edits,
		type Schema,
	} from '../api';
	import Flyout from './Flyout.svelte';
	import UploadButton from './UploadButton.svelte';
	import { canAssets } from './access';
	import { time } from './format';
	import Icon from './Icon.svelte';
	import ImageEditor from './image/ImageEditor.svelte';
	import Thumb from './Thumb.svelte';
	import { closeFlyout } from './flyout.svelte';
	import { entryPath } from '../route.svelte';

	/**
	 * A file like Craft's asset panel: the image editor, title, alt text
	 * and filename, what it is and who uploaded it, download, copy the
	 * url, replace the file, and where it is used. In a flyout with Save
	 * and Cancel.
	 */
	let {
		schema,
		asset,
		onsaved,
		ondeleted,
		onclose,
	}: {
		schema: Schema;
		asset: Asset;
		onsaved: (a: Asset) => void;
		ondeleted: () => void;
		onclose: () => void;
	} = $props();

	const canEdit = $derived(canAssets(schema, 'edit'));
	const canDelete = $derived(canAssets(schema, 'delete'));

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
	let edits = $state<Edits>({ ...NO_EDITS, ...(asset.edits ?? {}) });
	let error = $state('');
	let busy = $state(false);
	let copied = $state(false);
	let usage = $state<AssetUsage[] | null>(null);

	$effect(() => {
		assetUsage(current.id)
			.then(u => (usage = u))
			.catch(() => (usage = []));
	});

	const form = () =>
		JSON.stringify({
			title,
			alt,
			filename,
			focal,
			edits: $state.snapshot(edits),
		});
	// what Cancel would throw away
	let initial = $state(form());
	const dirty = $derived(form() !== initial);

	function cancel() {
		if (dirty && !confirm('Discard the changes?')) return;
		onclose();
	}
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
					edits: noEdits(edits) ? null : $state.snapshot(edits),
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

	async function replace(files: FileList) {
		const file = files?.[0];
		if (!file) return;
		busy = true;
		error = '';
		try {
			current = await replaceAsset(current.id, file);
			filename = current.filename;
			focal = { x: current.focalX, y: current.focalY };
			edits = { ...NO_EDITS };
			initial = form();
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

{#snippet actions()}
	<button type="button" onclick={cancel}>Cancel</button>
	<button type="button" class="primary" disabled={busy} onclick={save}>
		<Icon name="save" size={14} /> Save
	</button>
{/snippet}

<Flyout
	title={current.title || current.filename}
	onclose={cancel}
	actions={canEdit ? actions : undefined}
>
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
								<span class="note">
									uploaded {current.originalWidth} × {current.originalHeight}
								</span>
							{/if}
						</dd>
					{/if}
					<dt>Type</dt>
					<dd>
						{current.mime}
						{#if current.edits && transparentEdges(current.edits) && current.mime !== 'image/png'}
							<span class="note">delivered as PNG</span>
						{/if}
					</dd>
					<dt>Uploaded</dt>
					<dd>
						{time(current.createdAt)}
						{#if current.uploadedBy}
							<span class="note">by {current.uploadedBy}</span>
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
						<UploadButton onfiles={replace} icon="replace">
							Replace file
						</UploadButton>
					{/if}
				</div>
			</div>

			<div class="card">
				<h3>Used in</h3>
				{#if usage === null}
					<p class="note">Loading…</p>
				{:else}
					<ul>
						{#each usage as u (u.id)}
							<li>
								<a
									href="/admin{entryPath(u.section, u.id)}"
									onclick={e => {
										if (
											dirty &&
											!confirm('Discard the changes?')
										)
											e.preventDefault();
										else closeFlyout();
									}}
								>
									{u.title}
								</a>
								<span class="note">
									{sectionName(u.section)} · {u.sites
										.map(s => s.toUpperCase())
										.join(' ')}
								</span>
							</li>
						{:else}
							<li class="note">Not used in any entry</li>
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
</Flyout>

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
		font-size: var(--fs-sm);
		font-weight: 600;
	}

	dl {
		display: grid;
		grid-template-columns: auto 1fr;
		gap: 4px 12px;
		margin: 0;
		font-size: var(--fs-sm);
	}

	dt {
		color: var(--muted);
	}

	dd {
		margin: 0;
		word-break: break-word;
	}

	.actions {
		display: flex;
		flex-wrap: wrap;
		gap: 6px;
	}

	h3 {
		margin: 0;
		font-size: var(--fs-sm);
		font-weight: 600;
	}

	ul {
		display: flex;
		flex-direction: column;
		gap: 6px;
		margin: 0;
		padding: 0;
		list-style: none;
		font-size: var(--fs-md);

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
