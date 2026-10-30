<script>
	import Table from 'zipp-ui/Table.svelte';

	/**
	 * The SEO of a section on its setup page: the defaults of the code for
	 * the section and its entry types, read only like all of the setup.
	 */
	let { schema, section, data } = $props();

	const rows = $derived.by(() => {
		const out = [{ name: 'Section', d: data.sections[section.handle] }];
		for (const t of section.types) {
			if (!data.types[t]) continue;
			const name = schema.components.find(c => c.handle === t)?.name;
			out.push({ name: name ?? t, d: data.types[t] });
		}
		return out;
	});
	const pages = $derived(
		section.uri !== null || Object.keys(section.type_uris ?? {}).length,
	);
	const val = s => s ?? '–';
</script>

{#if pages}
	<div class="card seo">
		<h2>SEO</h2>
		<Table
			{rows}
			key={r => r.name}
			columns={[
				'',
				'Title',
				'Description',
				'Image from',
				'Robots',
				'Sitemap',
			]}
		>
			{#snippet row(r)}
				<td>{r.name}</td>
				<td>
					<code>{val(r.d?.titlePattern)}</code>
					{#if r.d?.titleFrom}
						<br />
						from
						<code>{r.d.titleFrom}</code>
					{/if}
				</td>
				<td>
					{#if r.d?.descriptionFrom}
						from <code>{r.d.descriptionFrom}</code>
					{:else}
						{val(r.d?.description)}
					{/if}
				</td>
				<td><code>{val(r.d?.imageFrom)}</code></td>
				<td>{val(r.d?.robots)}</td>
				<td>{r.d?.sitemap === false ? 'No' : 'Yes'}</td>
			{/snippet}
		</Table>
		<p class="muted">
			The defaults of the code, from the SEO plugin. Editors override them
			per site in the
			<a href="/admin/entries/seoSettings">SEO settings</a>
			(Per section and entry type), an entry's SEO field overrides both; empty
			values come from the SEO settings.
		</p>
	</div>
{/if}

<style>
	.seo {
		margin-bottom: 1.25rem;
		overflow-x: auto;
	}

	.muted {
		color: var(--muted);
		font-size: 0.875rem;
	}
</style>
