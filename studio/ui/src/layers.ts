/**
 * The open popups and flyouts, the last one on top. Escape closes only
 * the top one, so a popup in a flyout closes before the flyout.
 */
const layers: (() => void)[] = [];

function onkeydown(e: KeyboardEvent) {
	if (e.key !== 'Escape' || !layers.length) return;
	// a modal dialog (the session ended) is above every layer, the
	// layers under it stay as they are
	if (document.querySelector('dialog:modal')) return;
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

const FOCUSABLE =
	'a[href], button:not(:disabled), input:not(:disabled), ' +
	'select:not(:disabled), textarea:not(:disabled), ' +
	'[tabindex]:not([tabindex="-1"]), [contenteditable="true"]';

/**
 * For the keydown of a modal layer: Tab stays in `root`, from the last
 * to the first and back.
 */
export function trapTab(e: KeyboardEvent, root: HTMLElement) {
	if (e.key !== 'Tab') return;
	// shown, and not only for the arrow keys (`tabindex="-1"`)
	const all = [...root.querySelectorAll<HTMLElement>(FOCUSABLE)].filter(
		el => el.offsetParent !== null && !el.matches('[tabindex="-1"]'),
	);
	if (!all.length) return;
	const [first, last] = [all[0], all[all.length - 1]];
	const at = document.activeElement;
	if (e.shiftKey && (at === first || at === root)) {
		e.preventDefault();
		last.focus();
	} else if (!e.shiftKey && at === last) {
		e.preventDefault();
		first.focus();
	}
}
