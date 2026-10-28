<script module lang="ts">
	import { marked } from 'marked';

	// html in the text is dropped, like zipp does for the website
	marked.use({ renderer: { html: () => '' } });
</script>

<script lang="ts">
	import { untrack } from 'svelte';
	import type { Field, Values } from '../api';

	/**
	 * Markdown: shown rendered when there is text, so it reads like docs,
	 * Write to change it.
	 */
	let { field, values, id }: { field: Field; values: Values; id: string } =
		$props();

	let writing = $state(untrack(() => !values[field.handle]));
	const html = $derived(
		marked.parse(values[field.handle] ?? '', { async: false }) as string,
	);
</script>

<div class="markdown">
	<div class="tabs">
		<button class:on={writing} onclick={() => (writing = true)}>
			Write
		</button>
		<button class:on={!writing} onclick={() => (writing = false)}>
			Preview
		</button>
	</div>
	{#if writing}
		<textarea {id} rows="14" bind:value={values[field.handle]}></textarea>
	{:else}
		<div class="preview">{@html html}</div>
	{/if}
</div>

<style lang="scss">
	.tabs {
		display: flex;
		gap: 4px;
		margin-bottom: 6px;

		button {
			padding: 4px 10px;
			border: none;
			background: none;
			color: var(--muted);

			&.on {
				color: var(--text);
				border-bottom: 2px solid var(--dark);
			}
		}
	}

	textarea {
		font-family: monospace;
		font-size: 14px;
	}

	.preview {
		padding: 4px 2px;
		line-height: 1.55;

		:global(> * + *) {
			margin-top: 0.8em;
		}

		:global(h1),
		:global(h2),
		:global(h3) {
			margin: 1.2em 0 0.4em;
		}

		:global(code) {
			padding: 1px 4px;
			background: var(--input);
			font-size: 0.9em;
		}

		:global(table) {
			border-collapse: collapse;
		}

		:global(td),
		:global(th) {
			padding: 4px 10px;
			border: 1px solid var(--line);
			text-align: left;
		}
	}
</style>
