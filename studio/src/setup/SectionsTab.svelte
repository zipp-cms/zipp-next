<script lang="ts">
	import { type Schema } from '../core/schema';
	import { accessOf } from '../core/access';
	import { accessText, visibleSections } from './setup';
	import Table from 'zipp-ui/Table.svelte';
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
		href={s => `/admin/setup/sections/${s.handle}`}
	>
		{#snippet row(s)}
			<td>
				<a href="/admin/setup/sections/{s.handle}">{s.name}</a>
			</td>
			<td><code>{s.handle}</code></td>
			<td>{s.kind}</td>
			<td>
				{#each s.types as type, i (type)}
					{i ? ', ' : ''}
					<a href="/admin/setup/types/{type}">{type}</a>
				{/each}
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
