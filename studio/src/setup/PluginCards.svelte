<script lang="ts">
	import type { Schema, Section } from '../core/schema';
	import { host } from '../fields/host.svelte';

	/**
	 * The cards plugins add to the setup page of a section: a plugin's
	 * module exports `sectionSetup(target, { schema, section, data, host })`
	 * and returns `{ destroy() }`, see `PluginStudio` in zipp.
	 */
	let { schema, section }: { schema: Schema; section: Section } = $props();

	type Mounted = { destroy(): void };

	function card(target: HTMLElement, name: string) {
		const plugin = schema.plugins.find(p => p.name === name)!;
		let mounted: Mounted | null = null;
		let gone = false;
		import(/* @vite-ignore */ `/_plugins/${name}.js`)
			.then(m => {
				if (gone || !m.sectionSetup) return;
				mounted = m.sectionSetup(target, {
					schema: $state.snapshot(schema),
					section: $state.snapshot(section),
					data: plugin.data,
					host,
				});
			})
			.catch(e => (target.textContent = `${name}: ${e.message}`));
		return {
			destroy() {
				gone = true;
				mounted?.destroy();
			},
		};
	}
</script>

{#each schema.plugins as p (p.name + section.handle)}
	<div use:card={p.name}></div>
{/each}
