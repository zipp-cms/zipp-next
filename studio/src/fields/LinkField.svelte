<script lang="ts">
	import {
		listAssets,
		type Asset,
		type EntryRef,
		type Field,
		type Link,
		type LinkKind,
		type Refs,
		type Schema,
		type Values,
	} from '../api';
	import AssetPicker from '../lib/AssetPicker.svelte';
	import EntrySearch from '../lib/EntrySearch.svelte';
	import Thumb from '../lib/Thumb.svelte';
	import Icon from '../lib/Icon.svelte';

	let {
		schema,
		field,
		values,
		refs,
	}: { schema: Schema; field: Field; values: Values; refs: Refs } = $props();

	const NAMES: Record<LinkKind, string> = {
		entry: 'Entry',
		url: 'URL',
		email: 'Email',
		phone: 'Phone',
		asset: 'File',
	};

	const to = $derived(field.kind === 'link' ? field.to : []);
	const link = $derived<Link>(
		values[field.handle] ?? { type: to[0], value: null },
	);
	const entryTypes = $derived(
		schema.components.filter(c => c.entry).map(c => c.handle),
	);
	// like the website: external urls and files in a new tab
	const external = $derived(
		link.type === 'asset' ||
			(link.type === 'url' &&
				String(link.value ?? '').startsWith('http') &&
				!Object.values(schema.siteUrls).some(
					b => b && String(link.value).startsWith(b),
				)),
	);
	const newTab = $derived(
		link.newTab ?? (field.kind === 'link' && field.newTab && external),
	);

	let asset = $state<Asset | null>(null);
	let picking = $state(false);

	$effect(() => {
		if (link.type !== 'asset' || typeof link.value !== 'number') {
			asset = null;
			return;
		}
		listAssets({ ids: String(link.value) }).then(
			a => (asset = a[0] ?? null),
		);
	});

	function set(change: Partial<Link>) {
		values[field.handle] = { ...link, ...change };
	}

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

<div class="link">
	<div class="row">
		{#if to.length > 1}
			<select
				value={link.type}
				onchange={e => setType(e.currentTarget.value as LinkKind)}
			>
				{#each to as t (t)}
					<option value={t}>{NAMES[t]}</option>
				{/each}
			</select>
		{/if}

		<div class="value">
			{#if link.type === 'entry'}
				{#if typeof link.value === 'number'}
					<div class="chip">
						<span>
							{refs[link.value]?.title ?? `#${link.value}`}
						</span>
						<button
							class="icon"
							title="Remove"
							onclick={() => set({ value: null })}
						>
							<Icon name="delete" />
						</button>
					</div>
				{:else}
					<EntrySearch
						{schema}
						types={entryTypes}
						onpick={pickEntry}
					/>
				{/if}
			{:else if link.type === 'asset'}
				{#if asset}
					<div class="chip">
						<span class="thumb"><Thumb {asset} /></span>
						<span>{asset.filename}</span>
						<button
							class="icon"
							title="Remove"
							onclick={() => set({ value: null })}
						>
							<Icon name="delete" />
						</button>
					</div>
				{:else}
					<button onclick={() => (picking = true)}>
						Choose file
					</button>
				{/if}
			{:else}
				<input
					type={link.type === 'email'
						? 'email'
						: link.type === 'phone'
							? 'tel'
							: 'url'}
					placeholder={link.type === 'url'
						? 'https://…'
						: link.type === 'email'
							? 'name@example.ch'
							: '+41 31 …'}
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

	<label class="tab">
		<input
			type="checkbox"
			checked={newTab}
			onchange={e => set({ newTab: e.currentTarget.checked })}
		/>
		Open in a new tab
	</label>
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

	.chip {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 5px 4px 5px 12px;
		border-radius: 2px;
		background: var(--input);

		button {
			margin-left: auto;
		}
	}

	.thumb {
		width: 32px;
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 14px;
		color: var(--muted);
	}
</style>
