<script lang="ts">
	import type { Snippet } from 'svelte';
	import Icon from '../../lib/Icon.svelte';

	/**
	 * Something in a rich text that isn't text, like the design: a card
	 * with what it shows, the form opens with edit.
	 */
	let {
		kind,
		editable,
		editing = $bindable(false),
		onremove,
		preview,
		children,
	}: {
		/** what it is, like Buttons or the name of a block */
		kind: string;
		editable: boolean;
		editing?: boolean;
		onremove: () => void;
		preview: Snippet;
		children: Snippet;
	} = $props();
</script>

<!-- not part of the text: the editor doesn't place the cursor in it -->
<div class="card-embed" class:editing contenteditable="false">
	<div class="head">
		<div class="preview">
			<span class="kind">{kind}</span>
			{@render preview()}
		</div>
		<button
			type="button"
			class="icon"
			title={editing ? 'Close' : 'Edit'}
			aria-expanded={editing}
			onclick={() => (editing = !editing)}
		>
			<Icon name={editing ? 'chevron-down' : 'edit'} size={15} />
		</button>
		{#if editable}
			<button
				type="button"
				class="icon remove"
				title="Remove"
				onclick={onremove}
			>
				<Icon name="delete" size={15} />
			</button>
		{/if}
	</div>
	{#if editing}
		<div class="form">{@render children()}</div>
	{/if}
</div>

<style lang="scss">
	.card-embed {
		margin: 8px 0;
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: 8px;
		font-size: 13px;
		line-height: 1.4;
		white-space: normal;
	}

	.editing {
		border-color: var(--border-strong);
	}

	.head {
		display: flex;
		align-items: flex-start;
		gap: 4px;
		padding: 8px 8px 8px 12px;
	}

	.preview {
		display: flex;
		flex-direction: column;
		gap: 3px;
		flex: 1;
		min-width: 0;
	}

	.kind {
		color: var(--accent-hover);
		font-size: 10px;
		font-weight: 600;
		letter-spacing: 0.06em;
		text-transform: uppercase;
	}

	.remove {
		color: var(--danger);

		&:hover:not(:disabled) {
			color: var(--danger);
		}
	}

	.form {
		padding: 12px;
		background: var(--surface-2);
		border-top: 1px solid var(--line);
		border-radius: 0 0 8px 8px;
	}
</style>
