import type { Component, Values } from '../core/schema';
import { tabShown } from './conditions';

/**
 * The tabs of an entry type or a block whose conditions hold, and the
 * open one: the first when the open one hides. Made while a component
 * mounts, like its state.
 */
export class Tabs {
	current = $state('');
	shown = $derived.by(() => {
		const c = this.component();
		return (c?.tabs ?? []).filter(t => tabShown(c!, t, this.values()));
	});
	/** the fields of the open tab, null without tabs: all */
	fields = $derived(
		this.shown.find(t => t.name === this.current)?.fields ?? null,
	);
	/** whether the open tab is one card */
	card = $derived(!!this.shown.find(t => t.name === this.current)?.card);

	constructor(
		private component: () => Component | undefined,
		private values: () => Values,
		first = '',
	) {
		this.current = first;
		$effect(() => {
			const tabs = this.shown;
			if (tabs.length && !tabs.some(t => t.name === this.current))
				this.current = tabs[0].name;
		});
	}
}
