<script lang="ts">
	import BlockOutline from './BlockOutline.svelte';
	import EntryStatus from './EntryStatus.svelte';
	import type { Block, Schema } from '../../core/schema';
	import type { Revision } from '../drafts';
	import RevisionList from './RevisionList.svelte';
	import type { EntryEditor } from './entryEditor.svelte';
	import { t } from '../../i18n/index.svelte';

	/**
	 * Next to the content like Craft's sidebar: the status in each site,
	 * type, parent and slug, the outline of the blocks, copying from
	 * another site and the revisions. A global has no page and is always
	 * live, it has no status, slug or type. A `narrow` editor has the
	 * status on top (`EntryStatus`, collapsed), copying in the bar's ⋯
	 * menu, the rest goes under the content with the revisions closed.
	 */
	let {
		schema,
		ed,
		shownFields,
		narrow,
		oncopy,
		onview,
	}: {
		schema: Schema;
		/** with its section and type, see Edit */
		ed: EntryEditor;
		/** the fields of the open tab, all without tabs */
		shownFields: string[] | null;
		/** the editor is too narrow for a column on the side */
		narrow: boolean;
		oncopy: (site: string) => void;
		onview: (rev: Revision) => void;
	} = $props();

	const comp = $derived(ed.comp!);
	const isNew = $derived(ed.id === null);
	const multiSite = $derived(schema.sites.length > 1);

	// the blocks fields on screen, each with its blocks
	const outlines = $derived(
		comp.fields
			.filter(f => f.kind === 'blocks')
			.filter(f => !shownFields || shownFields.includes(f.handle))
			.map(f => ({
				field: f,
				blocks: (ed.fields[f.handle] ?? []) as Block[],
			}))
			.filter(o => o.blocks.length),
	);
</script>

<aside>
	{#if !ed.global && !narrow}
		<EntryStatus {schema} {ed} />
	{/if}

	{#each outlines as o (o.field.handle)}
		<BlockOutline
			{schema}
			field={o.field}
			blocks={o.blocks}
			refs={ed.refs}
		/>
	{/each}

	<!-- narrow: in the ⋯ menu of the bar instead -->
	{#if !isNew && multiSite && ed.may.edit && !narrow}
		<div class="card">
			<label>
				<span>{t('entries.copyContent')}</span>
				<select
					value=""
					onchange={e => {
						const other = e.currentTarget.value;
						e.currentTarget.value = '';
						oncopy(other);
					}}
				>
					<option value="" disabled>
						{t('entries.fromOtherSite')}
					</option>
					{#each schema.sites.filter(s => s.handle !== ed.site) as s (s.handle)}
						<option value={s.handle}>{s.name}</option>
					{/each}
				</select>
			</label>
		</div>
	{/if}

	{#snippet revisions()}
		<RevisionList
			revisions={ed.revisions}
			{comp}
			{multiSite}
			{onview}
			onrestore={ed.may.edit ? rev => ed.restore(rev) : null}
		/>
	{/snippet}
	{#if ed.revisions.length && narrow}
		<!-- at the end of a long page, closed -->
		<details class="card">
			<summary>
				{t('entries.revisionsCount', { n: ed.revisions.length })}
			</summary>
			{@render revisions()}
		</details>
	{:else if ed.revisions.length}
		<div class="card">
			<h3>{t('entries.revisions')}</h3>
			{@render revisions()}
		</div>
	{/if}
</aside>

<style lang="scss">
	aside {
		position: sticky;
		top: 4rem;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		max-height: calc(100vh - 5rem);
		overflow-y: auto;
	}

	// under the content
	@include editor-narrow {
		aside {
			position: static;
			max-height: none;
			overflow: visible;
		}
	}

	.card {
		display: flex;
		flex-direction: column;
		gap: 0.875rem;
		padding: 0.75rem;
	}

	label {
		display: flex;
		flex-direction: column;
		gap: 0.3125rem;

		> span:first-child {
			font-size: var(--fs-sm);
			font-weight: 600;
		}
	}

	h3,
	summary {
		margin: 0 0 -0.375rem;
	}

	summary {
		font-size: var(--fs-sm);
		font-weight: 600;
		cursor: pointer;
	}

	details:not([open]) {
		gap: 0;

		summary {
			margin: 0;
		}
	}
</style>
