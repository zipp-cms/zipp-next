<script lang="ts">
	import { untrack } from 'svelte';
	import type { Field, Link, Refs, Schema } from '../../api';
	import LinkField from '../LinkField.svelte';
	import Icon from '../../lib/Icon.svelte';

	/** A row of buttons in a rich text, each one a link with a label. */
	let {
		attrs,
		schema,
		refs,
		editable,
		update,
		remove,
	}: {
		attrs: { links: Link[] };
		schema: Schema;
		refs: Refs;
		editable: boolean;
		update: (attrs: Record<string, unknown>) => void;
		remove: () => void;
	} = $props();

	// attrs are reactive proxies, structuredClone can't copy them
	const clone = <T,>(v: T): T => JSON.parse(JSON.stringify(v));

	// what the link field of a button allows
	const field: Field = {
		handle: 'link',
		label: 'Button',
		required: false,
		indexed: false,
		translatable: false,
		kind: 'link',
		to: ['entry', 'url', 'email', 'phone'],
		labels: true,
		newTab: true,
	};

	let items = $state(
		untrack(() => attrs.links.map(link => ({ link: clone(link) }))),
	);
	let last = JSON.stringify(untrack(() => attrs.links));

	$effect(() => {
		const outer = JSON.stringify(attrs.links);
		if (outer !== last) {
			last = outer;
			items = attrs.links.map(link => ({ link: clone(link) }));
		}
	});
	$effect(() => {
		const links = $state.snapshot(items).map(i => i.link);
		const inner = JSON.stringify(links);
		if (inner === last) return;
		last = inner;
		update({ links });
	});

	function add() {
		items.push({ link: { type: 'entry', value: null } });
	}
</script>

<div class="buttons">
	<div class="head">
		<span>Buttons</span>
		{#if editable}
			<button class="icon" title="Remove the row" onclick={remove}>
				<Icon name="delete" />
			</button>
		{/if}
	</div>
	<fieldset disabled={!editable}>
		{#each items as item, i (i)}
			<div class="item">
				<LinkField {schema} {field} values={item} {refs} />
				<button
					class="icon"
					title="Remove the button"
					onclick={() => items.splice(i, 1)}
				>
					<Icon name="delete" />
				</button>
			</div>
		{/each}
		<button class="add" onclick={add}>
			<Icon name="add" />
			Add a button
		</button>
	</fieldset>
</div>

<style>
	.buttons {
		margin: 8px 0;
		padding: 10px 12px 12px;
		background: #f4f4f5;
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

	.item {
		display: flex;
		gap: 8px;
		align-items: flex-start;
		padding-bottom: 10px;
		margin-bottom: 10px;
		border-bottom: 1px solid var(--line);

		> :global(:first-child) {
			flex: 1;
		}
	}

	.add {
		border: none;
		background: none;
		padding: 4px 0;
	}

	fieldset {
		margin: 0;
		padding: 0;
		border: none;
	}
</style>
