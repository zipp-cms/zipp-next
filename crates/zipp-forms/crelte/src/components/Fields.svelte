<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { FieldData } from '../types';
	import { getForm } from '../context';
	import Field from './Field.svelte';

	/** The fields of the form in their order, each a `Field` or `field`. */
	let { field }: { field?: Snippet<[FieldData]> } = $props();

	const form = getForm();
</script>

{#each form.fields as f (f.handle)}
	{#if field}
		{@render field(f)}
	{:else}
		<Field field={f} />
	{/if}
{/each}
