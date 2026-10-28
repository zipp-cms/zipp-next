<script lang="ts">
	import type {
		Component,
		Entry,
		Revision,
		Schema,
		Section,
	} from '../../api';
	import { setSite, site } from '../../route.svelte';
	import RevisionList from './RevisionList.svelte';

	/**
	 * Next to the content like Craft's sidebar: the status in each site,
	 * slug, type and parent, copying from another site and the revisions.
	 * A global only has the last two, it has no page and is always live.
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
		oncopy: (site: string) => void;
		onview: (rev: Revision) => void;
		onrestore: (rev: Revision) => void;
	} = $props();

	const single = $derived(secDef.kind === 'single');
	const multiSite = $derived(schema.sites.length > 1);
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
					Published in {schema.sites.find(s => s.handle === site.current)
						?.name}
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
						{#if changed.has('slug')}<span class="dot"></span>{/if}Slug
					</span>
					<input placeholder="from the title" bind:value={slug} />
					{#if uri !== null}<span class="muted">/{uri}</span>{/if}
					{#if errors.slug}<span class="error">{errors.slug}</span>{/if}
				</label>
			{/if}

			<label>
				<span>Type</span>
				{#if isNew && secDef.types.length > 1}
					<select bind:value={ty}>
						{#each secDef.types as t (t)}
							<option value={t}>
								{schema.components.find(c => c.handle === t)?.name}
							</option>
						{/each}
					</select>
				{:else}
					<span class="value">{comp.name}</span>
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
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	fieldset {
		margin: 0;
		border: none;
		min-width: 0;
	}

	.card {
		display: flex;
		flex-direction: column;
		gap: 18px;
		font-size: 14px;
	}

	label {
		display: flex;
		flex-direction: column;
		gap: 8px;

		> span:first-child {
			color: var(--muted);
		}
	}

	.switch {
		flex-direction: row;
		align-items: center;
		gap: 10px;

		> span:first-child {
			color: var(--text);
		}
	}

	.sites {
		display: flex;
		gap: 4px;
		margin-top: -8px;

		button {
			border: none;
			cursor: pointer;

			&.current {
				outline: 2px solid var(--muted);
				outline-offset: 1px;
			}
		}
	}

	.value {
		color: var(--text);
	}

	.muted {
		margin-top: -2px;
		color: var(--muted);
		font-size: 13px;
		word-break: break-all;
	}

	h3 {
		margin: 0 0 -8px;
		font-size: 16px;
		font-weight: 500;
	}
</style>
