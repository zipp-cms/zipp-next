import { t } from '../i18n/index.svelte';

/**
 * Changes that aren't saved anywhere yet: a new entry, a user, an asset
 * in the flyout. Leaving the page (a link, Back) asks first, closing the
 * tab too. An entry's draft needs none, it is saved on leaving.
 */
const guards = new Set<() => boolean>();

/** `unsaved` says whether there are changes; the result removes it. */
export function guardLeave(unsaved: () => boolean): () => void {
	guards.add(unsaved);
	return () => guards.delete(unsaved);
}

const anyUnsaved = () => [...guards].some(unsaved => unsaved());

/** Whether the page may go: nothing unsaved, or the editor says so. */
export function mayLeave(): boolean {
	if (!anyUnsaved()) return true;
	if (!confirm(t('common.confirmLeave'))) return false;
	// asked once: not again for a redirect of the same way
	guards.clear();
	return true;
}

if (typeof window !== 'undefined') {
	window.addEventListener('beforeunload', e => {
		if (anyUnsaved()) e.preventDefault();
	});
}
