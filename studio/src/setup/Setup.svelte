<script lang="ts">
	import type { Schema } from '../core/schema';
	import PageTop from '../ui/PageTop.svelte';
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
	}: { schema: Schema; tab?: string; handle?: string | null } = $props();

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

{#if tab === 'sections' && handle}
	<SetupSection {schema} {handle} />
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
