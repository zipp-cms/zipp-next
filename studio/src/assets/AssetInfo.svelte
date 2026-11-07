<script lang="ts">
	import { type Asset } from './api';
	import { transparentEdges } from './image/edits';
	import { fileSize, time } from '../core/format';
	import { copyUrl, downloadOriginal } from './actions';
	import Icon from 'zipp-ui/Icon.svelte';
	import UploadButton from 'zipp-ui/UploadButton.svelte';
	import { t } from '../i18n/index.svelte';

	/**
	 * What a file is, as saved: size, type and who uploaded it; download,
	 * copy its url, replace it.
	 */
	let {
		asset,
		canEdit,
		onreplace,
	}: {
		asset: Asset;
		canEdit: boolean;
		onreplace: (file: File) => void;
	} = $props();

	let copied = $state(false);

	async function copy() {
		await copyUrl(asset);
		copied = true;
		setTimeout(() => (copied = false), 1500);
	}
</script>

<div class="card">
	<dl>
		<dt>{t('assets.size')}</dt>
		<dd>{fileSize(asset.size)}</dd>
		{#if asset.width}
			<dt>{t('assets.dimensions')}</dt>
			<dd>
				{asset.width} × {asset.height}
				{#if asset.edits}
					<span class="note">
						{t('assets.uploadedSize', {
							width: asset.originalWidth ?? '',
							height: asset.originalHeight ?? '',
						})}
					</span>
				{/if}
			</dd>
		{/if}
		<dt>{t('assets.type')}</dt>
		<dd>
			{asset.mime}
			{#if asset.edits && transparentEdges(asset.edits) && asset.mime !== 'image/png'}
				<span class="note">
					{t('assets.deliveredAsPng')}
				</span>
			{/if}
		</dd>
		<dt>{t('assets.uploaded')}</dt>
		<dd>
			{time(asset.createdAt)}
			{#if asset.uploadedBy}
				<span class="note">
					{t('common.byEmail', {
						email: asset.uploadedBy,
					})}
				</span>
			{/if}
		</dd>
	</dl>
	<div class="actions">
		<button type="button" onclick={() => downloadOriginal(asset)}>
			<Icon name="download" size={14} />
			{t('common.download')}
		</button>
		<button type="button" onclick={copy}>
			<Icon name="copy-url" size={14} />
			{copied ? t('common.copied') : t('assets.copyUrl')}
		</button>
		{#if canEdit}
			<UploadButton onfiles={f => onreplace(f[0])} icon="replace">
				{t('assets.replaceFile')}
			</UploadButton>
		{/if}
	</div>
</div>

<style lang="scss">
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
</style>
