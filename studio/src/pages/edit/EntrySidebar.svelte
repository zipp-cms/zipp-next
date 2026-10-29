<script lang="ts">
	import type {
		Component,
		Entry,
		Revision,
		Schema,
		Section,
	} from '../../api';
	import type { Block, Values } from '../../api';
	import { blockSummary, showBlock } from '../../lib/blocks.svelte';
	import { setSite, site } from '../../route.svelte';
	import RevisionList from './RevisionList.svelte';

	/**
	 * Next to the content like Craft's sidebar: the status in each site,
	 * slug, type and parent, the outline of the blocks, copying from
	 * another site and the revisions. A global has no page and is always
	 * live, it has no status, slug or type.
	 */
	let {
		schema,
		secDef,
		comp,
		global,
		isNew,
		canEdit,
		canPublish,
		published = $bindable(),
		slug = $bindable(),
		ty = $bindable(),
		parentId = $bindable(),
		uri,
		perSite,
		changed,
		errors,
		parents,
		revisions,
		values,
		shownFields,
		oncopy,
		onview,
		onrestore,
	}: {
		schema: Schema;
		secDef: Section;
		comp: Component;
		global: boolean;
		isNew: boolean;
		canEdit: boolean;
		canPublish: boolean;
		published: boolean;
		slug: string;
		ty: string;
		parentId: number | null;
		uri: string | null;
		/** whether the entry is live in each site, as saved */
		perSite: Record<string, boolean>;
		changed: Set<string>;
		errors: Record<string, string>;
		parents: Entry[];
		revisions: Revision[];
		/** the fields of the entry, for the outline */
		values: Values;
		/** the fields of the open tab, all without tabs */
		shownFields: string[] | null;
		oncopy: (site: string) => void;
		onview: (rev: Revision) => void;
		onrestore: (rev: Revision) => void;
	} = $props();

	const single = $derived(secDef.kind === 'single');
	const multiSite = $derived(schema.sites.length > 1);

	// the blocks fields on screen, each with its blocks
	const outlines = $derived(
		comp.fields
			.filter(f => f.kind === 'blocks')
			.filter(f => !shownFields || shownFields.includes(f.handle))
			.map(f => ({
				field: f,
				blocks: (values[f.handle] ?? []) as Block[],
			}))
			.filter(o => o.blocks.length),
	);
	const nameOf = (type: string) =>
		schema.components.find(c => c.handle === type)?.name ?? type;
</script>

<aside>
	{#if !global}
		<!-- read only for a role that may only look -->
		<fieldset class="card" disabled={!canEdit}>
			<label class="switch">
				<input
					type="checkbox"
					bind:checked={published}
					disabled={!canPublish}
				/>
				<span>
					{#if changed.has('published')}<span class="dot"></span>{/if}
					Published in {schema.sites.find(
						s => s.handle === site.current,
					)?.name}
				</span>
			</label>
			{#if multiSite && !isNew}
				<div class="sites">
					{#each schema.sites as s (s.handle)}
						<button
							type="button"
							class="badge"
							class:draft={!perSite[s.handle]}
							class:current={s.handle === site.current}
							title="{s.name}: {perSite[s.handle]
								? 'published'
								: 'not published'}, open"
							onclick={() => setSite(s.handle)}
						>
							{s.handle.toUpperCase()}
						</button>
					{/each}
				</div>
			{/if}

			{#if !single}
				<label>
					<span>
						{#if changed.has('slug')}<span
								class="dot"
							></span>{/if}Slug
					</span>
					<input placeholder="from the title" bind:value={slug} />
					{#if uri !== null}<span class="muted">/{uri}</span>{/if}
					{#if errors.slug}<span class="error">
							{errors.slug}
						</span>{/if}
				</label>
			{/if}

			<label>
				<span>Type</span>
				{#if isNew && secDef.types.length > 1}
					<select bind:value={ty}>
						{#each secDef.types as t (t)}
							<option value={t}>
								{schema.components.find(c => c.handle === t)
									?.name}
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

			{#if isNew && parents.length}
				<label>
					<span>Parent</span>
					<select bind:value={parentId}>
						<option value={null}>None, on the top</option>
						{#each parents as p (p.id)}
							<option value={p.id}>{p.title}</option>
						{/each}
					</select>
				</label>
			{/if}
		</fieldset>
	{/if}

	{#each outlines as o (o.field.handle)}
		<nav class="card outline" aria-label="Outline of {o.field.label}">
			<h3>Outline · {o.field.label}</h3>
			<ol>
				{#each o.blocks as b, i (i)}
					<li>
						<button
							type="button"
							onclick={() => showBlock(o.field.handle, i)}
						>
							<span class="n">
								{String(i + 1).padStart(2, '0')}
							</span>
							<span class="name">{nameOf(b.type)}</span>
							<span class="sum">
								{blockSummary(
									schema,
									schema.components.find(
										c => c.handle === b.type,
									),
									b.fields,
								)}
							</span>
						</button>
					</li>
				{/each}
			</ol>
		</nav>
	{/each}

	{#if !isNew && multiSite && canEdit}
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
					{#each schema.sites.filter(s => s.handle !== site.current) as s (s.handle)}
						<option value={s.handle}>{s.name}</option>
					{/each}
				</select>
			</label>
		</div>
	{/if}

	{#if revisions.length}
		<div class="card">
			<h3>Revisions</h3>
			<RevisionList
				{revisions}
				{comp}
				{multiSite}
				{onview}
				onrestore={canEdit ? onrestore : null}
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

	fieldset {
		margin: 0;
		border: none;
		min-width: 0;
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
			font-size: 12px;
			font-weight: 600;
		}
	}

	.switch {
		flex-direction: row;
		align-items: center;
		gap: 8px;

		> span:first-child {
			font-size: 13px;
			font-weight: 500;
		}
	}

	.sites {
		display: flex;
		gap: 4px;
		margin-top: -6px;

		button {
			border: none;
			cursor: pointer;

			&.current {
				outline: 2px solid var(--accent);
				outline-offset: 1px;
			}
		}
	}

	.value {
		color: var(--text);
	}

	.muted {
		color: var(--subtle);
		font-size: 12px;
		word-break: break-all;
	}

	h3 {
		margin: 0 0 -6px;
		font-size: 12px;
		font-weight: 600;
	}

	.outline {
		gap: 8px;

		ol {
			margin: 0;
			padding: 0;
			list-style: none;
		}

		button {
			display: grid;
			grid-template-columns: 22px auto minmax(0, 1fr);
			align-items: baseline;
			gap: 6px;
			width: 100%;
			padding: 5px 4px;
			border: none;
			background: none;
			text-align: left;
			font-weight: 400;
		}

		.n {
			color: var(--subtle);
			font-family: var(--mono);
			font-size: 10px;
		}

		.name {
			font-size: 13px;
			font-weight: 500;
		}

		.sum {
			overflow: hidden;
			color: var(--subtle);
			font-size: 11px;
			text-align: right;
			text-overflow: ellipsis;
			white-space: nowrap;
		}
	}
</style>
