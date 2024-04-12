<script>
	import Table from '$lib/components/docs/table.svelte';
	import Button from '$lib/registry/button/button.svelte';
</script>

<div class="m-8">
	<h2 class="text-3xl">Buttons</h2>
	<h3 class="text-xl">variants</h3>
	<Table>
		{#each ['primary', 'default', 'outline', 'ghost'] as variant}
			<p>
				variant: {variant}
			</p>
			<Button {variant}>{variant}</Button>
		{/each}
	</Table>
	<hr />
	<Table>
		<p>with icon</p>
		<Button>
			<span>👍</span>
			<span>Button</span>
		</Button>
	</Table>
	<hr />
	<Table>
		<p>disabled</p>
		<Button disabled>Disabled</Button>
	</Table>
</div>
