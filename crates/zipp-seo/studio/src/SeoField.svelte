<script>
	import FieldGroup from 'zipp-ui/FieldGroup.svelte';
	import FieldRow from 'zipp-ui/FieldRow.svelte';
	import { texts } from './texts.js';

	/**
	 * The SEO of a single entry, like the Craft plugin's field: every value
	 * is optional, the inherited one is the placeholder; a search result
	 * shows how it looks. The images are the studio's own field, lent by
	 * the host. It looks like a group field: a card with its values in
	 * rows.
	 */
	let { value, onchange, entryId, site, host } = $props();
	// the ids of its inputs, an entry can have more than one SEO field
	const uid = $props.id();
	const tx = texts();

	const ROBOTS = [
		'index, follow',
		'noindex, follow',
		'index, nofollow',
		'noindex, nofollow',
	];

	// the field's values, always an object here; what it sends is its
	// value at once (zipp-ui's fieldModule)
	const seo = $derived(value ?? {});
	const set = (key, v) => onchange({ ...seo, [key]: v });

	let defaults = $state(null);
	let social = $state(false);
	$effect(() => {
		if (entryId === null) return;
		host.request('GET', `/entries/${entryId}/seo?site=${site}`)
			.then(d => (defaults = d))
			.catch(() => {});
	});

	const title = $derived(
		(defaults?.titlePrefix ?? '') +
			(seo.title || defaults?.title || '') +
			(defaults?.titleSuffix ?? ''),
	);
	const description = $derived(
		seo.description || defaults?.description || '',
	);

	/** The studio's images field for one of the values. */
	function image(target, key) {
		const field = {
			handle: key,
			label: tx[key],
			required: false,
			indexed: false,
			translatable: false,
			kind: 'assets',
			images: true,
			max: 1,
		};
		const m = host.mountField(target, {
			field,
			value: seo[key] ?? null,
			onchange: v => set(key, v?.length ? v : undefined),
		});
		$effect(() => m.update({ value: seo[key] ?? null }));
		return { destroy: () => m.destroy() };
	}
</script>

<FieldGroup>
	<FieldRow label={tx.preview}>
		<div class="preview" role="group" aria-label={tx.searchResult}>
			<span class="url">{defaults?.url ?? ''}</span>
			<span class="title">{title}</span>
			<span class="description">{description}</span>
		</div>
	</FieldRow>
	<FieldRow label={tx.title} for="{uid}-title">
		{#snippet info()}
			<span class:over={title.length > 60}>{title.length} / 60</span>
		{/snippet}
		<input
			id="{uid}-title"
			bind:value={() => seo.title, v => set('title', v)}
			placeholder={defaults?.title ?? ''}
		/>
	</FieldRow>
	<FieldRow label={tx.description} for="{uid}-description">
		{#snippet info()}
			<span class:over={description.length > 160}>
				{description.length} / 160
			</span>
		{/snippet}
		<textarea
			id="{uid}-description"
			rows="3"
			bind:value={() => seo.description, v => set('description', v)}
			placeholder={defaults?.description ?? ''}
		></textarea>
	</FieldRow>
	<FieldRow label={tx.image}>
		<div use:image={'image'}></div>
	</FieldRow>
	<FieldRow label={tx.robots} for="{uid}-robots">
		<select
			id="{uid}-robots"
			bind:value={() => seo.robots, v => set('robots', v)}
		>
			<option value={undefined}>
				{tx.inherit}: {defaults?.robots ?? 'index, follow'}
			</option>
			{#each ROBOTS as r (r)}<option value={r}>{r}</option>{/each}
		</select>
	</FieldRow>
	<FieldRow label={tx.canonicalUrl} for="{uid}-canonical">
		<input
			id="{uid}-canonical"
			bind:value={() => seo.canonicalUrl, v => set('canonicalUrl', v)}
			placeholder={defaults?.canonicalUrl ?? ''}
		/>
	</FieldRow>

	<FieldRow>
		<button type="button" class="link" onclick={() => (social = !social)}>
			{social ? tx.hideSharing : tx.showSharing}
		</button>
	</FieldRow>
	{#if social}
		<FieldRow label={tx.socialTitle} for="{uid}-social-title">
			<input
				id="{uid}-social-title"
				bind:value={() => seo.socialTitle, v => set('socialTitle', v)}
				placeholder={title}
			/>
		</FieldRow>
		<FieldRow label={tx.socialDescription} for="{uid}-social-description">
			<textarea
				id="{uid}-social-description"
				rows="2"
				bind:value={
					() => seo.socialDescription,
					v => set('socialDescription', v)
				}
				placeholder={description}
			></textarea>
		</FieldRow>
		<FieldRow label={tx.socialImage}>
			<div use:image={'socialImage'}></div>
		</FieldRow>
	{/if}
</FieldGroup>

<style>
	.preview {
		display: grid;
		gap: 2px;
		padding: 0.75rem 0.875rem;
		background: white;
		border: 1px solid var(--line);
		border-radius: 2px;
		font-family: Arial, sans-serif;
	}

	.url {
		font-size: 0.8125rem;
		color: #4d5156;
	}

	.title {
		font-size: 1.1875rem;
		color: #1a0dab;
	}

	.description {
		font-size: 0.875rem;
		color: #4d5156;
	}

	.over {
		color: var(--danger);
	}
</style>
