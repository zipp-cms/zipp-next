<script>
	import FieldGroup from 'zipp-ui/FieldGroup.svelte';
	import FieldRow from 'zipp-ui/FieldRow.svelte';

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

	const ROBOTS = [
		'index, follow',
		'noindex, follow',
		'index, nofollow',
		'noindex, nofollow',
	];

	// the field's values, always an object here
	let seo = $state({ ...(value ?? {}) });
	let sent = JSON.stringify(value ?? {});
	// a new value from outside, like a restored revision
	$effect.pre(() => {
		const outside = JSON.stringify(value ?? {});
		if (outside !== sent) {
			sent = outside;
			seo = { ...(value ?? {}) };
		}
	});
	$effect(() => {
		const now = JSON.stringify(seo);
		if (now === sent) return;
		sent = now;
		onchange($state.snapshot(seo));
	});

	let defaults = $state(null);
	let social = $state(false);
	$effect(() => {
		if (entryId === null) return;
		host
			.request('GET', `/entries/${entryId}/seo?site=${site}`)
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
			label: key,
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
			onchange: v => (seo[key] = v?.length ? v : undefined),
		});
		$effect(() => m.update({ value: seo[key] ?? null }));
		return { destroy: () => m.destroy() };
	}
</script>

<FieldGroup>
	<FieldRow label="Preview">
		<div class="preview">
			<span class="url">{defaults?.url ?? ''}</span>
			<span class="title">{title}</span>
			<span class="description">{description}</span>
		</div>
	</FieldRow>
	<FieldRow label="Title" for="{uid}-title">
		{#snippet info()}
			<span class:over={title.length > 60}>{title.length} / 60</span>
		{/snippet}
		<input
			id="{uid}-title"
			bind:value={seo.title}
			placeholder={defaults?.title ?? ''}
		/>
	</FieldRow>
	<FieldRow label="Description" for="{uid}-description">
		{#snippet info()}
			<span class:over={description.length > 160}>
				{description.length} / 160
			</span>
		{/snippet}
		<textarea
			id="{uid}-description"
			rows="3"
			bind:value={seo.description}
			placeholder={defaults?.description ?? ''}
		></textarea>
	</FieldRow>
	<FieldRow label="Image">
		<div use:image={'image'}></div>
	</FieldRow>
	<FieldRow label="Robots" for="{uid}-robots">
		<select id="{uid}-robots" bind:value={seo.robots}>
			<option value={undefined}>
				Inherit: {defaults?.robots ?? 'index, follow'}
			</option>
			{#each ROBOTS as r (r)}<option value={r}>{r}</option>{/each}
		</select>
	</FieldRow>
	<FieldRow label="Canonical URL" for="{uid}-canonical">
		<input
			id="{uid}-canonical"
			bind:value={seo.canonicalUrl}
			placeholder={defaults?.canonicalUrl ?? ''}
		/>
	</FieldRow>

	<FieldRow>
		<button type="button" class="link" onclick={() => (social = !social)}>
			{social ? 'Hide' : 'Show'} sharing (Open Graph, X)
		</button>
	</FieldRow>
	{#if social}
		<FieldRow label="Social title" for="{uid}-social-title">
			<input
				id="{uid}-social-title"
				bind:value={seo.socialTitle}
				placeholder={title}
			/>
		</FieldRow>
		<FieldRow label="Social description" for="{uid}-social-description">
			<textarea
				id="{uid}-social-description"
				rows="2"
				bind:value={seo.socialDescription}
				placeholder={description}
			></textarea>
		</FieldRow>
		<FieldRow label="Social image">
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
