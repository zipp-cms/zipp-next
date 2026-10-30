import { inLayout } from '../core/router.svelte';
import { Shell } from '../shell/layouts';

/** The trash: deleted entries and assets, to restore. */
export function registerTrash() {
	inLayout(Shell, [['/admin/trash', () => import('./Trash.svelte')]]);
}
