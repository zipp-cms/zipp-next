import { getContext, setContext, type Snippet } from 'svelte';

/**
 * What the frame of the pages puts before every page's title, the menu
 * button of its hidden sidebar: the PageTops of its pages show it.
 */
export type PageLead = { content: Snippet | null };

const KEY = Symbol('page-lead');

export function providePageLead(lead: PageLead) {
	setContext(KEY, lead);
}

/** The frame's lead, none outside of a frame. */
export function pageLead(): PageLead | undefined {
	return getContext(KEY);
}
