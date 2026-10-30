<script module lang="ts">
	import { getTrash } from './api';

	export async function loadProps() {
		return { trash: await getTrash() };
	}
</script>

<script lang="ts">
	import { untrack } from 'svelte';
	import PageTop from 'zipp-ui/PageTop.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import Table from 'zipp-ui/Table.svelte';
	import { sectionName, type Schema } from '../core/schema';
	import { Busy } from '../core/busy.svelte';
	import { entryPath } from '../core/router.svelte';
	import { fileSize, plural, time } from '../core/format';
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
		const also = e.children
			? ` And ${plural(e.children, 'entry', 'entries')} below it.`
			: '';
		if (
			!confirm(`Delete ${e.title} for good?${also} It can't be restored.`)
		)
			return;
		run(() => purge('entries', e.id));
	}

	function purgeAsset(a: TrashedAsset) {
		if (!confirm(`Delete ${a.filename} for good? It can't be restored.`))
			return;
		run(() => purge('assets', a.id));
	}

	function emptyAll() {
		const n = trash.entries.length + trash.assets.length;
		const what = plural(n, 'item', 'items');
		if (!confirm(`Delete ${what} for good? They can't be restored.`))
			return;
		run(emptyTrash);
	}
</script>

<PageTop
	title="Trash"
	note="Deleted entries and assets, for {trash.keepDays} days"
>
	{#snippet actions()}
		{#if !empty}
			<button class="danger" disabled={action.busy} onclick={emptyAll}>
				<Icon name="delete" />
				Empty trash
			</button>
		{/if}
	{/snippet}
</PageTop>

{#if action.error}<p class="error">{action.error}</p>{/if}

{#if restored.length}
	<p class="note restored">
		Restored:
		{#each restored as e, i (e.id)}{i ? ', ' : ''}
			<a href={entryPath(e.section, e.id)}>{e.title}</a>{/each}
	</p>
{/if}

{#if empty}
	<p class="card empty">The trash is empty.</p>
{/if}

{#if trash.entries.length}
	<div class="card">
		<h2>Entries</h2>
		<Table
			rows={trash.entries}
			key={e => e.id}
			columns={['Entry', 'Section', 'Deleted', '']}
			widths={['auto', '12rem', '16rem', '15rem']}
		>
			{#snippet row(e)}
				<td>
					<span class="title">{e.title}</span>
					{#if e.children}
						<span class="note">
							and {plural(e.children, 'entry', 'entries')} below
						</span>
					{/if}
				</td>
				<td>{sectionName(schema, e.section)}</td>
				<td class="muted">
					{time(e.deletedAt)}
					{#if e.deletedBy}<span class="by">
							by {e.deletedBy}
						</span>{/if}
				</td>
				<td class="actions">
					<button
						class="light"
						disabled={action.busy}
						onclick={() => restoreEntry(e)}
					>
						<Icon name="reset" />
						Restore
					</button>
					<button
						class="icon danger"
						aria-label="Delete {e.title} for good"
						title="Delete for good"
						disabled={action.busy}
						onclick={() => purgeEntry(e)}
					>
						<Icon name="delete" />
					</button>
				</td>
			{/snippet}
		</Table>
	</div>
{/if}

{#if trash.assets.length}
	<div class="card">
		<h2>Assets</h2>
		<Table
			rows={trash.assets}
			key={a => a.id}
			columns={['Asset', 'Size', 'Deleted', '']}
			widths={['auto', '12rem', '16rem', '15rem']}
		>
			{#snippet row(a)}
				<td>
					<Icon name={a.kind === 'image' ? 'dashboard' : 'file'} />
					<span class="title">{a.filename}</span>
				</td>
				<td class="muted">{fileSize(a.size)}</td>
				<td class="muted">
					{time(a.deletedAt)}
					{#if a.deletedBy}<span class="by">
							by {a.deletedBy}
						</span>{/if}
				</td>
				<td class="actions">
					<button
						class="light"
						disabled={action.busy}
						onclick={() => run(() => restore('assets', a.id))}
					>
						<Icon name="reset" />
						Restore
					</button>
					<button
						class="icon danger"
						aria-label="Delete {a.filename} for good"
						title="Delete for good"
						disabled={action.busy}
						onclick={() => purgeAsset(a)}
					>
						<Icon name="delete" />
					</button>
				</td>
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
