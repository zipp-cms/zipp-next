<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLButtonAttributes } from 'svelte/elements';
	import { getForm } from '../context';

	/**
	 * The button that goes to the next page or sends on the last, with the
	 * form's labels (or `children`).
	 */
	let { children, ...rest }: { children?: Snippet } & HTMLButtonAttributes =
		$props();

	const form = getForm();
</script>

<button type="submit" disabled={form.sending} {...rest}>
	{#if children}{@render children()}{:else if form.isLast}{form.data
			.submitLabel}{:else}{form.data.nextLabel}{/if}
</button>
