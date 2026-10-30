import { inLayout } from '../core/router.svelte';
import { Shell } from '../shell/layouts';

export function registerAi() {
	inLayout(Shell, [['/ai', () => import('./Ai.svelte')]]);
}
