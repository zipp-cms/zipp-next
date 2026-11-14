<script>
	import DragHandle from 'zipp-ui/DragHandle.svelte';
	import Menu from 'zipp-ui/Menu.svelte';
	import { texts } from './texts.js';

	/**
	 * A field of the form in the builder's grid: its label and a preview
	 * of its input, like the website shows it. A click edits it, the ⋯
	 * menu (or a right click) has the rest.
	 */
	let {
		item,
		typeName,
		selected = false,
		readonly = false,
		sortable,
		index,
		menu,
		onedit,
	} = $props();
	const tx = texts();

	const f = $derived(item.block.fields);
	const off = $derived(item.block.enabled === false);
	const textarea = $derived(item.block.type === 'formTextarea');

	function onclick(e) {
		if (e.target.closest('button, a, input, select')) return;
		onedit();
	}

	function onkeydown(e) {
		if (e.target !== e.currentTarget) return;
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			onedit();
		}
	}
</script>

<div
	class="field-card"
	class:selected
	class:off
	role="button"
	tabindex="0"
	data-menu-area
	{onclick}
	{onkeydown}
>
	<div class="top">
		{#if !readonly}
			<span class="grip">
				<DragHandle {sortable} {index} />
			</span>
		{/if}
		<span class="label truncate">
			{f.label || typeName}{#if f.required}<span class="star">
					*
				</span>{/if}
		</span>
		<span class="type">{typeName}</span>
		{#if off}<span class="badge">{tx.off}</span>{/if}
		{#if !readonly}
			<Menu items={menu} label={tx.fieldsActions} />
		{/if}
	</div>
	<div class="preview" class:textarea>
		<span class="truncate">{f.placeholder ?? ''}</span>
	</div>
	{#if f.instructions}
		<p class="hint truncate">{f.instructions}</p>
	{/if}
</div>

<style lang="scss">
	@use 'zipp-ui/vars.scss' as *;

	.field-card {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
		height: 100%;
		padding: 0.625rem 0.75rem 0.75rem;
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
		cursor: pointer;
		transition:
			border-color 0.12s,
			box-shadow 0.12s;

		@include hover {
			border-color: var(--accent);
		}

		&:focus-visible {
			@include focus-ring;
		}

		&.selected {
			border-color: var(--accent);
			box-shadow: 0 0 0 1px var(--accent);
		}

		&.off {
			opacity: 0.55;
		}
	}

	.top {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		min-height: 1.75rem;
	}

	.grip {
		display: flex;
		margin-left: -0.375rem;
		color: var(--muted);
	}

	.label {
		flex: 1;
		min-width: 0;
		font-weight: 600;
		font-size: var(--fs-base);
	}

	.star {
		margin-left: 0.125rem;
		color: var(--danger);
	}

	.type {
		color: var(--muted);
		font-size: var(--fs-xs);
		white-space: nowrap;
	}

	.preview {
		display: flex;
		align-items: center;
		height: 2rem;
		padding: 0 0.625rem;
		color: var(--muted);
		font-size: var(--fs-md);
		background: var(--hover);
		border: 1px solid var(--line);
		border-radius: var(--radius);

		&.textarea {
			align-items: flex-start;
			height: 4.5rem;
			padding-top: 0.375rem;
		}
	}

	.hint {
		margin: 0;
		color: var(--muted);
		font-size: var(--fs-xs);
	}
</style>
