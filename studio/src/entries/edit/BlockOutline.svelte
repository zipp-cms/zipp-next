<script lang="ts">
	import type { Block, Field, Schema } from '../../core/schema';
	import {
		blockMissing,
		blockSummary,
		showBlock,
	} from '../../fields/blocks.svelte';
	import { componentName, componentOf } from '../../core/schema';
	import type { Refs } from '../api';

	/** The blocks of a field as a list; a click opens one and goes there. */
	let {
		schema,
		field,
		blocks,
		refs,
	}: { schema: Schema; field: Field; blocks: Block[]; refs: Refs } = $props();
</script>

<nav class="card outline" aria-label="Outline of {field.label}">
	<h3>Outline · {field.label}</h3>
	<ol class="plain">
		{#each blocks as b, i (i)}
			{@const comp = componentOf(schema, b.type)}
			{@const missing = blockMissing(comp, b.fields)}
			<li>
				<button
					type="button"
					class="block plain"
					onclick={() => showBlock(field.handle, i)}
				>
					<span class="n">{String(i + 1).padStart(2, '0')}</span>
					<span class="name">{componentName(schema, b.type)}</span>
					<span class="note truncate" class:missing>
						{missing
							? 'Missing'
							: blockSummary(comp, b.fields, refs)}
					</span>
				</button>
			</li>
		{/each}
	</ol>
</nav>

<style lang="scss">
	.outline {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		padding: 0.75rem;
	}

	h3 {
		margin: 0;
		font-size: var(--fs-sm);
		font-weight: 600;
	}

	.block {
		display: grid;
		grid-template-columns: 1.375rem auto minmax(0, 1fr);
		align-items: baseline;
		gap: 0.375rem;
		width: 100%;
		padding: 0.3125rem 0.25rem;
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

	.missing {
		color: var(--danger);
	}
</style>
