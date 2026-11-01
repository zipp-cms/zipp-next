<script lang="ts">
	import { view } from '../core/router.svelte';
	import { session } from '../core/session.svelte';
	import PageLoader from './PageLoader.svelte';
	import SessionDialog from '../auth/SessionDialog.svelte';
	import { applyLanguage } from '../i18n/index.svelte';

	/** The page of the router in its frame, see core/router.svelte.ts. */
	const page = $derived(view.page!);

	// again when the user picks another language
	$effect.pre(applyLanguage);
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

<!-- over the page, which stays as it was -->
{#if session.expired}<SessionDialog />{/if}
