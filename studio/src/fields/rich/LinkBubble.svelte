<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import { untrack } from 'svelte';
	import type { Link, Schema } from '../../core/schema';
	import type { Refs } from '../../entries/api';
	import Icon from 'zipp-ui/Icon.svelte';
	import { openAsset, openEntry } from '../../core/flyout.svelte';
	import LinkEditor from '../link/LinkEditor.svelte';
	import LinkRow from '../link/LinkRow.svelte';
	import { BUTTON_LINKS, linkSummary, opensInNewTab } from '../link/link';
	import { setLink } from './editor';

	/**
	 * The link the cursor is on, under it: where it goes, and edit, remove
	 * and open it; editing shows its settings right there.
	 */
	let {
		editor,
		tick,
		schema,
		refs,
	}: {
		editor: Editor;
		/** changes with the selection */
		tick: number;
		schema: Schema;
		refs: Refs;
	} = $props();

	let editing = $state(false);
	let draft = $state<Link>({ type: 'url', value: '' });
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
	const shown = $derived(editing ? draft : focused ? link : null);

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
		draft = $state.snapshot(link) as Link;
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
		if (l.type === 'entry' && typeof l.value === 'number')
			return openEntry(l.value);
		if (l.type === 'asset' && typeof l.value === 'number')
			return openAsset(l.value);
		const v = String(l.value ?? '');
		const href =
			l.type === 'email'
				? `mailto:${v}`
				: l.type === 'phone'
					? `tel:${v}`
					: v + (l.anchor ? `#${l.anchor}` : '');
		window.open(href, '_blank', 'noopener');
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
		class="link-bubble"
		class:editing
		style:top="{place?.top ?? 0}px"
		style:left="{place?.left ?? 0}px"
		style:width="{place?.width ?? 0}px"
		role="dialog"
		aria-label="Link"
		tabindex="-1"
		bind:this={bubble}
		{onkeydown}
		{onfocusout}
	>
		{#if editing}
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
		{:else}
			<LinkRow
				link={shown}
				summary={linkSummary(schema, shown, refs)}
				newTab={opensInNewTab(schema, shown, true)}
			>
				{#snippet actions()}
					<button
						type="button"
						class="icon"
						title="Open the target"
						onclick={() => open(shown)}
					>
						<Icon name="external" size={15} />
					</button>
					<button
						type="button"
						class="icon"
						title="Edit the link"
						onclick={edit}
					>
						<Icon name="edit" size={15} />
					</button>
					<button
						type="button"
						class="icon remove"
						title="Remove the link"
						onclick={() => apply({ type: 'url', value: null })}
					>
						<Icon name="unlink" size={15} />
					</button>
				{/snippet}
			</LinkRow>
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
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
		background: var(--surface);
		box-shadow: var(--shadow-pop);

		&.editing {
			padding: 1rem;
		}
	}

	.remove {
		color: var(--danger);
	}
</style>
