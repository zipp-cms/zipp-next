<script lang="ts">
	import type { SitePreview } from './preview.svelte';

	/** The website with every draft, next to the chat. */
	let { preview }: { preview: SitePreview } = $props();

	let frame: HTMLIFrameElement;
	$effect(() => preview.start(frame));
</script>

<section class="site">
	<div class="bar">
		<span class="url truncate">{decodeURI(preview.view)}</span>
		<button class="link" onclick={() => preview.refresh()}>Reload</button>
		{#if preview.token}
			<a
				href={preview.withToken(preview.view)}
				target="_blank"
				rel="noreferrer"
			>
				Open
			</a>
		{/if}
	</div>
	<!-- its src is set by the preview, see `load` -->
	<iframe bind:this={frame} title="Website"></iframe>
</section>

<style lang="scss">
	.site {
		display: flex;
		flex-direction: column;
		min-width: 0;
	}

	.bar {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 8px 16px;
		background: var(--surface);
		font-size: var(--fs-base);

		.url {
			flex: 1;
			overflow: hidden;
			color: var(--muted);
			white-space: nowrap;
			text-overflow: ellipsis;
		}
	}

	iframe {
		flex: 1;
		width: 100%;
		border: none;
		background: var(--surface);
	}
</style>
