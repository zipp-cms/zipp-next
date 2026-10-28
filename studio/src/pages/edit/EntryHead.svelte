<script lang="ts">
	/** The title, with the dot of a change and its error. */
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
	<span>
		{#if changed.has('title')}<span class="dot"></span>{/if}Title
	</span>
	<input placeholder="Title" bind:value={title} />
	{#if errors.title}<span class="error">{errors.title}</span>{/if}
</label>

<style>
	.head {
		display: flex;
		flex-direction: column;
		gap: 10px;
		margin-bottom: 30px;
	}
</style>
