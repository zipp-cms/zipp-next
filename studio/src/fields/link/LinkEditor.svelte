<script lang="ts">
	import type { Asset } from '../../assets/api';
	import type { Link, LinkKind, Schema } from '../../core/schema';
	import type { EntryRef, Refs } from '../../entries/api';
	import AddButton from 'zipp-ui/AddButton.svelte';
	import AssetChip from '../../assets/AssetChip.svelte';
	import AssetPicker from '../../assets/AssetPicker.svelte';
	import EntrySearch from '../../entries/EntrySearch.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import Segmented from 'zipp-ui/Segmented.svelte';
	import { chosenAssets } from '../../assets/chosen.svelte';
	import { sectionOf } from '../../core/schema';
	import {
		LINK_ICONS,
		LINK_NAMES,
		hasAnchor,
		linkInput,
		linkableTypes,
	} from './link';

	/**
	 * The settings of a link, a row each: the kind of target, the target,
	 * the label, an anchor on the page, a new tab. `ondone` closes it.
	 */
	let {
		schema,
		refs,
		link,
		to,
		labels = false,
		newTab,
		onchange,
		ondone,
		onremove,
	}: {
		schema: Schema;
		refs: Refs;
		link: Link;
		to: LinkKind[];
		labels?: boolean;
		/** whether it opens in a new tab now */
		newTab: boolean;
		onchange: (link: Link) => void;
		ondone?: () => void;
		/** a way to remove it, like a link of a rich text */
		onremove?: () => void;
	} = $props();

	const input = $derived(linkInput(link.type));
	const entry = $derived(
		link.type === 'entry' && typeof link.value === 'number'
			? refs[link.value]
			: null,
	);
	const asset = chosenAssets(() =>
		link.type === 'asset' && typeof link.value === 'number'
			? [link.value]
			: [],
	);
	let picking = $state(false);

	function set(change: Partial<Link>) {
		const next = { ...link, ...change };
		// empty texts aren't kept
		if (!next.label) delete next.label;
		if (!next.anchor) delete next.anchor;
		onchange(next);
	}

	// the target of each kind while it is open: trying another kind and
	// coming back doesn't lose the one chosen
	const byKind: Partial<Record<LinkKind, Link>> = {};

	/** Another kind of link: its target from before, else empty; the label stays. */
	function setType(type: LinkKind) {
		const { label } = link;
		byKind[link.type] = $state.snapshot(link) as Link;
		const back = byKind[type];
		const { label: _, ...target } = back ?? { type, value: null };
		onchange({ ...target, ...(label ? { label } : {}) });
	}

	function pickEntry(e: EntryRef) {
		refs[e.id] = e;
		set({ value: e.id });
	}

	function pickAsset(a: Asset) {
		picking = false;
		set({ value: a.id });
	}
</script>

<div class="link-editor">
	{#if to.length > 1}
		<span class="label">Target</span>
		<Segmented
			options={to.map(k => ({ value: k, label: LINK_NAMES[k] }))}
			value={link.type}
			onchange={setType}
			label="Target"
		>
			{#snippet item({ value, label })}
				<Icon name={LINK_ICONS[value]} size={14} />
				{label}
			{/snippet}
		</Segmented>
	{/if}

	<span class="label">{LINK_NAMES[link.type]}</span>
	<div>
		{#if link.type === 'entry'}
			{#if entry}
				<span class="entry">
					<span
						class="dot"
						class:live={entry.published}
						title={entry.published ? 'Published' : 'Not published'}
					></span>
					<span class="truncate">{entry.title}</span>
					<span class="note">
						{sectionOf(schema, entry.section)?.name ?? ''}
					</span>
					<button
						type="button"
						class="icon"
						title="Another entry"
						onclick={() => set({ value: null })}
					>
						<Icon name="close" size={14} />
					</button>
				</span>
			{:else}
				<EntrySearch
					{schema}
					types={linkableTypes(schema)}
					onpick={pickEntry}
				/>
			{/if}
		{:else if link.type === 'asset'}
			{@const chosen =
				typeof link.value === 'number' ? asset.get(link.value) : null}
			{#if chosen}
				<AssetChip asset={chosen}>
					{#snippet actions()}
						<button
							type="button"
							class="icon"
							title="Another file"
							onclick={() => set({ value: null })}
						>
							<Icon name="close" size={14} />
						</button>
					{/snippet}
				</AssetChip>
			{:else}
				<AddButton onclick={() => (picking = true)}>
					Choose file
				</AddButton>
			{/if}
		{:else}
			<input
				type={input.type}
				class:mono={link.type === 'url'}
				placeholder={input.placeholder}
				value={link.value ?? ''}
				oninput={e => set({ value: e.currentTarget.value })}
			/>
		{/if}
	</div>

	{#if labels}
		<span class="label">Label</span>
		<input
			placeholder="From the target"
			value={link.label ?? ''}
			oninput={e => set({ label: e.currentTarget.value })}
		/>
	{/if}

	{#if hasAnchor(link.type)}
		<span class="label">Anchor</span>
		<div class="anchor">
			<span class="hash">
				<span>#</span>
				<input
					class="mono"
					placeholder="optional, e.g. contact"
					value={link.anchor ?? ''}
					oninput={e =>
						set({
							anchor: e.currentTarget.value.replace(/^#/, ''),
						})}
				/>
			</span>
			<span class="note">Jumps to a section of the target page.</span>
		</div>
	{/if}

	<span></span>
	<div class="end">
		<label class="check">
			<input
				type="checkbox"
				checked={newTab}
				onchange={e => set({ newTab: e.currentTarget.checked })}
			/>
			Open in a new tab
		</label>
		{#if link.type === 'asset'}
			<label class="check">
				<input
					type="checkbox"
					checked={link.download ?? false}
					onchange={e =>
						set({ download: e.currentTarget.checked || undefined })}
				/>
				Download instead of opening
			</label>
		{/if}
		{#if onremove}
			<button type="button" class="push" onclick={onremove}>
				Remove link
			</button>
		{/if}
		{#if ondone}
			<button
				type="button"
				class="primary"
				class:push={!onremove}
				onclick={ondone}
			>
				Done
			</button>
		{/if}
	</div>
</div>

{#if picking}
	<AssetPicker
		images={false}
		multiple={false}
		selected={typeof link.value === 'number' ? [link.value] : []}
		onpick={pickAsset}
		onclose={() => (picking = false)}
	/>
{/if}

<style lang="scss">
	// a label column and the controls, like the fields of a block
	.link-editor {
		display: grid;
		grid-template-columns: 5.5rem minmax(0, 1fr);
		align-items: center;
		gap: 0.75rem 1rem;
	}

	// as wide as its kinds, not the column
	.link-editor > :global(.segmented) {
		justify-self: start;
	}

	.label {
		font-size: var(--fs-sm);
		font-weight: 600;
	}

	.mono {
		font-family: var(--mono);
		font-size: var(--fs-md);
	}

	// the chosen entry, like a filled input
	.entry {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		min-height: 2.25rem;
		padding: 0.125rem 0.25rem 0.125rem 0.75rem;
		border: 1px solid var(--border-strong);
		border-radius: var(--radius);
		background: var(--input);

		.truncate {
			flex: 1;
			min-width: 0;
		}
	}

	.dot {
		width: 0.5rem;
		height: 0.5rem;
		flex-shrink: 0;
		border-radius: 50%;
		background: var(--subtle);

		&.live {
			background: var(--ok);
		}
	}

	.anchor {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	// the # in front of the input
	.hash {
		display: flex;
		width: min(13rem, 100%);

		> span {
			display: grid;
			place-items: center;
			padding: 0 0.625rem;
			border: 1px solid var(--border-strong);
			border-right: none;
			border-radius: var(--radius) 0 0 var(--radius);
			background: var(--surface-2);
			color: var(--muted);
		}

		input {
			border-radius: 0 var(--radius) var(--radius) 0;
		}
	}

	.end {
		display: flex;
		align-items: center;
		gap: 1.25rem;
		flex-wrap: wrap;
	}

	// the buttons at the end of the row
	.push {
		margin-left: auto;
	}
</style>
