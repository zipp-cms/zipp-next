<script lang="ts">
	import { untrack } from 'svelte';
	import {
		ASSET_FIELDS,
		assetUrl,
		isVideo,
		root,
		folderRows,
		transparentEdges,
		type Asset,
		type Folder,
	} from './api';
	import { componentOf, type Schema } from '../core/schema';
	import Fields from '../fields/Fields.svelte';
	import { entryPath, site } from '../core/router.svelte';
	import { canAssets } from '../core/access';
	import { AssetForm } from './assetForm.svelte';
	import Flyout from 'zipp-ui/Flyout.svelte';
	import { closeFlyout } from '../core/flyout.svelte';
	import { fileSize, time } from '../core/format';
	import { confirmDelete, copyUrl as copyAssetUrl } from './actions';
	import Icon from 'zipp-ui/Icon.svelte';
	import ImageEditor from './image/ImageEditor.svelte';
	import { sectionName } from '../core/schema';
	import Thumb from './Thumb.svelte';
	import UploadButton from 'zipp-ui/UploadButton.svelte';
	import { t } from '../i18n/index.svelte';

	/**
	 * A file like Craft's asset panel: the image editor or a player,
	 * title, alt text, the asset fields of the schema and filename, what
	 * it is and who uploaded it, download, copy the url, replace the
	 * file, its folder, and where it is used. In a flyout with Save and
	 * Cancel.
	 */
	let {
		schema,
		folders = [],
		asset,
		onsaved,
		ondeleted,
		onclose,
	}: {
		schema: Schema;
		/** to move it into another folder, none: no folder select */
		folders?: Folder[];
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
	const assetFields = $derived(componentOf(schema, ASSET_FIELDS));
	let copied = $state(false);
	// the alt text of the site the studio edits
	const altLabel = $derived.by(() => {
		if (schema.sites.length < 2) return t('assets.altText');
		const name = schema.sites.find(s => s.handle === site.current)?.name;
		return t('assets.altTextOf', { site: name ?? '' });
	});
	// a site without its own alt text has the primary site's
	const altPlaceholder = $derived.by(() => {
		const primary = schema.sites[0].handle;
		const fallback = site.current !== primary && form.alt[primary];
		return fallback || t('assets.altPlaceholder');
	});

	/** Whether the changes may go: none, or the editor says so. */
	const mayLeave = () => !form.dirty || confirm(t('assets.confirmDiscard'));

	function cancel() {
		if (mayLeave()) onclose();
	}

	async function save() {
		const saved = await form.save();
		if (saved) onsaved(saved);
	}

	async function remove() {
		if (!confirmDelete(current, form.usage?.length ?? 0)) return;
		if (await form.remove()) ondeleted();
	}

	async function copyUrl() {
		await copyAssetUrl(current);
		copied = true;
		setTimeout(() => (copied = false), 1500);
	}
</script>

{#snippet actions()}
	<button type="button" onclick={cancel}>{t('common.cancel')}</button>
	<button type="button" class="primary" disabled={form.busy} onclick={save}>
		<Icon name="save" size={14} />
		{t('common.save')}
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
			{:else if isVideo(current)}
				{#key current.filename + current.size}
					<!-- svelte-ignore a11y_media_has_caption -->
					<video
						class="player"
						src={assetUrl(current)}
						controls
						preload="metadata"
					></video>
				{/key}
			{:else if current.mime.startsWith('audio/')}
				{#key current.filename + current.size}
					<audio
						src={assetUrl(current)}
						controls
						preload="metadata"
					></audio>
				{/key}
			{:else}
				<div class="file"><Thumb asset={current} /></div>
			{/if}
		</div>

		<aside>
			<div class="card fields">
				<label class="stacked">
					{t('common.title')}
					<input
						bind:value={form.title}
						placeholder={t('assets.titlePlaceholder')}
						disabled={!canEdit}
					/>
				</label>
				<label class="stacked">
					{altLabel}
					<input
						bind:value={form.alt[site.current]}
						placeholder={altPlaceholder}
						disabled={!canEdit}
					/>
				</label>
				{#if assetFields?.fields.length}
					<!-- a disabled fieldset disables every input in it -->
					<fieldset disabled={!canEdit}>
						<Fields
							{schema}
							component={assetFields}
							values={form.values}
							errors={form.fields}
							refs={{}}
						/>
					</fieldset>
				{/if}
				<label class="stacked">
					{t('assets.filename')}
					<input bind:value={form.filename} disabled={!canEdit} />
				</label>
				{#if folders.length}
					<label class="stacked">
						{t('assets.folder')}
						<select bind:value={form.folder} disabled={!canEdit}>
							<option value={null}>
								{t('assets.noFolder', { root: root() })}
							</option>
							{#each folderRows(folders) as r (r.folder.id)}
								<option value={r.folder.id}>{r.path}</option>
							{/each}
						</select>
					</label>
				{/if}
				{#if form.error}<p class="error">{form.error}</p>{/if}
			</div>

			<div class="card">
				<dl>
					<dt>{t('assets.size')}</dt>
					<dd>{fileSize(current.size)}</dd>
					{#if current.width}
						<dt>{t('assets.dimensions')}</dt>
						<dd>
							{current.width} × {current.height}
							{#if current.edits}
								<span class="note">
									{t('assets.uploadedSize', {
										width: current.originalWidth ?? '',
										height: current.originalHeight ?? '',
									})}
								</span>
							{/if}
						</dd>
					{/if}
					<dt>{t('assets.type')}</dt>
					<dd>
						{current.mime}
						{#if current.edits && transparentEdges(current.edits) && current.mime !== 'image/png'}
							<span class="note">
								{t('assets.deliveredAsPng')}
							</span>
						{/if}
					</dd>
					<dt>{t('assets.uploaded')}</dt>
					<dd>
						{time(current.createdAt)}
						{#if current.uploadedBy}
							<span class="note">
								{t('assets.uploadedBy', {
									email: current.uploadedBy,
								})}
							</span>
						{/if}
					</dd>
				</dl>
				<div class="actions">
					<a
						class="light"
						href={assetUrl(current, false, true)}
						download={current.filename}
					>
						<Icon name="download" size={14} />
						{t('common.download')}
					</a>
					<button type="button" onclick={copyUrl}>
						<Icon name="copy-url" size={14} />
						{copied ? t('common.copied') : t('assets.copyUrl')}
					</button>
					{#if canEdit}
						<UploadButton
							onfiles={f => form.replace(f[0])}
							icon="replace"
						>
							{t('assets.replaceFile')}
						</UploadButton>
					{/if}
				</div>
			</div>

			<div class="card">
				<h3>{t('assets.usedIn')}</h3>
				{#if form.usage === null}
					<p class="empty">{t('common.loading')}</p>
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
							<li class="note">{t('assets.notUsed')}</li>
						{/each}
					</ul>
				{/if}
			</div>

			{#if canDelete}
				<button type="button" class="danger delete" onclick={remove}>
					{t('assets.deleteFile')}
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
			grid-template-columns: minmax(0, 1fr);
		}
	}

	.file {
		width: 15rem;
	}

	.player {
		display: block;
		width: 100%;
		max-height: 70vh;
		background: var(--text);
		border-radius: var(--radius);
	}

	audio {
		width: 100%;
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
