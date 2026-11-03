<script lang="ts">
	import { markdown } from '../core/markdown';
	import { untrack } from 'svelte';
	import type { FieldOf, Values } from '../core/schema';
	import { valueOf } from '../core/schema';
	import Segmented from 'zipp-ui/Segmented.svelte';
	import { t } from '../i18n/index.svelte';
	import { labelId } from './labels';

	/**
	 * Markdown: shown rendered when there is text, so it reads like docs,
	 * Write to change it.
	 */
	let {
		field,
		values,
		path: id,
	}: { field: FieldOf<'markdown'>; values: Values; path: string } = $props();

	let writing = $state(untrack(() => !values[field.handle]));
	const html = $derived(markdown(valueOf(values, field) ?? ''));
</script>

<div class="markdown">
	<div class="tabs">
		<Segmented
			tabs
			options={[
				{ value: 'write', label: t('fields.write') },
				{ value: 'preview', label: t('common.preview') },
			]}
			value={writing ? 'write' : 'preview'}
			onchange={v => (writing = v === 'write')}
		/>
	</div>
	{#if writing}
		<!-- the id for a plugin's FieldRow, the label's id in the studio -->
		<textarea
			{id}
			aria-labelledby={labelId(id)}
			rows="14"
			bind:value={values[field.handle]}
		></textarea>
	{:else}
		<div class="markdown-body">{@html html}</div>
	{/if}
</div>

<style lang="scss">
	.tabs {
		margin-bottom: 0.375rem;
	}

	textarea {
		font-family: var(--mono);
		font-size: var(--fs-md);
	}
</style>
