<script lang="ts">
	import type { Field, Schema, Values } from '../api';
	import { site } from '../route.svelte';

	/**
	 * The texts of the frontend: the keys come from the code with the text
	 * of the primary site, editors write them per site. An empty text is
	 * the code's, other sites mark it as not translated.
	 */
	let {
		schema,
		field,
		values,
	}: { schema: Schema; field: Field; values: Values } = $props();

	const keys = $derived(field.kind === 'texts' ? field.keys : []);
	const primary = $derived(site.current === schema.sites[0].handle);
	const texts = $derived((values[field.handle] ?? {}) as Values);

	function set(key: string, text: string) {
		values[field.handle] = { ...texts, [key]: text };
	}
</script>

<table>
	<tbody>
		{#each keys as [key, fallback] (key)}
			{@const own = texts[key] ?? ''}
			<tr>
				<td class="key">{key}</td>
				<td>
					<!-- the primary site has the code's text as value -->
					<input
						class:empty={!own && !primary}
						value={primary ? own || fallback : own}
						placeholder={primary
							? fallback
							: `Not translated: ${fallback}`}
						oninput={e => {
							const v = e.currentTarget.value;
							set(key, primary && v === fallback ? '' : v);
						}}
					/>
				</td>
			</tr>
		{/each}
	</tbody>
</table>

<style>
	table {
		width: 100%;
		border-collapse: collapse;
	}

	td {
		padding: 3px 0;
	}

	.key {
		width: 12rem;
		padding-right: 12px;
		font-family: monospace;
		font-size: var(--fs-md);
		color: var(--muted);
	}

	/* not translated: the website shows the code's text */
	input.empty {
		box-shadow: inset 3px 0 var(--changed);
	}
</style>
