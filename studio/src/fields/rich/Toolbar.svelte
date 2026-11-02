<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import type { Component, Link, Schema } from '../../core/schema';
	import type { Refs } from '../../entries/api';
	import Icon from 'zipp-ui/Icon.svelte';
	import Popover from 'zipp-ui/Popover.svelte';
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
	const toggles = $derived([
		{
			name: 'bold',
			icon: 'bold',
			title: t('fields.bold'),
			run: () => chain().toggleBold().run(),
		},
		{
			name: 'italic',
			icon: 'italic',
			title: t('fields.italic'),
			run: () => chain().toggleItalic().run(),
		},
		...(opts.lists
			? [
					{
						name: 'bulletList',
						icon: 'channel',
						title: t('fields.list'),
						run: () => chain().toggleBulletList().run(),
					},
					{
						name: 'orderedList',
						icon: 'numbered',
						title: t('fields.numberedList'),
						run: () => chain().toggleOrderedList().run(),
					},
				]
			: []),
	]);

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
	{#if opts.headings.length}
		<select
			class="format"
			title={t('fields.format')}
			value={opts.headings.find(level => active('heading', { level })) ??
				0}
			onchange={e => setFormat(editor, Number(e.currentTarget.value))}
		>
			<option value={0}>{t('fields.paragraph')}</option>
			{#each opts.headings as level (level)}
				<option value={level}>{t('fields.heading', { level })}</option>
			{/each}
		</select>
		<span class="toolbar-sep"></span>
	{/if}
	{#if styles.length}
		<Popover bind:open={styling} width="14rem" label={t('fields.styles')}>
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
						{st.inline ? t('fields.text') : t('fields.paragraph')}
					</span>
				</button>
			{/each}
		</Popover>
		<span class="toolbar-sep"></span>
	{/if}
	{#each toggles as b (b.name)}
		<button
			type="button"
			class:on={active(b.name)}
			aria-pressed={active(b.name)}
			title={b.title}
			onclick={b.run}
		>
			<Icon name={b.icon} size={16} />
		</button>
	{/each}
	{#if opts.links}
		<Popover bind:open={linking} width="34rem" label={t('fields.link')}>
			{#snippet trigger({ toggle })}
				<button
					type="button"
					class:on={active('link')}
					title={t('fields.link')}
					onclick={() => openLink(toggle)}
				>
					<Icon name="link" size={16} />
				</button>
			{/snippet}
			<div class="link">
				<RichLinkEditor
					{editor}
					{schema}
					{refs}
					{link}
					onclose={() => (linking = false)}
				/>
			</div>
		</Popover>
	{/if}
	{#if opts.buttons || blockTypes.length}
		<span class="toolbar-sep"></span>
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
			{#if opts.buttons}
				<button
					type="button"
					class="popup-item"
					onclick={() => insert(() => insertButton(editor))}
				>
					{t('fields.button')}
				</button>
			{/if}
			<BlockTypeList
				types={blockTypes}
				onpick={h => insert(() => insertBlock(editor, schema, h))}
			/>
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
