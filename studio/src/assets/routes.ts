import { inLayout } from '../core/router.svelte';
import { Shell } from '../shell/layouts';

export function registerAssets() {
	inLayout(Shell, [['/assets', () => import('./AssetsPage.svelte')]]);
}
