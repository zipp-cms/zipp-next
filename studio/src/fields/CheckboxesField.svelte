<script lang="ts">
	import type { FieldOf, Values } from '../core/schema';
	import { valueOf } from '../core/schema';

	/** Some of the options, stored in the order of the options. */
	let { field, values }: { field: FieldOf<'checkboxes'>; values: Values } =
		$props();

	const options = $derived(field.options);
	const picked = $derived(valueOf(values, field) ?? []);

	function toggle(value: string, on: boolean) {
		const next = options
			.map(o => o.value)
			.filter(v => (v === value ? on : picked.includes(v)));
		values[field.handle] = next.length ? next : null;
	}
</script>

<div class="options">
	{#each options as o (o.value)}
		<label>
			<input
				type="checkbox"
				checked={picked.includes(o.value)}
				onchange={e => toggle(o.value, e.currentTarget.checked)}
			/>
			{o.label}
		</label>
	{/each}
</div>

<style>
	.options {
		display: flex;
		flex-wrap: wrap;
		gap: 8px 20px;
	}

	label {
		display: flex;
		align-items: center;
		gap: 6px;
	}
</style>
