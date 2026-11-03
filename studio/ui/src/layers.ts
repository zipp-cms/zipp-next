/**
 * The open popups and flyouts, the last one on top. Escape closes only
 * the top one, so a popup in a flyout closes before the flyout. A plugin
 * bundles its own zipp-ui, so the stack lives on `globalThis` (like
 * `texts.svelte.ts`): Escape in a plugin's popover closes only that.
 */
type Layers = { stack: (() => void)[]; onkeydown: (e: KeyboardEvent) => void };

const KEY = Symbol.for('zipp-ui-layers');
const global = globalThis as { [KEY]?: Layers };

const layers: Layers = (global[KEY] ??= {
	stack: [],
	onkeydown(e: KeyboardEvent) {
		const stack = layers.stack;
		if (e.key !== 'Escape' || !stack.length) return;
		// a modal dialog (the session ended) is above every layer, the
		// layers under it stay as they are
		if (document.querySelector('dialog:modal')) return;
		e.preventDefault();
		stack[stack.length - 1]();
	},
});

/** Adds a layer that `close` closes, the result removes it again. */
function pushLayer(close: () => void): () => void {
	const { stack, onkeydown } = layers;
	if (!stack.length) window.addEventListener('keydown', onkeydown);
	stack.push(close);
	return () => {
		const i = stack.indexOf(close);
		if (i >= 0) stack.splice(i, 1);
		if (!stack.length) window.removeEventListener('keydown', onkeydown);
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
