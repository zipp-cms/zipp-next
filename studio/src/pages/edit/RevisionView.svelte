<script lang="ts">
	import type { Component, Refs, Revision, Schema, SiteDoc } from '../../api';
	import Fields from '../../fields/Fields.svelte';
	import { changeText, time } from '../../lib/format';
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
		<button class="dark" onclick={onrestore}>Restore</button>
	{/if}
	<button class="light" onclick={onback}>Back to editing</button>
</div>
{#if doc}
	<!-- a disabled fieldset disables every input in it -->
	<fieldset disabled>
		<EntryHead title={doc.title} slug={doc.slug} {single} {changed} />
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

	fieldset {
		margin: 0;
		padding: 0;
		border: none;
	}

	.muted {
		color: var(--muted);
	}
</style>
