<script module>
	import {
		createGlobalLoader,
		createTranslations,
	} from 'crelte/translations';

	export const templates = import.meta.glob('@/templates/*.svelte');

	// the texts come from zipp's translations, see global.graphql
	export const plugins = [createTranslations({ loader: createGlobalLoader() })];

	/** @type {import('crelte').Config} */
	export const config = {
		preloadOnMouseOver: true,
	};
</script>

<script>
	import Footer from './components/Footer.svelte';
	import Header from './components/Header.svelte';

	/** @type {import('crelte').AppProps} */
	let { route } = $props();

	let entry = $derived($route.entry);
	let Template = $derived($route.template.default);
	let templateData = $derived($route.loadedData);
</script>

<svelte:head>
	<!-- the seo group of pages, the title as fallback -->
	<title>{entry?.seo?.title || entry?.title || 'Not Found'}</title>
	{#if entry?.seo?.description}
		<meta name="description" content={entry.seo.description} />
	{/if}
</svelte:head>

<Header />

<!-- update entire component if page changes -->
{#key entry?.url}
	<div class="app">
		<Template {entry} {...templateData} />
	</div>
{/key}

<Footer />
