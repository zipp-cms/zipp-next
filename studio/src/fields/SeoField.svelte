<script lang="ts">
	import {
		seoDefaults,
		type Field,
		type SeoDefaults,
		type Values,
	} from '../api';
	import { route, site } from '../route.svelte';
	import AssetField from './AssetField.svelte';

	/**
	 * The SEO of a single entry, like the Craft plugin's field: every value
	 * is optional, the inherited one is the placeholder; a search result
	 * shows how it looks.
	 */
	let { field, values }: { field: Field; values: Values } = $props();

	const ROBOTS = [
		'index, follow',
		'noindex, follow',
		'index, nofollow',
		'noindex, nofollow',
	];

	let defaults = $state<SeoDefaults | null>(null);
	let social = $state(false);

	// the field's values, always an object
	$effect.pre(() => {
		if (!values[field.handle]) values[field.handle] = {};
	});
	const seo = $derived((values[field.handle] ?? {}) as Values);

	$effect(() => {
		const r = route.current;
		if (r.page !== 'entry') return;
		seoDefaults(r.id, site.current)
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

	const sub = (handle: string, images = true): Field => ({
		handle,
		label: handle,
		required: false,
		indexed: false,
		translatable: false,
		kind: 'assets',
		images,
		max: 1,
	});
</script>

{#if values[field.handle]}
	<div class="seo">
		<div class="preview" aria-label="Search result">
			<span class="url">{defaults?.url ?? ''}</span>
			<span class="title">{title}</span>
			<span class="description">{description}</span>
		</div>

		<label>
			<span>
				Title <small class:over={title.length > 60}>
					{title.length} / 60
				</small>
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
			<AssetField field={sub('image')} values={seo} />
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
				<AssetField field={sub('socialImage')} values={seo} />
			</div>
		{/if}
	</div>
{/if}

<style>
	.seo {
		display: grid;
		gap: 12px;
	}

	.preview {
		display: grid;
		gap: 2px;
		padding: 12px 14px;
		background: white;
		border: 1px solid var(--line);
		border-radius: 2px;
		font-family: Arial, sans-serif;
	}

	.url {
		font-size: 13px;
		color: #4d5156;
	}

	.title {
		font-size: 19px;
		color: #1a0dab;
	}

	.description {
		font-size: 14px;
		color: #4d5156;
	}

	label,
	.row {
		display: grid;
		gap: 6px;
	}

	label > span,
	.row > span {
		display: flex;
		justify-content: space-between;
		font-size: 14px;
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
