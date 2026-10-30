<script lang="ts">
	import type { FieldOf, Values } from '../core/schema';
	import DatePicker from 'zipp-ui/DatePicker.svelte';

	/** A date, with `time` a date and time: zipp-ui's picker. */
	let {
		field,
		values,
	}: {
		field: FieldOf<'date'>;
		values: Values;
	} = $props();
</script>

<DatePicker
	value={(values[field.handle] as string | null) ?? ''}
	time={field.time}
	label={field.label}
	onchange={v => (values[field.handle] = v || null)}
/>
