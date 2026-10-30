<script lang="ts">
	import { untrack } from 'svelte';
	import { assetUrl, transparentEdges, type Asset } from './api';
	import { type Schema } from '../core/schema';
	import { entryPath } from '../core/router.svelte';
	import { canAssets } from '../core/access';
	import { AssetForm } from './assetForm.svelte';
	import Flyout from '../ui/Flyout.svelte';
	import { closeFlyout } from '../core/flyout.svelte';
	import { fileSize, time, plural } from '../core/format';
	import Icon from '../ui/Icon.svelte';
	import ImageEditor from './image/ImageEditor.svelte';
	import { sectionName } from '../core/schema';
	import Thumb from './Thumb.svelte';
	import UploadButton from '../ui/UploadButton.svelte';

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
	// the flyout is keyed by the asset
	const form = untrack(() => new AssetForm(asset));
	const current = $derived(form.current);
	let copied = $state(false);

	/** Whether the changes may go: none, or the editor says so. */
	const mayLeave = () => !form.dirty || confirm('Discard the changes?');

	function cancel() {
		if (mayLeave()) onclose();
	}

	async function save() {
		const saved = await form.save();
		if (saved) onsaved(saved);
	}

	async function remove() {
		const n = form.usage?.length ?? 0;
		const used = n
			? ` It is used in ${plural(n, 'entry', 'entries')}.`
			: '';
		if (!confirm(`Delete ${current.filename}?${used}`)) return;
		if (await form.remove()) ondeleted();
	}

	async function copyUrl() {
		await navigator.clipboard.writeText(
			location.origin + assetUrl(current),
		);
		copied = true;
		setTimeout(() => (copied = false), 1500);
	}
</script>

{#snippet actions()}
	<button type="button" onclick={cancel}>Cancel</button>
	<button type="button" class="primary" disabled={form.busy} onclick={save}>
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
						bind:edits={form.edits}
						bind:focal={form.focal}
						editable={canEdit}
					/>
				{/key}
			{:else}
				<div class="file"><Thumb asset={current} /></div>
			{/if}
		</div>

		<aside>
			<div class="card fields">
				<label class="stacked">
					Title
					<input
						bind:value={form.title}
						placeholder="From the filename"
						disabled={!canEdit}
					/>
				</label>
				<label class="stacked">
					Alt text
					<input
						bind:value={form.alt}
						placeholder="What the image shows"
						disabled={!canEdit}
					/>
				</label>
				<label class="stacked">
					Filename
					<input bind:value={form.filename} disabled={!canEdit} />
				</label>
				{#if form.error}<p class="error">{form.error}</p>{/if}
			</div>

			<div class="card">
				<dl>
					<dt>Size</dt>
					<dd>{fileSize(current.size)}</dd>
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
						<UploadButton
							onfiles={f => form.replace(f[0])}
							icon="replace"
						>
							Replace file
						</UploadButton>
					{/if}
				</div>
			</div>

			<div class="card">
				<h3>Used in</h3>
				{#if form.usage === null}
					<p class="empty">Loading…</p>
				{:else}
					<ul class="plain usage">
						{#each form.usage as u (u.id)}
							<li>
								<a
									href={entryPath(u.section, u.id)}
									onclick={e => {
										if (mayLeave()) closeFlyout();
										else e.preventDefault();
									}}
								>
									{u.title}
								</a>
								<span class="note">
									{sectionName(schema, u.section)} · {u.sites
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
		grid-template-columns: minmax(0, 1fr) 17.5rem;
		gap: 1.25rem;
		align-items: start;
		padding-top: 1rem;
	}

	@include max-desktop {
		.detail {
			grid-template-columns: 1fr;
		}
	}

	.file {
		width: 15rem;
	}

	aside {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.card {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		padding: 0.75rem;
	}

	dl {
		display: grid;
		grid-template-columns: auto 1fr;
		gap: 0.25rem 0.75rem;
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
		gap: 0.375rem;
	}

	h3 {
		margin: 0;
		font-size: var(--fs-sm);
		font-weight: 600;
	}

	.usage {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
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
