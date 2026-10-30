<script lang="ts">
	/** The title, large on top, with the dot of a change and its error. */
	let {
		title = $bindable(),
		changed,
		errors = {},
	}: {
		title: string;
		changed: Set<string>;
		errors?: Record<string, string>;
	} = $props();
</script>

<label class="head">
	<span class="label">
		{#if changed.has('title')}<span class="dot"></span>{/if}Title
		<span class="req">*</span>
	</span>
	<input placeholder="Title" bind:value={title} />
	{#if errors.title}<span class="error">{errors.title}</span>{/if}
</label>

<style>
	.head {
		display: flex;
		flex-direction: column;
		gap: 2px;
		margin-bottom: 18px;
	}

	.label {
		color: var(--muted);
		font-size: var(--fs-xs);
		font-weight: 600;
		letter-spacing: 0.06em;
		text-transform: uppercase;
	}

	.req {
		color: var(--danger);
	}

	input {
		padding: 4px 0 8px;
		font-size: var(--fs-title);
		font-weight: 700;
		line-height: 1.25;
		border: none;
		border-bottom: 1px solid var(--border);
		border-radius: 0;
		background: none;

		&:focus {
			outline: none;
			border-bottom-color: var(--accent);
		}
	}
</style>
