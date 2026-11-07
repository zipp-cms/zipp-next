<script lang="ts">
	import { untrack } from 'svelte';
	import type { Component, Schema } from '../../core/schema';
	import type { Refs } from '../api';
	import type { Revision, SiteDoc } from '../drafts';
	import Fields from '../../fields/Fields.svelte';
	import TabBar from '../../fields/TabBar.svelte';
	import { Tabs } from '../../fields/tabs.svelte';
	import { time } from '../../core/format';
	import { changeText } from './revisions';
	import EntryHead from './EntryHead.svelte';
	import { t } from '../../i18n/index.svelte';

	/** An old revision, read only, with the dots of what it changed. */
	let {
		schema,
		comp,
		refs,
		single,
		rev,
		doc,
		changed,
		tab = '',
		onrestore,
		onback,
	}: {
		schema: Schema;
		comp: Component;
		refs: Refs;
		single: boolean;
		rev: Revision;
		doc: SiteDoc | null;
		changed: Set<string>;
		/** the tab open in the editor, the view opens on it */
		tab?: string;
		/** none for a role that may not edit */
		onrestore: (() => void) | null;
		onback: () => void;
	} = $props();

	// the tabs like the editor's, with dots on those it changed
	const tabs = new Tabs(
		() => comp,
		() => doc?.fields ?? {},
		untrack(() => tab),
	);
</script>

<div class="viewing">
	<span>
		{rev.user
			? t('entries.revisionOfBy', {
					time: time(rev.createdAt),
					user: rev.user,
				})
			: t('entries.revisionOf', { time: time(rev.createdAt) })}
		<br />
		<small>
			{t('entries.changedList', {
				changes: changeText(rev, comp, schema.sites.length > 1),
			})}
		</small>
	</span>
	{#if onrestore}
		<button class="primary" onclick={onrestore}>
			{t('common.restore')}
		</button>
	{/if}
	<button class="light" onclick={onback}>{t('entries.backToEditing')}</button>
</div>
{#if doc}
	<!-- a disabled fieldset disables every input in it -->
	<fieldset disabled>
		<EntryHead title={doc.title} {changed} />
		{#if !single}
			<p class="slug">
				{#if changed.has('slug')}<span
						class="changed-dot"
					></span>{/if}{t('entries.slug')}:
				<code>{doc.slug}</code>
			</p>
		{/if}
	</fieldset>
	<!-- outside the fieldset, its buttons stay clickable -->
	{#if tabs.shown.length}
		<div class="tabs-row">
			<TabBar tabs={tabs.shown} bind:current={tabs.current} {changed} />
		</div>
	{/if}
	<fieldset disabled>
		<Fields
			{schema}
			component={comp}
			values={doc.fields}
			errors={{}}
			{refs}
			{changed}
			only={tabs.fields}
			card={!!(comp.card || tabs.card)}
		/>
	</fieldset>
{:else}
	<p class="muted">{t('entries.noContentHere')}</p>
{/if}

<style lang="scss">
	.viewing {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 1.5rem;
		padding: 0.75rem 1rem;
		background: var(--input);

		span {
			margin-right: auto;
		}
	}

	.tabs-row {
		margin-bottom: 1.125rem;
	}

	.slug {
		margin: -0.875rem 0 1.875rem;
		color: var(--muted);
		font-size: var(--fs-base);
	}
</style>
