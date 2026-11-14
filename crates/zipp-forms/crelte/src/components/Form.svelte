<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLFormAttributes } from 'svelte/elements';
	import { untrack } from 'svelte';
	import type { ZippForm } from '../form.svelte.js';
	import { setForm } from '../context';

	/**
	 * A form of zipp-forms: the `<form>` with the honeypot, the
	 * components inside get the form. After sending it shows the message
	 * of the form (`sent` to show something else), why it couldn't send
	 * above the button.
	 */
	let {
		form,
		children,
		sent,
		...rest
	}: {
		form: ZippForm;
		children?: Snippet;
		sent?: Snippet<[string]>;
	} & HTMLFormAttributes = $props();

	setForm(untrack(() => form));

	function onsubmit(e: SubmitEvent) {
		e.preventDefault();
		form.submit();
	}
</script>

{#if form.sent !== null}
	{#if sent}
		{@render sent(form.sent)}
	{:else}
		<p class="zipp-form-sent" role="status">{form.sent}</p>
	{/if}
{:else}
	<form {onsubmit} {...rest}>
		<!-- people don't see it, bots fill it -->
		<input
			class="zipp-form-honeypot"
			type="text"
			name={form.data.honeypot}
			tabindex="-1"
			autocomplete="off"
			aria-hidden="true"
			bind:value={form.values[form.data.honeypot]}
		/>
		{@render children?.()}
		{#if form.failed}
			<p class="zipp-form-failed" role="alert">{form.failed}</p>
		{/if}
	</form>
{/if}

<style>
	.zipp-form-honeypot {
		position: absolute;
		left: -10000px;
		width: 1px;
		height: 1px;
		overflow: hidden;
	}
</style>
