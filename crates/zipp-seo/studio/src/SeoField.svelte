<script>
	/**
	 * The SEO of a single entry, like the Craft plugin's field: every value
	 * is optional, the inherited one is the placeholder; a search result
	 * shows how it looks. The images are the studio's own field, lent by
	 * the host.
	 */
	let { value, onchange, entryId, site, host } = $props();

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

<div class="seo">
	<div class="preview" aria-label="Search result">
		<span class="url">{defaults?.url ?? ''}</span>
		<span class="title">{title}</span>
		<span class="description">{description}</span>
	</div>

	<label>
		<span>
			Title <small class:over={title.length > 60}>{title.length} / 60</small>
		</span>
		<input bind:value={seo.title} placeholder={defaults?.title ?? ''} />
	</label>
	<label>
		<span>
			Description
			<small class:over={description.length > 160}>
				{description.length} / 160
			</small>
		</span>
		<textarea
			rows="3"
			bind:value={seo.description}
			placeholder={defaults?.description ?? ''}
		></textarea>
	</label>
	<div class="row">
		<span>Image</span>
		<div use:image={'image'}></div>
	</div>
	<label>
		<span>Robots</span>
		<select bind:value={seo.robots}>
			<option value={undefined}>
				Inherit: {defaults?.robots ?? 'index, follow'}
			</option>
			{#each ROBOTS as r (r)}<option value={r}>{r}</option>{/each}
		</select>
	</label>
	<label>
		<span>Canonical URL</span>
		<input
			bind:value={seo.canonicalUrl}
			placeholder={defaults?.canonicalUrl ?? ''}
		/>
	</label>

	<button type="button" class="link" onclick={() => (social = !social)}>
		{social ? 'Hide' : 'Show'} sharing (Open Graph, X)
	</button>
	{#if social}
		<label>
			<span>Social title</span>
			<input bind:value={seo.socialTitle} placeholder={title} />
		</label>
		<label>
			<span>Social description</span>
			<textarea
				rows="2"
				bind:value={seo.socialDescription}
				placeholder={description}
			></textarea>
		</label>
		<div class="row">
			<span>Social image</span>
			<div use:image={'socialImage'}></div>
		</div>
	{/if}
</div>

<style>
	.seo {
		display: grid;
		gap: 0.75rem;
	}

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

	label,
	.row {
		display: grid;
		gap: 0.375rem;
	}

	label > span,
	.row > span {
		display: flex;
		justify-content: space-between;
		font-size: 0.875rem;
	}

	small {
		color: var(--muted);
	}

	small.over {
		color: var(--danger);
	}

	.link {
		justify-self: start;
	}
</style>
