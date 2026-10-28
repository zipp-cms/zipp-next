<script lang="ts">
	/** A borderless textarea that grows with its text. */
	let {
		value = $bindable(),
		el = $bindable(),
		id,
		placeholder = '',
		multiline = true,
		heading = false,
		maxlength = null,
		onkeydown,
		oninput,
	}: {
		value: string | undefined;
		el?: HTMLTextAreaElement;
		id?: string;
		placeholder?: string;
		multiline?: boolean;
		heading?: boolean;
		maxlength?: number | null;
		onkeydown?: (e: KeyboardEvent) => void;
		oninput?: (e: Event) => void;
	} = $props();

	function resize() {
		if (!el) return;
		el.style.height = 'auto';
		el.style.height = el.scrollHeight + 'px';
	}

	$effect(() => {
		void value;
		resize();
	});

	function keydown(e: KeyboardEvent) {
		onkeydown?.(e);
		if (!multiline && e.key === 'Enter') e.preventDefault();
	}
</script>

<svelte:window onresize={resize} />

<textarea
	bind:this={el}
	bind:value
	{id}
	{placeholder}
	maxlength={maxlength ?? undefined}
	rows="1"
	class:heading
	onkeydown={keydown}
	{oninput}
></textarea>

<style>
	textarea {
		display: block;
		width: 100%;
		min-height: 0;
		padding: 0.15rem 0;
		border: none;
		outline: none;
		resize: none;
		overflow: hidden;
		background: none;
		line-height: 1.5;
	}

	.heading {
		font-size: 1.4rem;
		font-weight: 600;
		margin-top: 0.6rem;
	}
</style>
