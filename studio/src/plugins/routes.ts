import { inLayout } from '../core/router.svelte';
import { Shell } from '../shell/layouts';

// `/admin/plugins/seo/redirects`: the plugin and its page
const PAGE = /\/admin\/plugins\/(?<plugin>[^/]+)\/(?<page>[^/]+)/;

/** The pages of plugins, their modules show them. */
export function registerPlugins() {
	inLayout(Shell, [[PAGE, () => import('./PluginPage.svelte')]]);
}
