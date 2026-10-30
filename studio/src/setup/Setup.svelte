<script module lang="ts">
	import type { Schema as SchemaT } from '../core/schema';
	import { listEntries } from '../entries/api';

	/** A section of its own: its first entries come with the page. */
	export async function loadProps(props: {
		schema: SchemaT;
		tab?: string;
		handle?: string;
	}) {
		if (props.tab !== 'sections' || !props.handle) return {};
		const section = props.schema.sections.find(
			s => s.handle === props.handle,
		);
		if (!section) throw new Error(`There is no section ${props.handle}`);
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
	import PageTop from 'zipp-ui/PageTop.svelte';
	import { SETUP_TABS } from '../shell/nav';
	import { componentOf, sectionOf } from '../core/schema';
	import ComponentPage from './ComponentPage.svelte';
	import ComponentsTab from './ComponentsTab.svelte';
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
		tab = 'sections',
		handle = null,
		list,
	}: {
		schema: Schema;
		tab?: string;
		handle?: string | null;
		/** the entries of a section, see `loadProps` */
		list?: EntryList;
	} = $props();

	const component = $derived(
		handle ? componentOf(schema, handle) : undefined,
	);

	// Setup, the tab, the section or type of a page of its own
	const crumbs = $derived.by(() => {
		const name = Object.fromEntries(SETUP_TABS)[tab] ?? tab;
		const out: { label: string; href?: string }[] = [
			{ label: 'Setup', href: '/admin/setup/sections' },
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

<PageTop {crumbs} note="Read only, the content model is code" />

{#if tab === 'sections' && handle && list}
	<SetupSection {schema} {handle} {list} />
{:else if tab === 'sections'}
	<SectionsTab {schema} />
{:else if (tab === 'types' || tab === 'blocks') && handle}
	{#if component}
		<ComponentPage {schema} {component} />
	{:else}
		<p class="error">
			No {tab === 'types' ? 'entry type' : 'block'}
			{handle}
		</p>
	{/if}
{:else if tab === 'types' || tab === 'blocks'}
	<ComponentsTab {schema} {tab} />
{:else}
	<SitesTab {schema} />
{/if}
