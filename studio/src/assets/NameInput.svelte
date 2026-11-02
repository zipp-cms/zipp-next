<script lang="ts">
	import { untrack } from 'svelte';

	/**
	 * An input that renames in place: Enter or leaving it saves, Escape
	 * cancels, and so does an empty or unchanged name.
	 */
	let {
		value,
		label,
		placeholder,
		selectStem = false,
		small = false,
		onsave,
		oncancel,
	}: {
		value: string;
		label: string;
		placeholder?: string;
		/** select the name without its extension, like a file manager */
		selectStem?: boolean;
		/** in a card's caption */
		small?: boolean;
		/** whether it worked, the input stays open when not */
		onsave: (name: string) => Promise<boolean>;
		oncancel: () => void;
	} = $props();

	let name = $state(untrack(() => value));
	// the blur after Enter or Escape, while it goes away, saves nothing
	let done = false;
	let saving = false;

	async function save() {
		if (done || saving) return;
		const n = name.trim();
		if (!n || n === value) return cancel();
		saving = true;
		done = await onsave(n);
		saving = false;
	}

	function cancel() {
		done = true;
		oncancel();
	}

	function onkeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			e.preventDefault();
			save();
		} else if (e.key === 'Escape') {
			e.preventDefault();
			cancel();
		}
	}

	function focus(el: HTMLInputElement) {
		el.focus();
		const dot = selectStem ? el.value.lastIndexOf('.') : -1;
		el.setSelectionRange(0, dot > 0 ? dot : el.value.length);
	}
</script>

<input
	class:small
	bind:value={name}
	{placeholder}
	aria-label={label}
	use:focus
	{onkeydown}
	onblur={save}
/>

<style lang="scss">
	input {
		flex: 1;
		min-width: 0;
		padding: 0.25rem 0.375rem;

		&.small {
			padding: 0.125rem 0.375rem;
			font-size: var(--fs-sm);
		}
	}
</style>
