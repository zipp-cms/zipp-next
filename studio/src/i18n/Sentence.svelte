<script lang="ts">
	import type { Snippet } from 'svelte';
	import { tParts, type Key, type Part } from './index.svelte';

	/**
	 * A text with markup around its params: code, or what `param`
	 * renders for one.
	 */
	let {
		key,
		params,
		param,
	}: {
		key: Key;
		params: Record<string, string | number>;
		param?: Snippet<[Part]>;
	} = $props();
</script>

<!-- the whitespace inside the blocks goes, only the texts' own stays -->
{#each tParts(key, params) as p, i (i)}
	{#if !p.name}
		{p.text}
	{:else if param}
		{@render param(p)}
	{:else}
		<code>{p.text}</code>
	{/if}
{/each}
