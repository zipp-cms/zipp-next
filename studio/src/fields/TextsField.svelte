<script lang="ts">
	import type { FieldOf, Schema, Values } from '../core/schema';
	import { valueOf } from '../core/schema';
	import { editedSite } from '../core/editing';

	/**
	 * The texts of the frontend: the keys come from the code with the text
	 * of the primary site, editors write them per site. An empty text is
	 * the code's, other sites mark it as not translated.
	 */
	let {
		schema,
		field,
		values,
	}: { schema: Schema; field: FieldOf<'texts'>; values: Values } = $props();

	const keys = $derived(field.keys);
	const edited = editedSite();
	const primary = $derived(edited === schema.sites[0].handle);
	const texts = $derived(valueOf(values, field) ?? {});
	// the key whose text was just emptied: it stays empty while typing,
	// not the code's text again under the cursor
	let cleared = $state<string | null>(null);

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
						value={primary
							? own || (cleared === key ? '' : fallback)
							: own}
						placeholder={primary
							? fallback
							: `Not translated: ${fallback}`}
						oninput={e => {
							const v = e.currentTarget.value;
							cleared = v === '' ? key : null;
							set(key, primary && v === fallback ? '' : v);
						}}
						onblur={() => (cleared = null)}
					/>
				</td>
			</tr>
		{/each}
	</tbody>
</table>

<style lang="scss">
	table {
		width: 100%;
		border-collapse: collapse;
	}

	td {
		padding: 3px 0;
	}

	.key {
		width: 10.5rem;
		padding-right: 0.75rem;
		font-family: monospace;
		font-size: var(--fs-md);
		color: var(--muted);
	}

	/* not translated: the website shows the code's text */
	input.empty {
		box-shadow: inset 3px 0 var(--changed);
	}
</style>
