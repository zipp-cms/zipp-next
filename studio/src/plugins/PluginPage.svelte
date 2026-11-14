<script module lang="ts">
	import { pluginPages } from '../core/access';
	import type { Schema as SchemaT } from '../core/schema';
	import { t } from '../i18n/index.svelte';

	/** The page of a plugin the user sees, by the path. */
	export function loadProps(props: {
		schema: SchemaT;
		plugin: string;
		page: string;
	}) {
		const found = pluginPages(props.schema).find(
			p => p.plugin === props.plugin && p.page.handle === props.page,
		);
		if (!found) throw new Error(t('shell.noPage'));
		return { found };
	}
</script>

<script lang="ts">
	import { host } from '../fields/host.svelte';
	import type { PluginPage, Schema } from '../core/schema';

	/**
	 * A page of a plugin: its module exports `page(target, { schema,
	 * page, data, host })` and returns `{ destroy() }`, see `PluginPage`
	 * in zipp.
	 */
	let {
		schema,
		found,
	}: { schema: Schema; found: { plugin: string; page: PluginPage } } =
		$props();

	type Mounted = { destroy(): void };

	function page(
		target: HTMLElement,
		found: { plugin: string; page: PluginPage },
	) {
		const plugin = schema.plugins.find(p => p.name === found.plugin)!;
		let mounted: Mounted | null = null;
		let gone = false;
		import(/* @vite-ignore */ `/_plugins/${found.plugin}.js`)
			.then(m => {
				if (gone || !m.page) return;
				mounted = m.page(target, {
					schema: $state.snapshot(schema),
					page: $state.snapshot(found.page),
					data: plugin.data,
					host,
				});
			})
			.catch(e => (target.textContent = `${found.plugin}: ${e.message}`));
		return {
			destroy() {
				gone = true;
				mounted?.destroy();
			},
		};
	}
</script>

{#key found.plugin + found.page.handle}
	<div use:page={found}></div>
{/key}
