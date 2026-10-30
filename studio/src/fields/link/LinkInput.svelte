<script lang="ts">
	import type { Link, LinkKind, Schema } from '../../core/schema';
	import type { Refs } from '../../entries/api';
	import Icon from 'zipp-ui/Icon.svelte';
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

	function add(type: LinkKind) {
		const { label } = link;
		onchange({ type, value: null, ...(label ? { label } : {}) });
		editing = true;
	}
</script>

{#snippet removeButton()}
	{#if onremove}
		<button
			type="button"
			class="icon remove"
			title={removeTitle}
			onclick={onremove}
		>
			<Icon name="delete" size={15} />
		</button>
	{/if}
{/snippet}

{#if !filled && !editing}
	<div class="link-input empty" class:required>
		<span>
			{required ? 'Required – no link yet.' : 'No link yet.'}
		</span>
		<span class="end">
			<LinkAdd {to} onadd={add} />
			{@render removeButton()}
		</span>
	</div>
{:else}
	<div class="link-input" class:editing>
		{#if filled}
			<div class="row">
				<LinkRow {link} {summary} {newTab}>
					{#snippet actions()}
						<button
							type="button"
							class="icon"
							class:on={editing}
							title="Edit the link"
							onclick={() => (editing = !editing)}
						>
							<Icon name="edit" size={15} />
						</button>
						<button
							type="button"
							class="icon remove"
							title="Remove the link"
							onclick={() => {
								editing = false;
								onchange({ type: link.type, value: null });
							}}
						>
							<Icon name="unlink" size={15} />
						</button>
						{@render removeButton()}
					{/snippet}
				</LinkRow>
			</div>
		{/if}
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

		.row + & {
			border-top: 1px solid var(--line);
		}
	}

	button.on {
		background: var(--accent-soft);
		color: var(--accent);
	}

	.remove {
		color: var(--danger);
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
		justify-content: space-between;
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
