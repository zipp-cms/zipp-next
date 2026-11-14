<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLButtonAttributes } from 'svelte/elements';
	import { getForm } from '../context';

	/**
	 * The button to the page before, with the form's label (or
	 * `children`); nothing on the first page.
	 */
	let { children, ...rest }: { children?: Snippet } & HTMLButtonAttributes =
		$props();

	const form = getForm();
</script>

{#if form.page > 0}
	<button type="button" onclick={() => form.back()} {...rest}>
		{#if children}{@render children()}{:else}{form.data.backLabel}{/if}
	</button>
{/if}
