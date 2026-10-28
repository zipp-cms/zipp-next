<script lang="ts">
	import { getTranslations, putTranslation, type Schema } from '../api';
	import { shown, site } from '../route.svelte';

	/**
	 * The texts of the frontend. The keys come from the code, editors fill
	 * in the site of the switch on top, next to the texts of another site;
	 * an empty text is the default of the schema.
	 */
	let { schema }: { schema: Schema } = $props();

	let saved = $state<Record<string, Record<string, Record<string, string>>>>(
		{},
	);
	let error = $state('');
	let done = $state('');

	$effect(() => {
		shown.section = 'translations';
	});

	getTranslations()
		.then(t => (saved = t))
		.catch(e => (error = e.message));

	function value(site: string, ns: string, key: string) {
		return saved[site]?.[ns]?.[key] ?? '';
	}

	const primary = $derived(schema.sites[0].handle);
	// the site to translate from, the primary one unless that is edited
	let chosen = $state('');
	const other = $derived(
		chosen && chosen !== site.current
			? chosen
			: (schema.sites.find(s => s.handle !== site.current)?.handle ?? ''),
	);
	const siteName = (h: string) =>
		schema.sites.find(s => s.handle === h)?.name;

	/** What the website shows: the saved text, else the default. */
	function shownText(site: string, ns: string, key: string, def: string) {
		return value(site, ns, key) || def;
	}

	const missing = $derived(
		site.current === primary
			? 0
			: schema.translations
					.flatMap(g =>
						g.keys.map(([k]) =>
							value(site.current, g.namespace, k),
						),
					)
					.filter(v => !v).length,
	);

	async function save(site: string, ns: string, key: string, text: string) {
		if (text === value(site, ns, key)) return;
		try {
			await putTranslation(site, ns, key, text);
			((saved[site] ??= {})[ns] ??= {})[key] = text;
			done = `${key} saved`;
			setTimeout(() => (done = ''), 1500);
		} catch (e: any) {
			error = e.message;
		}
	}
</script>

<div class="page-top">
	<h1>Translations</h1>
	{#if missing}<span class="missing">{missing} not translated</span>{/if}
	{#if done}<span class="done">{done}</span>{/if}
</div>

{#if error}<p class="error">{error}</p>{/if}

{#each schema.translations as group (group.namespace)}
	<div class="card">
		<h2>{group.namespace}</h2>
		<table>
			<thead>
				<tr>
					<th>Key</th>
					{#if other}
						<th>
							<select
								value={other}
								onchange={e => (chosen = e.currentTarget.value)}
								title="Compare with"
							>
								{#each schema.sites.filter(s => s.handle !== site.current) as s (s.handle)}
									<option value={s.handle}>{s.name}</option>
								{/each}
							</select>
						</th>
					{/if}
					<th>{siteName(site.current)}</th>
				</tr>
			</thead>
			<tbody>
				{#each group.keys as [key, fallback] (key)}
					{@const own = value(site.current, group.namespace, key)}
					<tr>
						<td class="key">{key}</td>
						{#if other}
							<td class="other">
								{shownText(
									other,
									group.namespace,
									key,
									fallback,
								)}
							</td>
						{/if}
						<td>
							<input
								class:empty={!own && site.current !== primary}
								value={own}
								placeholder={fallback}
								onchange={e =>
									save(
										site.current,
										group.namespace,
										key,
										e.currentTarget.value,
									)}
							/>
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
{/each}

<style lang="scss">
	.card {
		margin-bottom: 20px;
	}

	table {
		width: 100%;
		border-collapse: collapse;
	}

	th {
		padding: 6px 8px;
		text-align: left;
		font-weight: 600;
		font-size: 14px;
	}

	td {
		padding: 4px 8px;
	}

	.key {
		width: 14rem;
		font-family: monospace;
		font-size: 13px;
		color: var(--muted);
	}

	th select {
		width: auto;
		padding: 4px 8px;
		font-weight: 600;
	}

	.other {
		width: 40%;
		color: var(--muted);
	}

	// not translated: the website shows the default of the code
	input.empty {
		box-shadow: inset 3px 0 #e8a33d;
	}

	.missing {
		color: #b07020;
		font-size: 14px;
	}

	.done {
		color: var(--muted);
		font-size: 14px;
	}
</style>
