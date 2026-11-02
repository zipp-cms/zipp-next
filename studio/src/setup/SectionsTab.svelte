<script lang="ts">
	import Links from './Links.svelte';
	import { type Schema } from '../core/schema';
	import { accessOf, accessText } from '../core/access';
	import { visibleSections } from './setup';
	import Table from 'zipp-ui/Table.svelte';
	import { setupPath } from '../core/paths';
	import { t } from '../i18n/index.svelte';

	/** The sections the role sees, with how many entries each has. */
	let {
		schema,
		counts,
	}: {
		schema: Schema;
		/** in the primary site, see Setup's `loadProps` */
		counts: Record<string, number>;
	} = $props();

	const sections = $derived(visibleSections(schema));
</script>

<div class="card">
	<Table
		rows={sections}
		key={s => s.handle}
		columns={[
			t('common.name'),
			t('setup.handle'),
			t('setup.kind'),
			t('setup.entryTypes'),
			'URI',
			t('setup.entries'),
			t('setup.youMay'),
		]}
		href={s => setupPath('sections', s.handle)}
	>
		{#snippet row(s)}
			<td>
				<a href={setupPath('sections', s.handle)}>{s.name}</a>
			</td>
			<td><code>{s.handle}</code></td>
			<td>{s.kind}</td>
			<td>
				<Links
					links={s.types.map(type => ({
						text: type,
						href: setupPath('types', type),
					}))}
				/>
			</td>
			<td>
				<code>{s.uri ?? '–'}</code>
				{#each Object.entries(s.type_uris ?? {}) as [type, u] (type)}
					<br />
					<small>
						{type}:
						<code>{u || '/'}</code>
					</small>
				{/each}
			</td>
			<td>{counts[s.handle] ?? ''}</td>
			<td>{accessText(accessOf(schema, s.handle))}</td>
		{/snippet}
	</Table>
</div>
