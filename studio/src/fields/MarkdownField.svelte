<script lang="ts">
	import { markdown } from '../lib/markdown';
	import { untrack } from 'svelte';
	import type { Field, Values } from '../api';
	import Segmented from '../lib/Segmented.svelte';

	/**
	 * Markdown: shown rendered when there is text, so it reads like docs,
	 * Write to change it.
	 */
	let { field, values, id }: { field: Field; values: Values; id: string } =
		$props();

	let writing = $state(untrack(() => !values[field.handle]));
	const html = $derived(markdown(values[field.handle] ?? ''));
</script>

<div class="markdown">
	<div class="tabs">
		<Segmented
			tabs
			options={[
				{ value: 'write', label: 'Write' },
				{ value: 'preview', label: 'Preview' },
			]}
			value={writing ? 'write' : 'preview'}
			onchange={v => (writing = v === 'write')}
		/>
	</div>
	{#if writing}
		<textarea {id} rows="14" bind:value={values[field.handle]}></textarea>
	{:else}
		<div class="markdown-body">{@html html}</div>
	{/if}
</div>

<style>
	.tabs {
		margin-bottom: 6px;
	}

	textarea {
		font-family: var(--mono);
		font-size: var(--fs-md);
	}
</style>
