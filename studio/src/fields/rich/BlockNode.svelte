<script lang="ts">
	import { untrack } from 'svelte';
	import type { Refs, Schema, Values } from '../../api';
	import Fields from '../Fields.svelte';
	import Icon from '../../lib/Icon.svelte';

	/** A block inside a rich text, its fields like in a blocks field. */
	let {
		attrs,
		schema,
		refs,
		editable,
		update,
		remove,
	}: {
		attrs: { id: string | null; type: string; fields: Values };
		schema: Schema;
		refs: Refs;
		editable: boolean;
		update: (attrs: Record<string, unknown>) => void;
		remove: () => void;
	} = $props();

	// attrs are reactive proxies, structuredClone can't copy them
	const clone = <T,>(v: T): T => JSON.parse(JSON.stringify(v));

	const comp = $derived(schema.components.find(c => c.handle === attrs.type));
	let fields = $state<Values>(untrack(() => clone(attrs.fields)));
	let last = JSON.stringify(untrack(() => attrs.fields));

	// changes from the text (undo) come in, changes of the fields go out
	$effect(() => {
		const outer = JSON.stringify(attrs.fields);
		if (outer !== last) {
			last = outer;
			fields = clone(attrs.fields);
		}
	});
	$effect(() => {
		const inner = JSON.stringify($state.snapshot(fields));
		if (inner === last) return;
		last = inner;
		update({ ...attrs, fields: $state.snapshot(fields) });
	});
</script>

<div class="block">
	<div class="head">
		<span>{comp?.name ?? attrs.type}</span>
		{#if editable}
			<button class="icon" title="Remove" onclick={remove}>
				<Icon name="delete" />
			</button>
		{/if}
	</div>
	{#if comp}
		<fieldset disabled={!editable}>
			<Fields
				{schema}
				component={comp}
				values={fields}
				errors={{}}
				{refs}
				prefix="rt."
			/>
		</fieldset>
	{/if}
</div>

<style>
	.block {
		margin: 8px 0;
		padding: 10px 12px 14px;
		background: var(--side, #f4f4f5);
		border-left: 3px solid var(--dark);
	}

	.head {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 8px;
		font-size: 13px;
		font-weight: 600;
	}

	fieldset {
		margin: 0;
		padding: 0;
		border: none;
	}
</style>
