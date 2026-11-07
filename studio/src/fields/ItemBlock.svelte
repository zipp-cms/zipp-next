<script lang="ts">
	import type { Block } from '../core/schema';
	import { componentOf } from '../core/schema';
	import type { Source } from './itemBlocks';
	import BlockFields from './BlockFields.svelte';

	/**
	 * The fields of the block `id` in the items of a custom type, what
	 * `blocks.mount` of its module shows; nothing once it left them.
	 */
	let { src, id }: { src: Source; id: string } = $props();
	const { schema, field, values, errors, refs, path, changed } =
		$derived(src);

	const items = $derived(
		(values[field.handle] as { block?: Block }[] | null) ?? [],
	);
	const at = $derived(items.findIndex(i => i?.block?.id === id));
	const block = $derived(items[at]?.block);
	const component = $derived(block && componentOf(schema, block.type));
</script>

{#if block && component}
	<BlockFields
		{schema}
		{component}
		values={block.fields}
		{errors}
		{refs}
		prefix="{path}.{at}.block."
		{changed}
		{id}
	/>
{/if}
