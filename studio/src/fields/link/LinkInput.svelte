<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { Link, LinkKind, Schema } from '../../core/schema';
	import type { Refs } from '../../entries/api';
	import Menu from 'zipp-ui/Menu.svelte';
	import { chosenAssets } from '../../assets/chosen.svelte';
	import LinkAdd from './LinkAdd.svelte';
	import LinkEditor from './LinkEditor.svelte';
	import LinkRow from './LinkRow.svelte';
	import { hasTarget, linkSummary, opensInNewTab } from './link';

	/**
	 * A link to an entry, a url, an email, a phone or a file: a row with
	 * where it goes, its settings under it while editing; without a target
	 * the kinds to add one. The link field and the buttons of a rich text.
	 */
	let {
		schema,
		refs,
		link,
		to,
		labels = false,
		newTab: externalInNewTab = true,
		required = false,
		onchange,
		onremove,
		removeTitle = 'Remove',
		handle,
	}: {
		schema: Schema;
		refs: Refs;
		link: Link;
		/** the kinds it may be */
		to: LinkKind[];
		/** with a label */
		labels?: boolean;
		/** external links open in a new tab, unless it's set */
		newTab?: boolean;
		required?: boolean;
		onchange: (link: Link) => void;
		/** removes what holds the link, like a button of a rich text */
		onremove?: () => void;
		removeTitle?: string;
		/** a DragHandle on the left, like a button of a rich text */
		handle?: Snippet;
	} = $props();

	let editing = $state(false);
	const filled = $derived(hasTarget(link));
	const newTab = $derived(opensInNewTab(schema, link, externalInNewTab));
	const asset = chosenAssets(() =>
		link.type === 'asset' && typeof link.value === 'number'
			? [link.value]
			: [],
	);
	const summary = $derived(
		linkSummary(
			schema,
			link,
			refs,
			typeof link.value === 'number'
				? asset.get(link.value)?.filename
				: undefined,
		),
	);

	// what holds the link goes too, like a button of a rich text
	const removeItem = $derived(
		onremove
			? [
					{
						label: removeTitle,
						icon: 'delete',
						onclick: onremove,
						danger: true,
					},
				]
			: [],
	);

	function add(type: LinkKind) {
		const { label } = link;
		onchange({ type, value: null, ...(label ? { label } : {}) });
		editing = true;
	}
</script>

{#if !filled && !editing}
	<div class="link-input empty" class:required data-menu-area>
		{@render handle?.()}
		<span class="message">
			{required ? 'Required – no link yet.' : 'No link yet.'}
		</span>
		<span class="end">
			<LinkAdd {to} onadd={add} />
			<Menu items={removeItem} label="Actions" />
		</span>
	</div>
{:else}
	<!-- the row stays while editing, also without a target (another kind) -->
	<div class="link-input" class:editing>
		<div class="row">
			<LinkRow
				{link}
				{summary}
				{newTab}
				{handle}
				{editing}
				onedit={() => (editing = !editing)}
				menu={[
					{
						label: 'Edit',
						icon: 'edit',
						onclick: () => (editing = true),
					},
					{
						label: 'Remove link',
						icon: 'unlink',
						onclick: () => {
							editing = false;
							onchange({ type: link.type, value: null });
						},
					},
					...removeItem,
				]}
			/>
		</div>
		{#if editing}
			<div class="panel">
				<LinkEditor
					{schema}
					{refs}
					{link}
					{to}
					{labels}
					{newTab}
					{onchange}
					ondone={() => (editing = false)}
				/>
			</div>
		{/if}
	</div>
{/if}

<style lang="scss">
	.link-input {
		border: 1px solid var(--border-strong);
		border-radius: var(--radius-lg);
		background: var(--surface);

		&.editing {
			border-color: var(--accent);
		}
	}

	.row {
		padding: 0.5rem 0.5rem 0.5rem 0.625rem;
	}

	// the settings under the row, set off by their background
	.panel {
		padding: 1rem;
		border-radius: 0 0 var(--radius-lg) var(--radius-lg);
		background: var(--surface-2);
		border-top: 1px solid var(--line);
	}

	.message {
		flex: 1;
	}

	.end {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	// no target yet: what to add
	.empty {
		display: flex;
		align-items: center;
		gap: 1rem;
		flex-wrap: wrap;
		padding: 0.625rem 0.625rem 0.625rem 1rem;
		border-style: dashed;
		color: var(--muted);

		&.required {
			border-color: var(--danger);
			background: var(--danger-soft);
			color: var(--danger);
		}
	}
</style>
