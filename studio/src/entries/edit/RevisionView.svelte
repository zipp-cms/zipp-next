<script lang="ts">
	import type { Component, Schema } from '../../core/schema';
	import type { Refs } from '../api';
	import type { Revision, SiteDoc } from '../drafts';
	import Fields from '../../fields/Fields.svelte';
	import { time } from '../../core/format';
	import { changeText } from './revisions';
	import EntryHead from './EntryHead.svelte';

	/** An old revision, read only, with the dots of what it changed. */
	let {
		schema,
		comp,
		refs,
		single,
		rev,
		doc,
		changed,
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
		/** none for a role that may not edit */
		onrestore: (() => void) | null;
		onback: () => void;
	} = $props();
</script>

<div class="viewing">
	<span>
		Revision of {time(rev.createdAt)}{rev.user ? ` by ${rev.user}` : ''},
		read only
		<br />
		<small>
			Changed: {changeText(rev, comp, schema.sites.length > 1)}
		</small>
	</span>
	{#if onrestore}
		<button class="primary" onclick={onrestore}>Restore</button>
	{/if}
	<button class="light" onclick={onback}>Back to editing</button>
</div>
{#if doc}
	<!-- a disabled fieldset disables every input in it -->
	<fieldset disabled>
		<EntryHead title={doc.title} {changed} />
		{#if !single}
			<p class="slug">
				{#if changed.has('slug')}<span class="dot"></span>{/if}Slug:
				<code>{doc.slug}</code>
			</p>
		{/if}
		<Fields
			{schema}
			component={comp}
			values={doc.fields}
			errors={{}}
			{refs}
			{changed}
		/>
	</fieldset>
{:else}
	<p class="muted">This revision has no content for this site.</p>
{/if}

<style lang="scss">
	.viewing {
		display: flex;
		align-items: center;
		gap: 12px;
		margin-bottom: 24px;
		padding: 12px 16px;
		background: var(--input);

		span {
			margin-right: auto;
		}
	}

	.slug {
		margin: -14px 0 30px;
		color: var(--muted);
		font-size: var(--fs-base);
	}
</style>
