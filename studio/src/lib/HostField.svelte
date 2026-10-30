<script lang="ts">
	import type { Field, Values } from '../api';
	import { FIELDS, HOSTED } from '../fields/registry';

	/**
	 * A field of the studio inside a plugin's module, see `host.ts`: the
	 * value comes in and goes out through `onchange`.
	 */
	let {
		field,
		value,
		onchange,
	}: {
		field: Field;
		value: unknown;
		onchange: (v: unknown) => void;
	} = $props();

	let values = $state<Values>({});
	// the value from outside, unless it is the one sent
	let sent: unknown = undefined;
	$effect.pre(() => {
		if (value !== sent) values[field.handle] = value ?? undefined;
	});
	$effect(() => {
		const v = $state.snapshot(values[field.handle]);
		if (JSON.stringify(v) === JSON.stringify(value)) return;
		sent = v;
		onchange(v ?? null);
	});
</script>

{#if HOSTED.includes(field.kind)}
	{@const Input = FIELDS[field.kind]}
	<Input {field} {values} path={field.handle} />
{:else}
	<span class="error">{field.kind} fields can't be mounted by plugins</span>
{/if}
