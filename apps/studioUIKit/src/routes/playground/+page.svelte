<script>
	import Button from '$lib/registry/button/button.svelte';
</script>

<div class="m-8">
	<h2 class="text-3xl">Buttons</h2>
	<div class="grid grid-cols-[minmax(0,200px)_auto] gap-4">
		{#each ['primary', 'default', 'outline', 'ghost'] as variant}
			<p>
				variant: {variant}
			</p>
			<Button {variant}>{variant}</Button>
		{/each}
	</div>
</div>
