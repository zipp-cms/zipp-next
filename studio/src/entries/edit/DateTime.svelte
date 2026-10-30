<script lang="ts">
	/**
	 * A date and time in the editor's time zone, as milliseconds; empty is
	 * none (a post date then comes with publishing, no expiry date: never).
	 */
	let {
		value,
		onchange,
		label,
	}: {
		value: number | null;
		onchange: (v: number | null) => void;
		label: string;
	} = $props();

	/** `2026-11-14T20:00` of the browser's time zone, what the input takes. */
	function local(ms: number): string {
		const d = new Date(ms);
		return new Date(ms - d.getTimezoneOffset() * 60_000)
			.toISOString()
			.slice(0, 16);
	}
</script>

<input
	type="datetime-local"
	aria-label={label}
	value={value === null ? '' : local(value)}
	onchange={e => {
		const v = e.currentTarget.value;
		onchange(v ? new Date(v).getTime() : null);
	}}
/>
