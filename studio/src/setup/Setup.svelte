<script module lang="ts">
	import type { Schema as SchemaT } from '../core/schema';
	import { listEntries } from '../entries/api';
	import { isAdmin } from '../core/access';
	import { listRequests } from './api';
	import { entryCounts } from './setup';
	import { t } from '../i18n/index.svelte';

	/**
	 * A section of its own: its first entries come with the page; the
	 * overview and the sections their counts, the requests their list.
	 */
	export async function loadProps(props: {
		schema: SchemaT;
		tab?: string;
		handle?: string;
	}) {
		const tab = props.tab ?? 'overview';
		if (tab === 'requests') return { requests: await listRequests() };
		if (tab === 'overview') {
			const admin = isAdmin(props.schema);
			const [counts, requests] = await Promise.all([
				entryCounts(props.schema),
				admin ? listRequests() : Promise.resolve([]),
			]);
			return {
				counts,
				openRequests: requests.filter(r => !r.done).length,
			};
		}
		if (tab === 'sections' && !props.handle)
			return { counts: await entryCounts(props.schema) };
		if (tab !== 'sections' || !props.handle) return {};
		const section = props.schema.sections.find(
			s => s.handle === props.handle,
		);
		if (!section)
			throw new Error(
				t('setup.noSectionThere', { handle: props.handle }),
			);
		const params: Record<string, string> = {
			section: section.handle,
			site: props.schema.sites[0].handle,
			limit: '1000',
		};
		if (section.kind === 'structure') params.order = 'structure';
		return { list: await listEntries(params) };
	}
</script>

<script lang="ts">
	import type { Schema } from '../core/schema';
	import type { EntryList } from '../entries/api';
	import type { ChangeRequest } from './api';
	import PageTop from 'zipp-ui/PageTop.svelte';
	import { SETUP_TABS } from '../shell/nav';
	import { componentOf, sectionOf } from '../core/schema';
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

	// Setup, the tab, the section or type of a page of its own
	const crumbs = $derived.by(() => {
		const key = SETUP_TABS.find(([h]) => h === tab)?.[1];
		const name = key ? t(key) : tab;
		const out: { label: string; href?: string }[] = [
			{ label: t('setup.setup'), href: '/admin/setup' },
			{ label: name, href: `/admin/setup/${tab}` },
		];
		if (handle) {
			const own =
				tab === 'sections'
					? sectionOf(schema, handle)?.name
					: component?.name;
			out.push({ label: own ?? handle });
		}
		return out;
	});
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
