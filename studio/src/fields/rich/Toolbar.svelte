<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import type { Component, Link, Schema } from '../../core/schema';
	import type { Refs } from '../../entries/api';
	import Icon from 'zipp-ui/Icon.svelte';
	import Popover from 'zipp-ui/Popover.svelte';
	import LinkEditor from '../link/LinkEditor.svelte';
	import { BUTTON_LINKS, opensInNewTab } from '../link/link';
	import {
		insertBlock,
		insertButton,
		setFormat,
		setLink,
		type RichOptions,
	} from './editor';

	/** The bar over a rich text: only what its field allows. */
	let {
		editor,
		tick,
		opts,
		blockTypes,
		schema,
		refs,
	}: {
		editor: Editor;
		/** changes with the selection */
		tick: number;
		opts: RichOptions;
		blockTypes: Component[];
		schema: Schema;
		refs: Refs;
	} = $props();

	function active(name: string, attrs?: Record<string, unknown>) {
		void tick;
		return editor.isActive(name, attrs);
	}

	const chain = () => editor.chain().focus();

	// the link of the selection, without a label
	let linking = $state(false);
	let link = $state<Link>({ type: 'url', value: '' });

	function openLink(toggle: () => void) {
		const current = editor.getAttributes('link').link;
		link = current ?? { type: 'url', value: '' };
		toggle();
	}

	function applyLink(to: Link) {
		linking = false;
		setLink(editor, to);
	}

	let inserting = $state(false);
	function insert(f: () => void) {
		inserting = false;
		f();
	}
</script>

<div class="toolbar">
	{#if opts.headings.length}
		<select
			class="format"
			title="Format"
			value={opts.headings.find(level => active('heading', { level })) ??
				0}
			onchange={e => setFormat(editor, Number(e.currentTarget.value))}
		>
			<option value={0}>Paragraph</option>
			{#each opts.headings as level (level)}
				<option value={level}>Heading {level}</option>
			{/each}
		</select>
		<span class="sep"></span>
	{/if}
	<button
		type="button"
		class:on={active('bold')}
		title="Bold"
		onclick={() => chain().toggleBold().run()}
	>
		<b>B</b>
	</button>
	<button
		type="button"
		class:on={active('italic')}
		title="Italic"
		onclick={() => chain().toggleItalic().run()}
	>
		<i class="italic">I</i>
	</button>
	{#if opts.lists}
		<button
			type="button"
			class:on={active('bulletList')}
			title="List"
			onclick={() => chain().toggleBulletList().run()}
		>
			<Icon name="channel" size={15} />
		</button>
		<button
			type="button"
			class:on={active('orderedList')}
			title="Numbered list"
			onclick={() => chain().toggleOrderedList().run()}
		>
			<span class="num">1.</span>
		</button>
	{/if}
	{#if opts.links}
		<Popover bind:open={linking} width="34rem" label="Link">
			{#snippet trigger({ toggle })}
				<button
					type="button"
					class:on={active('link')}
					title="Link"
					onclick={() => openLink(toggle)}
				>
					<Icon name="link" size={14} />
				</button>
			{/snippet}
			<div class="link">
				<LinkEditor
					{schema}
					{refs}
					{link}
					to={BUTTON_LINKS}
					newTab={opensInNewTab(schema, link, true)}
					onchange={l => (link = l)}
					ondone={() => applyLink($state.snapshot(link))}
					onremove={() => applyLink({ type: 'url', value: null })}
				/>
			</div>
		</Popover>
	{/if}
	{#if opts.buttons || blockTypes.length}
		<span class="sep"></span>
		<Popover bind:open={inserting} width="12.25rem" label="Insert">
			{#snippet trigger({ toggle })}
				<button type="button" class="insert" onclick={toggle}>
					<Icon name="add" size={13} />
					Insert
					<Icon name="chevron-down" size={13} />
				</button>
			{/snippet}
			{#if opts.buttons}
				<button
					type="button"
					class="popup-item"
					onclick={() => insert(() => insertButton(editor))}
				>
					Button
				</button>
			{/if}
			{#each blockTypes as b (b.handle)}
				<button
					type="button"
					class="popup-item"
					onclick={() => insert(() => insertBlock(editor, b.handle))}
				>
					{b.name}
				</button>
			{/each}
		</Popover>
	{/if}
</div>

<style lang="scss">
	// a light bar over the text
	.toolbar {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 2px;
		padding: 3px 0.25rem;
		background: var(--surface-2);
		border-bottom: 1px solid var(--line);
		border-radius: var(--radius) var(--radius) 0 0;

		> button,
		> :global(.popover > button) {
			padding: 3px 0.4375rem;
			border: none;
			background: none;
			color: var(--text-2);
			font-size: var(--fs-sm);

			&.on {
				background: var(--accent-soft);
				color: var(--accent);
			}

			@include hover {
				&:hover:not(:disabled) {
					background: var(--hover);
				}
			}
		}
	}

	.format {
		width: auto;
		height: 1.75rem;
		padding: 0 0.375rem;
		font-size: var(--fs-sm);
		border-color: var(--border);
		border-radius: var(--radius-sm);
	}

	.italic {
		font-family: Georgia, serif;
	}

	.num {
		font-size: var(--fs-xs);
		font-weight: 600;
	}

	.insert {
		color: var(--accent-hover) !important;
		font-weight: 500;
	}

	.link {
		padding: 0.75rem;
	}
</style>
