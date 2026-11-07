<script lang="ts">
	import { safeUrl } from '../../core/markdown';
	import type { Editor } from '@tiptap/core';
	import { untrack } from 'svelte';
	import type { Link, Schema } from '../../core/schema';
	import type { Refs } from '../../entries/api';
	import { openAsset, openEntry } from '../../core/flyout.svelte';
	import LinkRow from '../link/LinkRow.svelte';
	import { linkHref, linkedId } from '../link/link';
	import RichLinkEditor from './RichLinkEditor.svelte';
	import { setLink } from './editor';
	import { t } from '../../i18n/index.svelte';

	/**
	 * The link the cursor is on, under it: where it goes, and edit, remove
	 * and open it; editing shows its settings right there.
	 */
	let {
		editor,
		tick,
		schema,
		refs,
		sections = [],
	}: {
		editor: Editor;
		/** changes with the selection */
		tick: number;
		schema: Schema;
		refs: Refs;
		/** links only to entries of these sections, none: all */
		sections?: string[];
	} = $props();

	let editing = $state(false);
	// the link when editing started, the editor changes a copy
	let start = $state<Link>({ type: 'url', value: '' });
	let bubble = $state<HTMLElement>();

	// only while the text or the bubble has the focus: a click into
	// another field leaves the cursor on the link
	let focused = $state(untrack(() => editor.isFocused));
	const inside = (el: EventTarget | null) =>
		el instanceof Node &&
		(!!bubble?.contains(el) || editor.view.dom.contains(el));
	$effect(() => {
		const onfocus = () => (focused = true);
		const onblur = ({ event }: { event: FocusEvent }) => {
			focused = inside(event.relatedTarget);
		};
		editor.on('focus', onfocus);
		editor.on('blur', onblur);
		return () => {
			editor.off('focus', onfocus);
			editor.off('blur', onblur);
		};
	});

	function onfocusout(e: FocusEvent) {
		if (inside(e.relatedTarget)) return;
		// a moment later: a button that goes away (Edit) takes the focus
		// with it, the editor's first input gets it then
		setTimeout(() => {
			if (inside(document.activeElement)) return;
			focused = false;
			editing = false;
		});
	}

	// the link of the cursor, none when it is outside of one
	const link = $derived.by((): Link | null => {
		void tick;
		if (!editor.isEditable || !editor.isActive('link')) return null;
		return editor.getAttributes('link').link ?? null;
	});
	const shown = $derived(editing ? start : focused ? link : null);

	// under the start of the selection, in the rich text's box
	const place = $derived.by(() => {
		void tick;
		const box = editor.view.dom.closest('.rich')?.getBoundingClientRect();
		if (!box || !shown) return null;
		const at = editor.view.coordsAtPos(editor.state.selection.from);
		const width = editing ? 544 : 400;
		return {
			top: at.bottom - box.top + 6,
			left: Math.max(0, Math.min(at.left - box.left, box.width - width)),
			width,
		};
	});

	function edit() {
		start = $state.snapshot(link) as Link;
		editing = true;
	}

	// editing starts in its first input
	$effect(() => {
		if (editing) bubble?.querySelector<HTMLElement>('input')?.focus();
	});

	function apply(to: Link) {
		editing = false;
		setLink(editor, to);
	}

	/** An entry or a file in the flyout, the rest in a new tab. */
	function open(l: Link) {
		const entry = linkedId(l, 'entry');
		if (entry !== null) return openEntry(entry);
		const asset = linkedId(l, 'asset');
		if (asset !== null) return openAsset(asset);
		const href = linkHref(l);
		// a draft may have any url, only the web and the site open
		if (safeUrl(href)) window.open(href, '_blank', 'noopener');
	}

	function onkeydown(e: KeyboardEvent) {
		if (e.key === 'Escape' && editing) {
			e.stopPropagation();
			editing = false;
			editor.commands.focus();
		}
	}
</script>

{#if shown && place}
	<!-- not part of the text: clicks here keep the cursor on the link -->
	<div
		class="popup link-bubble"
		class:editing
		style:top="{place?.top ?? 0}px"
		style:left="{place?.left ?? 0}px"
		style:width="{place?.width ?? 0}px"
		role="dialog"
		aria-label={t('fields.link')}
		tabindex="-1"
		bind:this={bubble}
		{onkeydown}
		{onfocusout}
	>
		{#if editing}
			<RichLinkEditor
				{editor}
				{schema}
				{refs}
				link={start}
				{sections}
				onclose={() => (editing = false)}
			/>
		{:else}
			<LinkRow
				{schema}
				{refs}
				link={shown}
				onedit={edit}
				menu={[
					{ label: t('common.edit'), icon: 'edit', onclick: edit },
					{
						label: t('fields.openTarget'),
						icon: 'external',
						onclick: () => open(shown),
					},
					{
						label: t('fields.removeLink'),
						icon: 'unlink',
						onclick: () => apply({ type: 'url', value: null }),
						danger: true,
					},
				]}
			/>
		{/if}
	</div>
{/if}

<style lang="scss">
	// floats under the link, like a popover
	.link-bubble {
		position: absolute;
		z-index: var(--z-popup);
		max-width: calc(100% - 0.5rem);
		padding: 0.375rem 0.375rem 0.375rem 0.5rem;

		&.editing {
			padding: 1rem;
		}
	}
</style>
