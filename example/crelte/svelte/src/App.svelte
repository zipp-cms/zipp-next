<script module>
	import {
		createGlobalLoader,
		createTranslations,
	} from 'crelte/translations';
	import { searchEntry } from './search.js';

	export const templates = import.meta.glob('@/templates/*.svelte');

	// the texts come from zipp's translations, see global.graphql
	export const plugins = [
		createTranslations({ loader: createGlobalLoader() }),
	];

	// the search page isn't an entry of zipp, see search.js
	/** @param {import('crelte').Crelte} crelte */
	export function init(crelte) {
		crelte.events.on('loadEntry', searchEntry);
	}

	/** @type {import('crelte').Config} */
	export const config = {
		preloadOnMouseOver: true,
	};
</script>

<script>
	import Footer from './components/Footer.svelte';
	import Header from './components/Header.svelte';
	import PreviewBridge from './components/PreviewBridge.svelte';
	import { Seo } from 'meierlabs-seo';

	/** @type {import('crelte').AppProps} */
	let { route } = $props();

	let entry = $derived($route.entry);
	let Template = $derived($route.template.default);
	let templateData = $derived($route.loadedData);
</script>

<!-- title, meta tags, links and JSON-LD from zipp's SEO, the npm package
of the Craft plugin meierlabs-seo -->
<Seo seo={entry?.seoJson} />

<PreviewBridge />
<Header />

<!-- update entire component if page changes -->
{#key entry?.url}
	<div class="app">
		<Template {entry} {...templateData} />
	</div>
{/key}

<Footer />
