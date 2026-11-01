<script lang="ts">
	import DatePicker from 'zipp-ui/DatePicker.svelte';

	/**
	 * A date and time in the editor's time zone, as milliseconds; empty is
	 * none (a post date then comes with publishing, no expiry date: never).
	 * zipp-ui's picker with its time, `placeholder` says what none means.
	 */
	let {
		value,
		onchange,
		label,
		placeholder,
	}: {
		value: number | null;
		onchange: (v: number | null) => void;
		label: string;
		/** what none means for this date */
		placeholder: string;
	} = $props();

	/** `2026-11-14T20:00` in the browser's time zone, what the picker takes. */
	function local(ms: number): string {
		const d = new Date(ms);
		return new Date(ms - d.getTimezoneOffset() * 60_000)
			.toISOString()
			.slice(0, 16);
	}
</script>

<DatePicker
	time
	{label}
	{placeholder}
	value={value === null ? '' : local(value)}
	onchange={v => onchange(v ? new Date(v).getTime() : null)}
/>
