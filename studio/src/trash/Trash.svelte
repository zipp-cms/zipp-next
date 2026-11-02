<script module lang="ts">
	import { getTrash } from './api';

	export async function loadProps() {
		return { trash: await getTrash() };
	}
</script>

<script lang="ts">
	import Links from '../setup/Links.svelte';
	import { untrack } from 'svelte';
	import PageTop from 'zipp-ui/PageTop.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import Table from 'zipp-ui/Table.svelte';
	import { sectionName, type Schema } from '../core/schema';
	import { Busy } from '../core/busy.svelte';
	import { entryPath } from '../core/paths';
	import { fileSize, time } from '../core/format';
	import { t } from '../i18n/index.svelte';
	import {
		emptyTrash,
		purge,
		restore,
		type Trash,
		type TrashedAsset,
		type TrashedEntry,
	} from './api';

	/**
	 * The trash, like Craft's: what was deleted in the studio, entries
	 * with the children that went with them and assets. Restore brings
	 * one back, or it is deleted for good, by hand or after `keepDays`.
	 * A role sees what it may delete.
	 */
	let { schema, trash: loaded }: { schema: Schema; trash: Trash } = $props();

	// the page is new for every visit, then it reloads itself
	let trash = $state(untrack(() => loaded));
	const action = new Busy();
	// restored ones link to where they are back
	let restored = $state<TrashedEntry[]>([]);
	const empty = $derived(!trash.entries.length && !trash.assets.length);

	function run(fn: () => Promise<unknown>) {
		return action.run(async () => {
			await fn();
			trash = await getTrash();
		});
	}

	function restoreEntry(e: TrashedEntry) {
		run(async () => {
			await restore('entries', e.id);
			restored = [e, ...restored];
		});
	}

	function purgeEntry(e: TrashedEntry) {
		const question = e.children
			? t('trash.confirmEntryChildren', { name: e.title, n: e.children })
			: t('trash.confirmEntry', { name: e.title });
		if (!confirm(question)) return;
		run(() => purge('entries', e.id));
	}

	function purgeAsset(a: TrashedAsset) {
		if (!confirm(t('trash.confirmAsset', { name: a.filename }))) return;
		run(() => purge('assets', a.id));
	}

	function emptyAll() {
		const n = trash.entries.length + trash.assets.length;
		if (!confirm(t('trash.confirmAll', { n }))) return;
		run(emptyTrash);
	}
</script>

<PageTop
	title={t('trash.title')}
	note={t('trash.note', { days: trash.keepDays })}
>
	{#snippet actions()}
		{#if !empty}
			<button class="danger" disabled={action.busy} onclick={emptyAll}>
				<Icon name="delete" />
				{t('trash.empty')}
			</button>
		{/if}
	{/snippet}
</PageTop>

{#if action.error}<p class="error">{action.error}</p>{/if}

{#if restored.length}
	<p class="note restored">
		{t('trash.restored')}
		<Links
			links={restored.map(e => ({
				text: e.title,
				href: entryPath(e.section, e.id),
			}))}
		/>
	</p>
{/if}

{#snippet deleted(item: { deletedAt: number; deletedBy: string | null })}
	<td class="muted">
		{time(item.deletedAt)}
		{#if item.deletedBy}<span class="by">
				{t('trash.by', { email: item.deletedBy })}
			</span>{/if}
	</td>
{/snippet}

<!-- restore, delete for good -->
{#snippet actions(name: string, onrestore: () => void, onpurge: () => void)}
	<td class="actions">
		<button class="light" disabled={action.busy} onclick={onrestore}>
			<Icon name="reset" />
			{t('common.restore')}
		</button>
		<button
			class="icon danger"
			aria-label={t('trash.deleteNameForGood', { name })}
			title={t('trash.deleteForGood')}
			disabled={action.busy}
			onclick={onpurge}
		>
			<Icon name="delete" />
		</button>
	</td>
{/snippet}

{#if empty}
	<p class="card empty">{t('trash.isEmpty')}</p>
{/if}

{#if trash.entries.length}
	<div class="card">
		<h2>{t('trash.entries')}</h2>
		<Table
			rows={trash.entries}
			key={e => e.id}
			columns={[
				t('trash.entry'),
				t('trash.section'),
				t('trash.deleted'),
				'',
			]}
			widths={['auto', '12rem', '16rem', '15rem']}
		>
			{#snippet row(e)}
				<td>
					<span class="title">{e.title}</span>
					{#if e.children}
						<span class="note">
							{t('trash.below', { n: e.children })}
						</span>
					{/if}
				</td>
				<td>{sectionName(schema, e.section)}</td>
				{@render deleted(e)}
				{@render actions(
					e.title,
					() => restoreEntry(e),
					() => purgeEntry(e),
				)}
			{/snippet}
		</Table>
	</div>
{/if}

{#if trash.assets.length}
	<div class="card">
		<h2>{t('trash.assets')}</h2>
		<Table
			rows={trash.assets}
			key={a => a.id}
			columns={[
				t('trash.asset'),
				t('trash.size'),
				t('trash.deleted'),
				'',
			]}
			widths={['auto', '12rem', '16rem', '15rem']}
		>
			{#snippet row(a)}
				<td>
					<Icon name={a.kind === 'image' ? 'dashboard' : 'file'} />
					<span class="title">{a.filename}</span>
				</td>
				<td class="muted">{fileSize(a.size)}</td>
				{@render deleted(a)}
				{@render actions(
					a.filename,
					() => run(() => restore('assets', a.id)),
					() => purgeAsset(a),
				)}
			{/snippet}
		</Table>
	</div>
{/if}

<style lang="scss">
	.card + .card {
		margin-top: 1rem;
	}

	h2 {
		margin: 0 0 0.75rem;
		font-size: var(--fs-md);
	}

	.title {
		font-weight: 500;
	}

	td :global(.icon) {
		vertical-align: -3px;
		color: var(--muted);
	}

	.actions {
		text-align: right;
		white-space: nowrap;

		button + button {
			margin-left: 0.375rem;
		}
	}

	.by {
		display: block;
		font-size: var(--fs-sm);
	}

	.restored {
		margin-bottom: 1rem;
	}
</style>
