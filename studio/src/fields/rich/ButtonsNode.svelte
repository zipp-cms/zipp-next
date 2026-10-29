<script lang="ts">
	import { untrack } from 'svelte';
	import type { Field, Link, Refs, Schema } from '../../api';
	import LinkField from '../LinkField.svelte';
	import EmbedCard from './EmbedCard.svelte';
	import Icon from '../../lib/Icon.svelte';

	/** A row of buttons in a rich text, each one a link with a label. */
	let {
		attrs,
		schema,
		refs,
		editable,
		update,
		remove,
		insertLine,
	}: {
		attrs: { links: Link[] };
		schema: Schema;
		refs: Refs;
		editable: boolean;
		update: (attrs: Record<string, unknown>) => void;
		remove: () => void;
		insertLine: (where: 'before' | 'after') => void;
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

	// a new row without a target opens to be filled in
	let editing = $state(
		untrack(
			() =>
				!attrs.links.length || attrs.links.some(l => l.value === null),
		),
	);

	/** What a button says: its label, else the title of its entry. */
	function label(link: Link) {
		if (link.label) return link.label;
		if (link.type === 'entry' && typeof link.value === 'number')
			return refs[link.value]?.title ?? 'Button';
		return String(link.value ?? 'Button');
	}

	function target(link: Link) {
		if (link.value === null || link.value === '') return 'no target';
		if (link.type === 'entry' && typeof link.value === 'number')
			return refs[link.value]?.title ?? `entry ${link.value}`;
		return String(link.value);
	}
</script>

<EmbedCard kind="Buttons" {editable} bind:editing onremove={remove} oninsert={insertLine}>
	{#snippet preview()}
		{#each items as { link }, i (i)}
			<span class="button">
				<strong>{label(link)}</strong>
				<span class="target">→ {target(link)}</span>
			</span>
		{:else}
			<span class="none">No buttons yet</span>
		{/each}
	{/snippet}
	<fieldset disabled={!editable}>
		{#each items as item, i (i)}
			<div class="item">
				<LinkField {schema} {field} values={item} {refs} />
				<button
					type="button"
					class="icon"
					title="Remove the button"
					onclick={() => items.splice(i, 1)}
				>
					<Icon name="close" size={15} />
				</button>
			</div>
		{/each}
		<button type="button" class="link add" onclick={add}>
			<Icon name="add" size={14} />
			Add a button
		</button>
	</fieldset>
</EmbedCard>

<style>
	.button {
		display: flex;
		align-items: baseline;
		flex-wrap: wrap;
		gap: 4px 10px;
	}

	.target,
	.none {
		color: var(--muted);
		font-family: var(--mono);
		font-size: 11px;
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
		color: var(--accent);
	}

	fieldset {
		margin: 0;
		padding: 0;
		border: none;
	}
</style>
