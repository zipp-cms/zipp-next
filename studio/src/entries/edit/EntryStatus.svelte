<script lang="ts">
	import DateTime from './DateTime.svelte';
	import Switch from 'zipp-ui/Switch.svelte';
	import { dateState } from '../api';
	import { shortTime } from '../../core/format';
	import { canSetup } from '../../core/access';
	import { setupPath } from '../../core/paths';
	import SiteBadges from '../SiteBadges.svelte';
	import { componentName, siteName } from '../../core/schema';
	import type { Schema } from '../../core/schema';
	import { setSite } from '../../core/router.svelte';
	import type { EntryEditor } from './entryEditor.svelte';
	import { t } from '../../i18n/index.svelte';

	/**
	 * The status of the entry in each site and the dates that keep it off
	 * the website first, then what it is: type, parent and slug. In the
	 * sidebar, or `compact` on top of a narrow editor: closed under a line
	 * with the status and the date, open the switch in a row and the
	 * inputs next to each other as far as they fit.
	 */
	let {
		schema,
		ed,
		compact = false,
	}: {
		schema: Schema;
		/** with its section and type, see Edit */
		ed: EntryEditor;
		compact?: boolean;
	} = $props();

	const secDef = $derived(ed.secDef!);
	const comp = $derived(ed.comp!);
	const may = $derived(ed.may);
	const isNew = $derived(ed.id === null);
	const multiSite = $derived(schema.sites.length > 1);

	/** a published entry its dates keep off the website, and why */
	const waiting = $derived(dateState(ed.dates));

	// compact: open by a click, or when one of its fields has an error
	let open = $state(false);
	$effect(() => {
		if (ed.errors.expiryDate || ed.errors.slug) open = true;
	});
	const changed = $derived(
		['published', 'postDate', 'expiryDate', 'type', 'slug'].some(k =>
			ed.changed.has(k),
		),
	);
</script>

{#if compact}
	<details class="card status compact" bind:open>
		<summary>
			{#if changed}<span class="changed-dot"></span>{/if}
			<span class="published" class:waiting={ed.published && waiting}>
				{ed.published
					? t('entries.publishedIn', {
							site: siteName(schema, ed.site),
						})
					: t('entries.notPublishedIn', {
							site: siteName(schema, ed.site),
						})}
			</span>
			{#if ed.dates.postDate !== null}
				<span class="muted">
					· {t('entries.postDateAt', {
						time: shortTime(ed.dates.postDate),
					})}
				</span>
			{/if}
		</summary>
		<div class="body">{@render parts()}</div>
	</details>
{:else}
	<div class="card status">{@render parts()}</div>
{/if}

<!-- read only for a role that may only look, and while publishing,
discarding or restoring: what is typed then would be lost -->
{#snippet parts()}
	<div class="part">
		<fieldset class="inline" disabled={!may.edit || ed.busy}>
			<Switch bind:checked={ed.published} disabled={!may.publish}>
				<span class="published">
					{#if ed.changed.has('published')}<span
							class="changed-dot"
						></span>{/if}
					{t('entries.publishedIn', {
						site: siteName(schema, ed.site),
					})}
				</span>
			</Switch>
		</fieldset>
		<!-- the sites to switch to, also for who may only look -->
		{#if multiSite && !isNew}
			<SiteBadges {schema} published={ed.perSite} onselect={setSite} />
		{/if}
		{#if ed.published && waiting}
			<span class="note waiting">
				{waiting === 'pending'
					? t('entries.notLiveYet', {
							time: shortTime(ed.dates.postDate!),
						})
					: t('entries.notLiveAnymore', {
							time: shortTime(ed.dates.expiryDate!),
						})}
			</span>
		{/if}
	</div>

	<!-- in every site, like Craft's -->
	{#if !ed.single}
		<fieldset class="part" disabled={!may.edit || ed.busy}>
			<label class="date">
				<span>
					{#if ed.changed.has('postDate')}<span
							class="changed-dot"
						></span>{/if}{t('entries.postDate')}
				</span>
				<DateTime
					label={t('entries.postDate')}
					placeholder={t('entries.whenPublished')}
					value={ed.dates.postDate}
					onchange={v => (ed.dates.postDate = v)}
				/>
			</label>
			<label class="date">
				<span>
					{#if ed.changed.has('expiryDate')}<span
							class="changed-dot"
						></span>{/if}{t('entries.expiryDate')}
				</span>
				<DateTime
					label={t('entries.expiryDate')}
					placeholder={t('common.never')}
					value={ed.dates.expiryDate}
					onchange={v => (ed.dates.expiryDate = v)}
				/>
				{#if ed.errors.expiryDate}<span class="error">
						{ed.errors.expiryDate}
					</span>{/if}
			</label>
		</fieldset>
	{/if}

	<fieldset class="part" disabled={!may.edit || ed.busy}>
		<label>
			<span>
				{#if ed.changed.has('type')}<span
						class="changed-dot"
					></span>{/if}{t('entries.type')}
			</span>
			{#if secDef.types.length > 1}
				<select
					value={ed.ty}
					onchange={e => {
						// asked first when content would go
						if (!ed.changeType(e.currentTarget.value))
							e.currentTarget.value = ed.ty;
					}}
				>
					{#each secDef.types as ty (ty)}
						<option value={ty}>
							{componentName(schema, ty)}
						</option>
					{/each}
				</select>
			{:else if canSetup(schema)}
				<a
					class="value"
					href={setupPath('types', comp.handle)}
					title={t('entries.fieldsInSetup')}
				>
					{comp.name}
				</a>
			{:else}
				<span class="value">{comp.name}</span>
			{/if}
		</label>

		{#if isNew && ed.parents.length}
			<label>
				<span>{t('entries.parent')}</span>
				<select bind:value={ed.parentId}>
					<option value={null}>{t('entries.noParent')}</option>
					{#each ed.parents as p (p.id)}
						<option value={p.id}>{p.title}</option>
					{/each}
				</select>
			</label>
		{/if}
		<!-- the url is under View and in the live preview -->
		{#if !ed.single}
			<label>
				<span>
					{#if ed.changed.has('slug')}<span
							class="changed-dot"
						></span>{/if}{t('entries.slug')}
				</span>
				<input
					placeholder={t('entries.fromTitle')}
					bind:value={ed.slug}
				/>
				{#if ed.errors.slug}<span class="error">
						{ed.errors.slug}
					</span>{/if}
			</label>
		{/if}
	</fieldset>
{/snippet}

<style lang="scss">
	// in parts with a line between them
	.status {
		display: flex;
		flex-direction: column;
		padding: 0;
	}

	.part {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		padding: 0.75rem;

		& + .part {
			border-top: 1px solid var(--line);
		}
	}

	// a line with the status and the post date, open under it the switch
	// on its own row, the inputs next to each other as far as they fit; a
	// date has a day and a time
	.compact {
		display: block;
	}

	summary {
		padding: 0.625rem 0.75rem;
		font-size: var(--fs-sm);
		cursor: pointer;

		.published {
			font-size: var(--fs-sm);
			font-weight: 600;
		}
	}

	.compact .body {
		display: flex;
		flex-flow: row wrap;
		gap: 0.625rem 0.75rem;
		padding: 0.625rem 0.75rem;
		border-top: 1px solid var(--line);

		.part {
			display: contents;
		}

		.part:first-child {
			display: flex;
			flex-flow: row wrap;
			align-items: center;
			gap: 0.375rem 0.75rem;
			width: 100%;
			padding: 0;
		}

		label {
			flex: 1 1 10rem;
		}

		.date {
			flex-basis: 18rem;
		}
	}

	label {
		display: flex;
		flex-direction: column;
		gap: 0.3125rem;
		min-width: 0;

		> span:first-child {
			font-size: var(--fs-sm);
			font-weight: 600;
		}
	}

	.published {
		font-size: var(--fs-md);
		font-weight: 500;
	}

	.waiting {
		color: var(--warn);
	}

	// only disables, the card lays out what is in it
	.inline {
		display: contents;
	}

	.value {
		color: var(--text);
	}
</style>
