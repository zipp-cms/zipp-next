<script lang="ts">
	import Links from './Links.svelte';
	import type { Schema } from '../core/schema';
	import SchemaIcon from '../core/SchemaIcon.svelte';
	import { blocksOf, gqlType, usedIn, visibleEntryTypes } from './setup';
	import Table from 'zipp-ui/Table.svelte';
	import { componentPath } from '../core/paths';
	import { t } from '../i18n/index.svelte';

	/**
	 * The entry types or the blocks, with where they are used. Inline
	 * groups are no blocks to reuse, their parent's page links them.
	 */
	let { schema, tab }: { schema: Schema; tab: 'types' | 'blocks' } = $props();

	const components = $derived(
		tab === 'types' ? visibleEntryTypes(schema) : blocksOf(schema),
	);
</script>

<div class="card">
	<Table
		rows={components}
		key={c => c.handle}
		columns={[
			t('common.name'),
			t('setup.handle'),
			t('setup.fields'),
			t('setup.usedIn'),
			'GraphQL',
		]}
		href={componentPath}
	>
		{#snippet row(c)}
			<td>
				<a class="name" href={componentPath(c)}>
					<SchemaIcon icon={c.icon} size={14} />
					{c.name}
				</a>
			</td>
			<td><code>{c.handle}</code></td>
			<td>{c.fields.length}</td>
			<td>
				<Links links={usedIn(schema, c)} empty="–" />
			</td>
			<td><code>{gqlType(c)}</code></td>
		{/snippet}
	</Table>
</div>

<style lang="scss">
	.name {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
	}
</style>
