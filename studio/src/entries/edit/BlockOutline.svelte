<script lang="ts">
	import type { Block, Field, Schema } from '../../core/schema';
	import { blockSummary, showBlock } from '../../fields/blocks.svelte';
	import { componentName, componentOf } from '../../core/schema';

	/** The blocks of a field as a list; a click opens one and goes there. */
	let {
		schema,
		field,
		blocks,
	}: { schema: Schema; field: Field; blocks: Block[] } = $props();
</script>

<nav class="card outline" aria-label="Outline of {field.label}">
	<h3>Outline · {field.label}</h3>
	<ol class="plain">
		{#each blocks as b, i (i)}
			<li>
				<button
					type="button"
					onclick={() => showBlock(field.handle, i)}
				>
					<span class="n">{String(i + 1).padStart(2, '0')}</span>
					<span class="name">{componentName(schema, b.type)}</span>
					<span class="note truncate">
						{blockSummary(componentOf(schema, b.type), b.fields)}
					</span>
				</button>
			</li>
		{/each}
	</ol>
</nav>

<style>
	.outline {
		display: flex;
		flex-direction: column;
		gap: 8px;
		padding: 12px;
	}

	h3 {
		margin: 0;
		font-size: var(--fs-sm);
		font-weight: 600;
	}

	button {
		display: grid;
		grid-template-columns: 22px auto minmax(0, 1fr);
		align-items: baseline;
		gap: 6px;
		width: 100%;
		padding: 5px 4px;
		border: none;
		background: none;
		text-align: left;
		font-weight: 400;
	}

	.n {
		color: var(--subtle);
		font-family: var(--mono);
		font-size: var(--fs-2xs);
	}

	.name {
		font-size: var(--fs-md);
		font-weight: 500;
	}

	.note {
		font-size: var(--fs-xs);
		text-align: right;
	}
</style>
