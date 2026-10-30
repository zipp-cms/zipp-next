<script lang="ts">
	import SiteBadges from '../SiteBadges.svelte';
	import BlockOutline from './BlockOutline.svelte';
	import { componentName } from '../../core/schema';
	import type { Block, Component, Schema, Section } from '../../core/schema';
	import type { Revision } from '../drafts';
	import { setSite } from '../../core/router.svelte';
	import RevisionList from './RevisionList.svelte';
	import type { EntryEditor, May } from './entryEditor.svelte';

	/**
	 * Next to the content like Craft's sidebar: the status in each site,
	 * slug, type and parent, the outline of the blocks, copying from
	 * another site and the revisions. A global has no page and is always
	 * live, it has no status, slug or type.
	 */
	let {
		schema,
		ed,
		secDef,
		comp,
		global,
		may,
		shownFields,
		oncopy,
		onview,
	}: {
		schema: Schema;
		ed: EntryEditor;
		secDef: Section;
		comp: Component;
		global: boolean;
		may: May;
		/** the fields of the open tab, all without tabs */
		shownFields: string[] | null;
		oncopy: (site: string) => void;
		onview: (rev: Revision) => void;
	} = $props();

	const isNew = $derived(ed.id === null);
	const single = $derived(secDef.kind === 'single');
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
	{#if !global}
		<!-- read only for a role that may only look -->
		<div class="card">
			<fieldset class="inline" disabled={!may.edit}>
				<label class="switch">
					<input
						type="checkbox"
						bind:checked={ed.published}
						disabled={!may.publish}
					/>
					<span>
						{#if ed.changed.has('published')}<span
								class="dot"
							></span>{/if}
						Published in {schema.sites.find(
							s => s.handle === ed.site,
						)?.name}
					</span>
				</label>
			</fieldset>
			<!-- the sites to switch to, also for who may only look -->
			{#if multiSite && !isNew}
				<div class="sites">
					<SiteBadges
						{schema}
						published={ed.perSite}
						onselect={setSite}
					/>
				</div>
			{/if}

			<fieldset class="inline" disabled={!may.edit}>
				{#if !single}
					<label>
						<span>
							{#if ed.changed.has('slug')}<span
									class="dot"
								></span>{/if}Slug
						</span>
						<input
							placeholder="from the title"
							bind:value={ed.slug}
						/>
						{#if ed.uri !== null}<span class="note uri">
								/{ed.uri}
							</span>{/if}
						{#if ed.errors.slug}<span class="error">
								{ed.errors.slug}
							</span>{/if}
					</label>
				{/if}

				<label>
					<span>Type</span>
					{#if isNew && secDef.types.length > 1}
						<select bind:value={ed.ty}>
							{#each secDef.types as t (t)}
								<option value={t}>
									{componentName(schema, t)}
								</option>
							{/each}
						</select>
					{:else}
						<a
							class="value"
							href="/admin/setup/types/{comp.handle}"
							title="Its fields in Setup"
						>
							{comp.name}
						</a>
					{/if}
				</label>

				{#if isNew && ed.parents.length}
					<label>
						<span>Parent</span>
						<select bind:value={ed.parentId}>
							<option value={null}>None, on the top</option>
							{#each ed.parents as p (p.id)}
								<option value={p.id}>{p.title}</option>
							{/each}
						</select>
					</label>
				{/if}
			</fieldset>
		</div>
	{/if}

	{#each outlines as o (o.field.handle)}
		<BlockOutline {schema} field={o.field} blocks={o.blocks} />
	{/each}

	{#if !isNew && multiSite && may.edit}
		<div class="card">
			<label>
				<span>Copy content</span>
				<select
					value=""
					onchange={e => {
						const other = e.currentTarget.value;
						e.currentTarget.value = '';
						oncopy(other);
					}}
				>
					<option value="" disabled>From another site…</option>
					{#each schema.sites.filter(s => s.handle !== ed.site) as s (s.handle)}
						<option value={s.handle}>{s.name}</option>
					{/each}
				</select>
			</label>
		</div>
	{/if}

	{#if ed.revisions.length}
		<div class="card">
			<h3>Revisions</h3>
			<RevisionList
				revisions={ed.revisions}
				{comp}
				{multiSite}
				{onview}
				onrestore={may.edit ? rev => ed.restore(rev) : null}
			/>
		</div>
	{/if}
</aside>

<style lang="scss">
	aside {
		position: sticky;
		top: 64px;
		display: flex;
		flex-direction: column;
		gap: 12px;
		max-height: calc(100vh - 80px);
		overflow-y: auto;
	}

	.card {
		display: flex;
		flex-direction: column;
		gap: 14px;
		padding: 12px;
	}

	label {
		display: flex;
		flex-direction: column;
		gap: 5px;

		> span:first-child {
			font-size: var(--fs-sm);
			font-weight: 600;
		}
	}

	.switch {
		flex-direction: row;
		align-items: center;
		gap: 8px;

		> span:first-child {
			font-size: var(--fs-md);
			font-weight: 500;
		}
	}

	// only disables, the card lays out what is in it
	.inline {
		display: contents;
	}

	.sites {
		margin-top: -6px;
	}

	.value {
		color: var(--text);
	}

	.uri {
		word-break: break-all;
	}

	h3 {
		margin: 0 0 -6px;
		font-size: var(--fs-sm);
		font-weight: 600;
	}
</style>
