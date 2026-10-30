<script lang="ts">
	import type { Asset } from '../assets/api';
	import type { Field, Link, LinkKind, Schema, Values } from '../core/schema';
	import type { EntryRef, Refs } from '../entries/api';
	import AddButton from '../ui/AddButton.svelte';
	import AssetChip from '../assets/AssetChip.svelte';
	import AssetPicker from '../assets/AssetPicker.svelte';
	import EntryChip from '../entries/EntryChip.svelte';
	import EntrySearch from '../entries/EntrySearch.svelte';
	import ItemActions from '../ui/ItemActions.svelte';
	import { chosenAssets } from '../assets/chosen.svelte';
	import { LINK_NAMES, isExternal, linkInput, linkableTypes } from './link';

	/**
	 * A link to an entry, a url, an email, a phone or a file, with an
	 * optional label and whether it opens in a new tab.
	 */
	let {
		schema,
		field,
		values,
		refs,
	}: { schema: Schema; field: Field; values: Values; refs: Refs } = $props();

	const to = $derived(field.kind === 'link' ? field.to : []);
	const link = $derived<Link>(
		values[field.handle] ?? { type: to[0], value: null },
	);
	const newTab = $derived(
		link.newTab ??
			(field.kind === 'link' && field.newTab && isExternal(schema, link)),
	);
	const input = $derived(linkInput(link.type));

	const asset = chosenAssets(() =>
		link.type === 'asset' && typeof link.value === 'number'
			? [link.value]
			: [],
	);
	let picking = $state(false);

	function set(change: Partial<Link>) {
		values[field.handle] = { ...link, ...change };
	}

	/** Another kind of link starts empty, its label stays. */
	function setType(type: LinkKind) {
		const { label } = link;
		values[field.handle] = {
			type,
			value: null,
			...(label ? { label } : {}),
		};
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

{#snippet clear()}
	<ItemActions
		index={0}
		count={1}
		onremove={() => set({ value: null })}
		removeTitle="Remove the target"
	/>
{/snippet}

<div class="link">
	<div class="row">
		{#if to.length > 1}
			<select
				value={link.type}
				onchange={e => setType(e.currentTarget.value as LinkKind)}
			>
				{#each to as t (t)}
					<option value={t}>{LINK_NAMES[t]}</option>
				{/each}
			</select>
		{/if}

		<div class="value">
			{#if link.type === 'entry'}
				{#if typeof link.value === 'number'}
					<EntryChip
						{schema}
						{refs}
						id={link.value}
						actions={clear}
					/>
				{:else}
					<EntrySearch
						{schema}
						types={linkableTypes(schema)}
						onpick={pickEntry}
					/>
				{/if}
			{:else if link.type === 'asset'}
				{@const chosen =
					typeof link.value === 'number'
						? asset.get(link.value)
						: null}
				{#if chosen}
					<AssetChip asset={chosen} actions={clear} />
				{:else}
					<AddButton onclick={() => (picking = true)}>
						Choose file
					</AddButton>
				{/if}
			{:else}
				<input
					type={input.type}
					placeholder={input.placeholder}
					value={link.value ?? ''}
					oninput={e => set({ value: e.currentTarget.value })}
				/>
			{/if}
		</div>
	</div>

	{#if field.kind === 'link' && field.labels}
		<input
			placeholder="Label (optional)"
			value={link.label ?? ''}
			oninput={e => set({ label: e.currentTarget.value || undefined })}
		/>
	{/if}

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
	.link {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.row {
		display: flex;
		gap: 8px;

		select {
			width: 8rem;
			flex-shrink: 0;
		}
	}

	.value {
		flex: 1;
		min-width: 0;
	}

	.check {
		display: flex;
		align-items: center;
		gap: 8px;
		color: var(--muted);
	}
</style>
