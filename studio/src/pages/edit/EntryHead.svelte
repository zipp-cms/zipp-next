<script lang="ts">
	/** Title and slug, with the dot of a change and errors. */
	let {
		title = $bindable(),
		slug = $bindable(),
		single,
		changed,
		uri = null,
		errors = {},
	}: {
		title: string;
		slug: string;
		single: boolean;
		changed: Set<string>;
		uri?: string | null;
		errors?: Record<string, string>;
	} = $props();
</script>

<div class="head">
	<label>
		<span>
			{#if changed.has('title')}<span class="dot"></span>{/if}Title
		</span>
		<input placeholder="Title" bind:value={title} />
		{#if errors.title}<span class="error">{errors.title}</span>{/if}
	</label>
	{#if !single}
		<label>
			<span>
				{#if changed.has('slug')}<span class="dot"></span>{/if}Slug
			</span>
			<input placeholder="from the title" bind:value={slug} />
			{#if uri !== null}<span class="uri">/{uri}</span>{/if}
			{#if errors.slug}<span class="error">{errors.slug}</span>{/if}
		</label>
	{/if}
</div>

<style>
	.head {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 24px;
		margin-bottom: 30px;
	}

	label {
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	.uri {
		margin-top: -4px;
		color: var(--muted);
		font-size: 14px;
	}

	@media (max-width: 800px) {
		.head {
			grid-template-columns: 1fr;
		}
	}
</style>
