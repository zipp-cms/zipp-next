/**
 * The open popups and flyouts, the last one on top. Escape closes only
 * the top one, so a popup in a flyout closes before the flyout.
 */
const layers: (() => void)[] = [];

function onkeydown(e: KeyboardEvent) {
	if (e.key !== 'Escape' || !layers.length) return;
	e.preventDefault();
	layers[layers.length - 1]();
}

/** Adds a layer that `close` closes, the result removes it again. */
export function pushLayer(close: () => void): () => void {
	if (!layers.length) window.addEventListener('keydown', onkeydown);
	layers.push(close);
	return () => {
		const i = layers.indexOf(close);
		if (i >= 0) layers.splice(i, 1);
		if (!layers.length) window.removeEventListener('keydown', onkeydown);
	};
}

/**
 * A Svelte action for the root of a layer: Escape closes it, and the
 * element that had the focus gets it back.
 */
export function layer(_node: HTMLElement, close: () => void) {
	const before = document.activeElement as HTMLElement | null;
	let current = close;
	const remove = pushLayer(() => current());
	return {
		update(next: () => void) {
			current = next;
		},
		destroy() {
			remove();
			if (before?.isConnected) before.focus();
		},
	};
}
