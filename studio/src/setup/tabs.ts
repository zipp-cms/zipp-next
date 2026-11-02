import type { Schema } from '../core/schema';
import { isAdmin } from '../core/access';
import type { Key } from '../i18n/index.svelte';

/**
 * The pages of Setup, also its crumbs: the handle and its name's key;
 * requests only for admins.
 */
export const SETUP_TABS: [string, Key][] = [
	['overview', 'shell.setupOverview'],
	['sections', 'shell.setupSections'],
	['types', 'shell.setupTypes'],
	['blocks', 'shell.setupBlocks'],
	['fields', 'shell.setupFields'],
	['sites', 'shell.setupSites'],
	['requests', 'shell.setupRequests'],
];

/** The pages of Setup the user sees. */
export function setupTabs(schema: Schema): [string, Key][] {
	return SETUP_TABS.filter(([h]) => h !== 'requests' || isAdmin(schema));
}
