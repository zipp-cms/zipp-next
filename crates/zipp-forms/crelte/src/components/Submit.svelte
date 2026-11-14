<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLButtonAttributes } from 'svelte/elements';
	import { getForm } from '../context';

	/** The button that sends, with the form's label (or `children`). */
	let {
		children,
		...rest
	}: { children?: Snippet } & HTMLButtonAttributes = $props();

	const form = getForm();
</script>

<button type="submit" disabled={form.sending} {...rest}>
	{#if children}{@render children()}{:else}{form.data.submitLabel}{/if}
</button>
