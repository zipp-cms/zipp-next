/**
 * What the flyout on the right shows, like Craft's slideout: an entry
 * with its editor or an asset with its details. `saved` counts saves, so
 * fields that show the thing load it again.
 */
export const flyout = $state<{
	open: { kind: 'entry' | 'asset'; id: number } | null;
	saved: number;
}>({ open: null, saved: 0 });

export function openEntry(id: number) {
	flyout.open = { kind: 'entry', id };
}

export function openAsset(id: number) {
	flyout.open = { kind: 'asset', id };
}

export function closeFlyout() {
	flyout.open = null;
	flyout.saved++;
}
