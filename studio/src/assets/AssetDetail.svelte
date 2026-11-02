<script lang="ts">
	import { onDestroy, untrack } from 'svelte';
	import {
		ASSET_FIELDS,
		assetUrl,
		isVideo,
		type Asset,
		type Folder,
	} from './api';
	import { root, folderRows } from './folders';
	import { componentOf, siteName, type Schema } from '../core/schema';
	import Fields from '../fields/Fields.svelte';
	import { site } from '../core/router.svelte';
	import { canAssets } from '../core/access';
	import { AssetForm } from './assetForm.svelte';
	import Flyout from 'zipp-ui/Flyout.svelte';
	import { guardLeave } from '../core/leave';
	import { confirmDelete } from './actions';
	import Icon from 'zipp-ui/Icon.svelte';
	import ImageEditor from './image/ImageEditor.svelte';
	import Thumb from './Thumb.svelte';
	import AssetInfo from './AssetInfo.svelte';
	import AssetUsage from './AssetUsage.svelte';
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
	// the alt text of the site the studio edits
	const altLabel = $derived.by(() => {
		if (schema.sites.length < 2) return t('assets.altText');
		return t('assets.altTextOf', { site: siteName(schema, site.current) });
	});
	// a site without its own alt text has the primary site's
	const altPlaceholder = $derived.by(() => {
		const primary = schema.sites[0].handle;
		const fallback = site.current !== primary && form.alt[primary];
		return fallback || t('assets.altPlaceholder');
	});

	/** Whether the changes may go: none, or the editor says so. */
	const mayLeave = () => !form.dirty || confirm(t('assets.confirmDiscard'));

	// leaving the page (a link, Back) and closing the tab ask too
	onDestroy(guardLeave(() => form.dirty));

	function cancel() {
		if (mayLeave()) onclose();
	}

	async function save() {
		const saved = await form.save();
		if (saved) onsaved(saved);
	}

	async function remove() {
		const usage = await form.usageLoaded;
		if (!confirmDelete(current, usage.length)) return;
		if (await form.remove()) ondeleted();
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
			<!-- a replaced file shows new -->
			{#key current.filename + current.size}
				{#if current.kind === 'image'}
					<ImageEditor
						src={assetUrl(current, false, true)}
						bind:edits={form.edits}
						bind:focal={form.focal}
						editable={canEdit}
					/>
				{:else if isVideo(current)}
					<!-- svelte-ignore a11y_media_has_caption -->
					<video
						class="player"
						src={assetUrl(current)}
						controls
						preload="metadata"
					></video>
				{:else if current.mime.startsWith('audio/')}
					<audio
						src={assetUrl(current)}
						controls
						preload="metadata"
					></audio>
				{:else}
					<div class="file"><Thumb asset={current} /></div>
				{/if}
			{/key}
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

			<AssetInfo
				asset={current}
				{canEdit}
				onreplace={f => form.replace(f)}
			/>

			<AssetUsage {schema} usage={form.usage} />

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

	.delete {
		align-self: flex-start;
		border-color: transparent;
		background: none;
	}
</style>
