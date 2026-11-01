<script lang="ts">
	import type { FieldOf, Values } from '../core/schema';
	import { valueOf } from '../core/schema';
	import CheckGroup from 'zipp-ui/CheckGroup.svelte';

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

<CheckGroup {options} {picked} ontoggle={toggle} label={field.label} />
