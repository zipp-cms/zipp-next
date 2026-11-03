<script lang="ts">
	import type { EntryRef } from './api';
	import { choiceState } from './picker';

	/**
	 * The status dot of an entry in a picker or a link: live, published
	 * but out of its dates, not published. Screen readers read its text.
	 */
	let { entry }: { entry: EntryRef } = $props();

	const status = $derived(choiceState(entry));
</script>

<span class="state-dot {status.state}" title={status.label}></span>
<span class="label">{status.label}</span>

<style lang="scss">
	.label {
		@include visually-hidden;
	}
</style>
