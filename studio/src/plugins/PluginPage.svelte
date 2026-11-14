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
	import { mountPlugin } from './module';
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

	function page(
		target: HTMLElement,
		found: { plugin: string; page: PluginPage },
	) {
		const plugin = schema.plugins.find(p => p.name === found.plugin)!;
		const props = {
			schema: $state.snapshot(schema),
			page: $state.snapshot(found.page),
		};
		return mountPlugin(target, plugin, 'page', props);
	}
</script>

{#key found.plugin + found.page.handle}
	<div use:page={found}></div>
{/key}
