<script lang="ts">
	import { getTranslations, putTranslation, type Schema } from '../api';
	import { shown } from '../route.svelte';

	/**
	 * The texts of the frontend. The keys come from the code, editors fill
	 * in every site; an empty text is the default of the schema.
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
					{#each schema.sites as s (s.handle)}<th>{s.name}</th>{/each}
				</tr>
			</thead>
			<tbody>
				{#each group.keys as [key, fallback] (key)}
					<tr>
						<td class="key">{key}</td>
						{#each schema.sites as s (s.handle)}
							<td>
								<input
									value={value(
										s.handle,
										group.namespace,
										key,
									)}
									placeholder={fallback}
									onchange={e =>
										save(
											s.handle,
											group.namespace,
											key,
											e.currentTarget.value,
										)}
								/>
							</td>
						{/each}
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
{/each}

<style>
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

	.done {
		color: var(--muted);
		font-size: 14px;
	}
</style>
