import { getContext, setContext, type Snippet } from 'svelte';

/**
 * The bar of a Flyout for a page in it: its PageTop shows there, one bar
 * with the page's crumbs and actions instead of two.
 */
export type FlyoutBar = { content: Snippet | null };

const KEY = Symbol('flyout-bar');

export function provideFlyoutBar(bar: FlyoutBar) {
	setContext(KEY, bar);
}

/** The bar of the Flyout around, none on a page. */
export function flyoutBar(): FlyoutBar | undefined {
	return getContext(KEY);
}
