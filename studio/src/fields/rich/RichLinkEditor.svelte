<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import { untrack } from 'svelte';
	import type { Link, Schema } from '../../core/schema';
	import type { Refs } from '../../entries/api';
	import LinkEditor from '../link/LinkEditor.svelte';
	import { BUTTON_LINKS, opensInNewTab } from '../link/link';
	import { setLink } from './editor';

	/**
	 * The settings of the selection's link, for the toolbar and the bubble:
	 * it edits a copy and sets it on the text when done or removed.
	 */
	let {
		editor,
		schema,
		refs,
		link,
		onclose,
	}: {
		editor: Editor;
		schema: Schema;
		refs: Refs;
		/** the link when it opens */
		link: Link;
		onclose: () => void;
	} = $props();

	let draft = $state<Link>(untrack(() => $state.snapshot(link) as Link));

	function apply(to: Link) {
		onclose();
		setLink(editor, to);
	}
</script>

<LinkEditor
	{schema}
	{refs}
	link={draft}
	to={BUTTON_LINKS}
	newTab={opensInNewTab(schema, draft, true)}
	onchange={l => (draft = l)}
	ondone={() => apply($state.snapshot(draft) as Link)}
	onremove={() => apply({ type: 'url', value: null })}
/>
