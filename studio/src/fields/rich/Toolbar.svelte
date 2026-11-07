<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import type { Component, Link, Schema } from '../../core/schema';
	import type { Refs } from '../../entries/api';
	import Icon from 'zipp-ui/Icon.svelte';
	import Popover from 'zipp-ui/Popover.svelte';
	import SchemaIcon from '../../core/SchemaIcon.svelte';
	import BlockTypeList from '../BlockTypeList.svelte';
	import RichLinkEditor from './RichLinkEditor.svelte';
	import {
		hasStyle,
		insertBlock,
		insertButton,
		setFormat,
		toggleStyle,
		type RichOptions,
	} from './editor';
	import { toolsOf } from './toolbar';
	import { t } from '../../i18n/index.svelte';

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

	// the marks and lists to switch on and off, by their node or mark name
	const toggles: Record<
		string,
		{ icon: string; title: string; run: () => void }
	> = $derived({
		bold: {
			icon: 'bold',
			title: t('fields.bold'),
			run: () => chain().toggleBold().run(),
		},
		italic: {
			icon: 'italic',
			title: t('fields.italic'),
			run: () => chain().toggleItalic().run(),
		},
		bulletList: {
			icon: 'channel',
			title: t('fields.list'),
			run: () => chain().toggleBulletList().run(),
		},
		orderedList: {
			icon: 'numbered',
			title: t('fields.numberedList'),
			run: () => chain().toggleOrderedList().run(),
		},
	});

	// what the field allows, in the order of its toolbar
	const tools = $derived(toolsOf(opts, blockTypes));

	// the link of the selection, without a label
	let linking = $state(false);
	let link = $state<Link>({ type: 'url', value: '' });

	function openLink(toggle: () => void) {
		const current = editor.getAttributes('link').link;
		link = current ?? { type: 'url', value: '' };
		toggle();
	}

	// the styles of the selection; the button shows the first
	const styles = $derived(opts.styles ?? []);
	const styled = $derived.by(() => {
		void tick;
		return styles.filter(st => hasStyle(editor, st));
	});
	let styling = $state(false);

	let inserting = $state(false);
	function insert(f: () => void) {
		inserting = false;
		f();
	}
</script>

<div class="toolbar">
	{#each tools as tool, i (i)}
		{#if tool.tool === '|'}
			<span class="toolbar-sep"></span>
		{:else if tool.tool === 'format'}
			<select
				class="format"
				title={t('fields.format')}
				value={opts.headings.find(level =>
					active('heading', { level }),
				) ?? 0}
				onchange={e => setFormat(editor, Number(e.currentTarget.value))}
			>
				<option value={0}>{t('fields.paragraph')}</option>
				{#each opts.headings as level (level)}
					<option value={level}>
						{t('fields.heading', { level })}
					</option>
				{/each}
			</select>
		{:else if tool.tool === 'styles'}
			<Popover
				bind:open={styling}
				width="14rem"
				label={t('fields.styles')}
			>
				{#snippet trigger({ toggle })}
					<button
						type="button"
						class="styles"
						class:on={styled.length}
						title={t('fields.styles')}
						onclick={toggle}
					>
						<span class="truncate">
							{styled[0]?.label ?? t('fields.styles')}
						</span>
						<Icon name="chevron-down" size={14} />
					</button>
				{/snippet}
				{#each styles as st (st.class)}
					<button
						type="button"
						class="popup-item"
						onclick={() => {
							styling = false;
							toggleStyle(editor, st);
						}}
					>
						<span class="mark">
							{#if styled.includes(st)}<Icon
									name="check"
									size={14}
								/>{/if}
						</span>
						{st.label}
						<span class="muted">
							{st.inline
								? t('fields.text')
								: t('fields.paragraph')}
						</span>
					</button>
				{/each}
			</Popover>
		{:else if tool.tool === 'link'}
			<Popover bind:open={linking} width="34rem" label={t('fields.link')}>
				{#snippet trigger({ toggle })}
					<button
						type="button"
						class:on={active('link')}
						title={t('fields.link')}
						aria-label={t('fields.link')}
						onclick={() => openLink(toggle)}
					>
						<Icon name="link" />
					</button>
				{/snippet}
				<div class="link">
					<RichLinkEditor
						{editor}
						{schema}
						{refs}
						{link}
						sections={opts.sections}
						onclose={() => (linking = false)}
					/>
				</div>
			</Popover>
		{:else if tool.tool === 'button'}
			<button
				type="button"
				title={t('fields.insertButton')}
				aria-label={t('fields.insertButton')}
				onclick={() => insertButton(editor)}
			>
				<Icon name="button" />
			</button>
		{:else if tool.tool === 'block'}
			{@const type = tool.type}
			<button
				type="button"
				title={t('fields.insertBlock', { name: type.name })}
				aria-label={t('fields.insertBlock', { name: type.name })}
				onclick={() => insertBlock(editor, schema, type.handle)}
			>
				<SchemaIcon icon={type.icon} fallback="block" />
			</button>
		{:else if tool.tool === 'insert'}
			{@const { button, types } = tool}
			<Popover
				bind:open={inserting}
				width="12.25rem"
				label={t('fields.insert')}
			>
				{#snippet trigger({ toggle })}
					<button type="button" class="insert" onclick={toggle}>
						<Icon name="add" size={14} />
						{t('fields.insert')}
						<Icon name="chevron-down" size={14} />
					</button>
				{/snippet}
				{#if button}
					<button
						type="button"
						class="popup-item"
						onclick={() => insert(() => insertButton(editor))}
					>
						{t('fields.button')}
					</button>
				{/if}
				<BlockTypeList
					{types}
					onpick={h => insert(() => insertBlock(editor, schema, h))}
				/>
			</Popover>
		{:else}
			{@const b = toggles[tool.tool]}
			<button
				type="button"
				class:on={active(tool.tool)}
				aria-pressed={active(tool.tool)}
				title={b.title}
				aria-label={b.title}
				onclick={b.run}
			>
				<Icon name={b.icon} />
			</button>
		{/if}
	{/each}
</div>

<style lang="scss">
	// a light bar over the text
	.toolbar {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 2px;
		padding: 0.25rem;
		background: var(--surface-2);
		border-bottom: 1px solid var(--line);
		border-radius: var(--radius) var(--radius) 0 0;

		> button,
		> :global(.popover > button) {
			// as big as the tabs of the editor
			min-width: 2rem;
			justify-content: center;
			padding: 0.3125rem 0.5rem;
			border: none;
			background: none;
			color: var(--text-2);
			font-size: var(--fs-md);

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

	// narrow: one line that scrolls sideways (the tools' popups are
	// fixed, nothing cuts them); a thin scrollbar for a mouse, none on
	// touch
	@include container-max(36rem) {
		.toolbar {
			flex-wrap: nowrap;
			overflow-x: auto;
			scrollbar-width: none;

			@include hover {
				scrollbar-width: thin;
			}

			> * {
				flex-shrink: 0;
			}
		}
	}

	.format {
		width: auto;
		height: 2rem;
		padding: 0 0.5rem;
		font-size: var(--fs-md);
		border-color: var(--border);
		border-radius: var(--radius-sm);
	}

	.styles {
		max-width: 10rem;
	}

	// the check of a style the selection has, room for it on the others
	.mark {
		display: flex;
		width: 0.875rem;
		color: var(--accent);
	}

	.popup-item .muted {
		margin-left: auto;
		font-size: var(--fs-sm);
		font-weight: normal;
	}

	.insert {
		color: var(--accent-hover) !important;
		font-weight: 500;
	}

	.link {
		padding: 0.75rem;
	}
</style>
