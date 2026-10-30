<script lang="ts">
	/** A bar at the top while a page loads, like hostdinghy's. */
	let { loading }: { loading: number } = $props();

	let progress = $state(0);
	let visible = $state(false);
	let before = 0;
	let hide: ReturnType<typeof setTimeout> | undefined;

	$effect(() => {
		if (before < loading) {
			progress = 0.8;
			visible = true;
		} else if (before !== 0 && loading === 0) {
			progress = 1;
			visible = false;
			clearTimeout(hide);
			hide = setTimeout(() => (progress = 0), 500);
		}
		before = loading;
	});
</script>

<div class="loader" class:visible style:--progress={progress}></div>

<style>
	.loader {
		position: fixed;
		top: 0;
		left: 0;
		z-index: var(--z-overlay);
		width: 100%;
		height: 3px;
		transform: translateY(-100%);
		transition: transform 0.5s ease;
	}

	.loader::before {
		content: '';
		display: block;
		height: 100%;
		background: var(--accent);
		transform: scaleX(var(--progress, 0));
		transform-origin: left;
		transition: transform 0.5s ease;
	}

	.visible {
		transform: translateY(0);
	}
</style>
