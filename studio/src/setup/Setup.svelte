<script module lang="ts">
	import type { Schema as SchemaT } from '../core/schema';
	import { loadSetup } from './setup';

	export function loadProps(props: {
		schema: SchemaT;
		tab?: string;
		handle?: string;
	}) {
		return loadSetup(props.schema, props.tab ?? 'overview', props.handle);
	}
</script>

<script lang="ts">
	import type { Schema } from '../core/schema';
	import type { EntryList } from '../entries/api';
	import type { ChangeRequest } from './api';
	import PageTop from 'zipp-ui/PageTop.svelte';
	import { setupCrumbs } from './setup';
	import { t } from '../i18n/index.svelte';
	import { componentOf } from '../core/schema';
	import { current } from '../core/router.svelte';
	import ComponentPage from './ComponentPage.svelte';
	import ComponentsTab from './ComponentsTab.svelte';
	import FieldsTab from './FieldsTab.svelte';
	import OverviewTab from './OverviewTab.svelte';
	import RequestChange from './RequestChange.svelte';
	import RequestsTab from './RequestsTab.svelte';
	import SectionsTab from './SectionsTab.svelte';
	import SetupSection from './SetupSection.svelte';
	import SitesTab from './SitesTab.svelte';

	/**
	 * The content model, read only like Craft's settings when admin
	 * changes are off: it is code, this shows what the code says, with the
	 * handles and GraphQL types a frontend uses.
	 */
	let {
		schema,
		tab = 'overview',
		handle = null,
		list,
		counts = {},
		openRequests,
		requests,
	}: {
		schema: Schema;
		tab?: string;
		handle?: string | null;
		/** the entries of a section, see `loadProps` */
		list?: EntryList;
		/** entries per section */
		counts?: Record<string, number>;
		openRequests?: number;
		requests?: ChangeRequest[];
	} = $props();

	const component = $derived(
		handle ? componentOf(schema, handle) : undefined,
	);
	const crumbs = $derived(setupCrumbs(schema, tab, handle));
</script>

<PageTop {crumbs} note={t('setup.readOnly')}>
	{#snippet actions()}
		{#if tab !== 'requests'}
			<RequestChange
				page={current.path}
				context={crumbs
					.slice(1)
					.map(c => c.label)
					.join(' › ')}
			/>
		{/if}
	{/snippet}
</PageTop>

{#if tab === 'overview'}
	<OverviewTab {schema} {counts} {openRequests} />
{:else if tab === 'sections' && handle && list}
	<SetupSection {schema} {handle} {list} />
{:else if tab === 'sections'}
	<SectionsTab {schema} {counts} />
{:else if tab === 'fields'}
	<FieldsTab {schema} />
{:else if tab === 'requests' && requests}
	<RequestsTab {requests} />
{:else if (tab === 'types' || tab === 'blocks') && handle}
	{#if component}
		<ComponentPage {schema} {component} />
	{:else}
		<p class="error">
			{t(tab === 'types' ? 'setup.noEntryType' : 'setup.noBlock', {
				handle,
			})}
		</p>
	{/if}
{:else if tab === 'types' || tab === 'blocks'}
	<ComponentsTab {schema} {tab} />
{:else}
	<SitesTab {schema} />
{/if}
