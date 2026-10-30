<script lang="ts">
	import { view } from '../core/router.svelte';
	import PageLoader from './PageLoader.svelte';

	/** The page of the router in its frame, see core/router.svelte.ts. */
	const page = $derived(view.page!);
</script>

<PageLoader loading={view.loading} />

<!-- the frame stays from page to page, the page is new -->
{#if page.layout}
	<page.layout {...page.props}>
		{#key page.key}
			<page.component {...page.props} />
		{/key}
	</page.layout>
{:else}
	{#key page.key}
		<page.component {...page.props} />
	{/key}
{/if}
