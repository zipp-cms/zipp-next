import { getContext, setContext, type Snippet } from 'svelte';

/** A place an outer component shows what an inner one puts there. */
export type Slot = { content: Snippet | null };

/** A slot handed down by context: `provide` it, `get` it inside. */
function snippetSlot(name: string) {
	const key = Symbol(name);
	return {
		provide: (slot: Slot) => void setContext(key, slot),
		/** none outside of the one that provides it */
		get: (): Slot | undefined => getContext(key),
	};
}

/**
 * What the frame of the pages puts before every page's title, the menu
 * button of its hidden sidebar: the PageTops of its pages show it.
 */
export const pageLead = snippetSlot('page-lead');

/**
 * The bar of a Flyout for a page in it: its PageTop shows there, one bar
 * with the page's crumbs and actions instead of two.
 */
export const flyoutBar = snippetSlot('flyout-bar');
