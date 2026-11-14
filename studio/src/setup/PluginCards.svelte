<script lang="ts">
	import type { Schema, Section } from '../core/schema';
	import { mountPlugin } from '../plugins/module';

	/**
	 * The cards plugins add to the setup page of a section: a plugin's
	 * module exports `sectionSetup(target, { schema, section, data, host })`
	 * and returns `{ destroy() }`, see `PluginStudio` in zipp.
	 */
	let { schema, section }: { schema: Schema; section: Section } = $props();

	function card(target: HTMLElement, name: string) {
		const plugin = schema.plugins.find(p => p.name === name)!;
		const props = {
			schema: $state.snapshot(schema),
			section: $state.snapshot(section),
		};
		return mountPlugin(target, plugin, 'sectionSetup', props, true);
	}
</script>

{#each schema.plugins.filter(p => p.module) as p (p.name + section.handle)}
	<div use:card={p.name}></div>
{/each}
